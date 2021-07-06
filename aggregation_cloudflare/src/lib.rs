extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use tokio;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[tokio::main]
#[wasm_bindgen]
pub async fn greet() -> String {
    //let host = "http://miguel.gouveia.me";
    //let a = host + "/a";
    //let b = host + "/b";
    let a = "http://miguel.gouveia.me/a";
    let b = "http://miguel.gouveia.me/b";


    let fetch_a = reqwest::get(a).await.unwrap().text().await.unwrap();
    //let fetch_b = reqwest::get(b).await.unwrap().text().await.unwrap();
    //let aggrega = format!("{}{}",fetch_a,fetch_b);

    //"Hello, wasm-worker! \n Miguel \n aggrega".to_string()
    fetch_a
}
