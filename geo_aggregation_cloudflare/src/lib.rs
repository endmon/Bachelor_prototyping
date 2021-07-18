extern crate js_sys;

use cfg_if::cfg_if;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use serde_json::Value;
use web_sys::{Request, RequestInit, Response, Blob, FileReaderSync};
use log::{info, Level};
use js_sys::{global, Function, Object, Promise, Reflect, JsString};
use urlparse::urlparse;
use urlparse::GetQuery;  // Trait
use worker_kv::*;
use std::fmt::Binary;
use wasm_bindgen::__rt::std::convert::TryInto;

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

async fn fetch_rust_wasm_binary(url:&str) -> Result<String, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");

    let request = Request::new_with_str_and_init(url, &opts)?;

    let global = worker_global_scope().unwrap();
    let responce = JsFuture::from(global.fetch_with_request(&request)).await?;
    let responce_value: Response = responce.dyn_into().unwrap();
    let responce_json = JsFuture::from(responce_value.blob()?).await?;
    //let responce_string = responce_json.as_string().unwrap();
    //let v:Vec<u8> = responce_json.try_into().unwrap();
    let responce_blob = Blob::new_with_buffer_source_sequence(&responce_json);
    let mut reps = "".to_string();
    if responce_blob.is_ok() {
        reps = "t".to_string();
    } else {
        reps = "nope".to_string();
    }

    //let responce_string = responce_json.as_string().unwrap();

    Ok(reps.to_string())
}

#[wasm_bindgen]
pub async fn test(request:Request) -> Result<String, JsValue> {

    //let url = urlparse(request.url()); //retourne un string de type https://example.com/example

    let mut result = "".to_string();
    //let mut v: Vec<&str> = url.path.rsplit("/").collect();

    //v.pop();
    //let mut layers = v.pop().unwrap();

    //let mut vector_layers:Vec<&str> = layers.rsplit("+").collect();

    //result = fetch_rust_wasm_binary("http://miguel-gouveia.me/a").await.unwrap();

    //let mut file:Binary;

    /*for layer in vector_layers {
        file += got fetch binarie
    }*/


    Ok(result)
}
