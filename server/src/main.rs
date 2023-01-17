use hyper::Method;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

use jsonrpsee::{
    core::server::host_filtering::AllowHosts,
    server::{RpcModule, ServerBuilder},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Start up a JSON-RPC server that allows cross origin requests.
    let server_addr = run_server().await?;
    futures::future::pending().await
}

async fn run_server() -> anyhow::Result<SocketAddr> {
    // Add a CORS middleware for handling HTTP requests.
    // This middleware does affect the response, including appropriate
    // headers to satisfy CORS. Because any origins are allowed, the
    // "Access-Control-Allow-Origin: *" header is appended to the response.
    let cors = CorsLayer::new()
        // Allow `POST` when accessing the resource
        .allow_methods([Method::POST])
        // Allow requests from any origin
        .allow_origin(Any)
        .allow_headers([hyper::header::CONTENT_TYPE]);
    let middleware = tower::ServiceBuilder::new().layer(cors);

    // The RPC exposes the access control for filtering and the middleware for
    // modifying requests / responses. These features are independent of one another
    // and can also be used separately.
    // In this example, we use both features.
    let server = ServerBuilder::default()
        .set_host_filtering(AllowHosts::Any)
        .set_middleware(middleware)
        .build("127.0.0.1:9999".parse::<SocketAddr>()?)
        .await?;

    let mut module = RpcModule::new(());
    module.register_method("say_hello", |_, _| {
        println!("say_hello method called!");
        Ok("Hello there!!")
    })?;

    let addr = server.local_addr()?;
    let handle = server.start(module)?;

    // In this example we don't care about doing shutdown so let's it run forever.
    // You may use the `ServerHandle` to shut it down or manage it yourself.
    tokio::spawn(handle.stopped());

    Ok(addr)
}
