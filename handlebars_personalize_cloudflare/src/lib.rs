extern crate js_sys;

use cfg_if::cfg_if;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Number, Map};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use serde_json::Value;
use web_sys::{Request, RequestInit, Response};
use log::{info, Level};
use js_sys::{global, Function, Object, Promise, Reflect, JsString};
use urlparse::urlparse;
use urlparse::GetQuery;  // Trait
use worker_kv::*;
use std::collections::BTreeMap;
use handlebars::Handlebars;

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
pub async fn main(request:Request) -> String {

    // create the handlebars registry
    let mut handlebars = Handlebars::new();

    let url = urlparse(request.url()); //retourne un string de type https://example.com/example

    let mut json = "".to_string();
    let mut template = "".to_string();

    let v:Vec<&str> = url.path.rsplit(".").collect();

    if url.query.is_some() {
        let query = url.get_parsed_query().unwrap();
        //traité query
    }

    //[0] == html  [1] == template/index   [2] == content/post
    match v[2] {
        "/content/post" => json = fetch_rust_wasm(
            "http://miguel-gouveia.me/content/posts.json").await.unwrap(),

        _ => json = "_GET".to_string(),
    }

    match v[1] {
        "/template/index" => template = fetch_rust_wasm(
            "http://miguel-gouveia.me/template/index.hbs").await.unwrap(),

        _ => template = "_GET".to_string(),
    }

    /*
    let template = fetch_rust_wasm("http://miguel-gouveia.me/template/index.hbs").await.unwrap();
    let json = fetch_rust_wasm("http://miguel-gouveia.me/content/posts.json").await.unwrap();*/

    let json_obj: Value = serde_json::from_str(&json).unwrap();


    handlebars.register_template_string("hello", template); //bind le template source2 avec le nom "hello"
    handlebars.render("hello", &json_obj).unwrap() //render le template nommer "hello" avec l'objet test1

}
