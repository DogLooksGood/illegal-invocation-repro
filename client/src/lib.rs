use wasm_bindgen::prelude::*;
use web_sys::console;
use jsonrpsee::{wasm_client::WasmClientBuilder, core::client::ClientT, rpc_params};
use serde::{Deserialize, Serialize};


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let client = WasmClientBuilder::default().build("ws://localhost:9999").await.unwrap();
    let _r: String = client.request("say_hello", rpc_params![]).await.unwrap();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
