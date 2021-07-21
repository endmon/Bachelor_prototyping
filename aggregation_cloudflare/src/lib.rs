extern crate js_sys;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use log::{info, Level};
use js_sys::JsString;

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

    let link1 = "http://miguel-gouveia.me/a";
    let link2 = "http://miguel-gouveia.me/b";
    let link3 = "http://miguel-gouveia.me/c";

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    init_log();

    let mut opts = RequestInit::new();
    opts.method("GET");

    let request_1 = Request::new_with_str_and_init(
        link1,
        &opts
    )?;

    let request_2 = Request::new_with_str_and_init(
        link2,
        &opts
    )?;

    let request_3 = Request::new_with_str_and_init(
        link3,
        &opts
    )?;


    let global = worker_global_scope().unwrap();
    let resp_value_1 = JsFuture::from(global.fetch_with_request(&request_1)).await?;
    let resp_value_2 = JsFuture::from(global.fetch_with_request(&request_2)).await?;
    let resp_value_3 = JsFuture::from(global.fetch_with_request(&request_3)).await?;


    assert!(resp_value_1.is_instance_of::<Response>());
    let resp_1: Response = resp_value_1.dyn_into().unwrap();
    let resp_2: Response = resp_value_2.dyn_into().unwrap();
    let resp_3: Response = resp_value_3.dyn_into().unwrap();


    let text_1 = JsFuture::from(resp_1.text()?).await?;
    let text_2 = JsFuture::from(resp_2.text()?).await?;
    let text_3 = JsFuture::from(resp_3.text()?).await?;

    let mut text_123 = format!("{}{}{}", text_1.as_string().unwrap(), text_2.as_string().unwrap(), text_3.as_string().unwrap());

    Ok(text_123)

}
