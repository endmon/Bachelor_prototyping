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
use js_sys::{global, Function, Object, Promise, Reflect, JsString, Array};
use urlparse::urlparse;
use urlparse::GetQuery;  // Trait
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

pub async fn fetch_wasm_binary(url:&str) -> Vec<u8> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request_a = Request::new_with_str_and_init(
        url,
        &opts
    ).unwrap();
    let global = worker_global_scope().unwrap();
    let resp_value_a = JsFuture::from(global.fetch_with_request(&request_a)).await.unwrap();
    let resp_a: Response = resp_value_a.dyn_into().unwrap();
    let json_a = JsFuture::from(resp_a.array_buffer().unwrap()).await.unwrap();
    let binary:js_sys::Uint8Array = js_sys::Uint8Array::new(&json_a);
    let mut body = vec![0; binary.length() as usize];
    binary.copy_to(&mut body[..]);
    body
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

/*async fn fetch_rust_wasm_binary(url:&str) -> Result<String, JsValue> {
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
}*/

#[wasm_bindgen]
pub async fn test(request:Request) -> Array {

    let url = urlparse(request.url()); //retourne un string de type https://example.com/example


    let mut v: Vec<&str> = url.path.rsplit("/").collect();

    v.pop(); //pop root /
    v.pop(); //pop /geo
    let mut layers = v.pop().unwrap();

    let mut vector_layers:Vec<&str> = layers.rsplit("+").collect();
    let z = v.pop().unwrap();
    let y = v.pop().unwrap();
    let x = v.pop().unwrap(); //{x.pbf}


    //let mut file:Binary;
    let mut file:Vec<u8> = Vec::new();
    let mut layer_string:String = "".to_string();

    for layer in vector_layers {
        let mut pbf = fetch_wasm_binary(format!("http://miguel-gouveia.me/tiles/{}/{}/{}/{}", layer, z, y, x).as_str()).await;
        file.append(&mut pbf);
    }

    file.into_iter().map(JsValue::from).collect()
}
