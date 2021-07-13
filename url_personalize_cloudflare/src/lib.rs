extern crate js_sys;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use log::{info, Level};
use js_sys::JsString;
use urlparse::urlparse;
use urlparse::GetQuery;  // Trait

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

async fn fetch_rust_wasm(url:&str) -> Result<String, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");

    let request = Request::new_with_str_and_init(url, &opts)?;

    let global = worker_global_scope().unwrap();
    let responce = JsFuture::from(global.fetch_with_request(&request)).await?;
    let responce_value: Response = responce.dyn_into().unwrap();
    let responce_json = JsFuture::from(responce_value.text()?).await?;
    let responce_string = responce_json.as_string().unwrap();

    Ok(responce_string)
}

#[wasm_bindgen]
pub async fn test(request:Request) -> Result<String, JsValue> {
    let met = request.method();
    let url = urlparse(request.url()); //retourne un string de type https://example.com/example

    let mut result = "".to_string();

    if url.query.is_some() {
        let query = url.get_parsed_query().unwrap();
        //traité query
    }

    /*if url.path == "/wiki" {
        let fetch = fetch_rust_wasm(
            "https://www.mediawiki.org/w/api.php?action=help").await.unwrap();

        result = fetch;
    }*/

    match url.path.as_str() {
        "/wiki" => result = fetch_rust_wasm(
            "https://www.mediawiki.org/w/api.php?action=help").await.unwrap(),

        _ => result = "".to_string(),
    }


    Ok(result)
}