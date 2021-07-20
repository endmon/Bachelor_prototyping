extern crate js_sys;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use log::{info, Level};
use js_sys::{JsString, ArrayBuffer, Array};

mod utils;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

cfg_if! {
    if #[cfg(feature = "console_log")] {
        fn init_log() {
            console_log::init_with_level(Level::Trace).expect("error initializing log");
        }
    } else {
        fn init_log() {}
    }
}

pub fn worker_global_scope() -> Option<web_sys::ServiceWorkerGlobalScope> {
    js_sys::global().dyn_into::<web_sys::ServiceWorkerGlobalScope>().ok()
}

#[wasm_bindgen]
pub async fn test(request:Request) -> Result<String, JsValue> {

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    init_log();

    let mut opts = RequestInit::new();
    opts.method("GET");

    let request_a = Request::new_with_str_and_init(
        "http://miguel-gouveia.me/a",
        &opts
    )?;

    let request_b = Request::new_with_str_and_init(
        "http://miguel-gouveia.me/b",
        &opts
    )?;

    let request_c = Request::new_with_str_and_init(
        "http://miguel-gouveia.me/c",
        &opts
    )?;


    let global = worker_global_scope().unwrap();
    let resp_value_a = JsFuture::from(global.fetch_with_request(&request_a)).await?;
    let resp_value_b = JsFuture::from(global.fetch_with_request(&request_b)).await?;
    let resp_value_c = JsFuture::from(global.fetch_with_request(&request_c)).await?;


    assert!(resp_value_a.is_instance_of::<Response>());
    let resp_a: Response = resp_value_a.dyn_into().unwrap();
    let resp_b: Response = resp_value_b.dyn_into().unwrap();
    let resp_c: Response = resp_value_c.dyn_into().unwrap();


    let json_a = JsFuture::from(resp_a.text()?).await?;
    let json_b = JsFuture::from(resp_b.text()?).await?;
    let json_c = JsFuture::from(resp_c.text()?).await?;

    let mut json_abc = format!("{}{}{}", json_a.as_string().unwrap(), json_b.as_string().unwrap(), json_c.as_string().unwrap());

    Ok(json_abc)

}
