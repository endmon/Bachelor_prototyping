extern crate js_sys;

use cfg_if::cfg_if;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
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

#[derive(Serialize)]


#[wasm_bindgen]
pub fn main(data:String) -> String{
    // create the handlebars registry
    let mut handlebars = Handlebars::new();

    //let test1;
    //let value: serde_json::Value = serde_wasm_bindgen::from_value(data).unwrap();

    let value:serde_json::Value = serde_json::from_str(data.as_str()).unwrap();



    // register the template. The template string will be verified and compiled.
    //let source = "hello {{world}}";
    let source2 = "<h1>Handlebars JS Example</h1>
        <table>
        <thead>
        <th>Name</th>
        <th>Job Title</th>
        <th>Twitter</th>
        </thead>
        <tbody>
        {{#users}}
        <tr>
        <td>{{fullName person}}</td>
        <td>{{jobTitle}}</td>
        <td><a href='https://twitter.com/{{twitter}}'>@{{twitter}}</a></td>
        </tr>
        {{/users}}
        </tbody>
        </table>";
    //assert!(handlebars.register_template_string("t1", source).is_ok());

    // Prepare some data.
    //
    // The data type should implements `serde::Serialize`
    //let mut data = BTreeMap::new();
    //data.insert("world".to_string(), "世界!".to_string());



    handlebars.register_template_string("hello", source2); //bind le template source2 avec le nom "hello"
    handlebars.render("hello", &value).unwrap() //render le template nommer "hello" avec l'objet test1


    //data.as_string().unwrap()
    //data.to_string()
    //value.to_string()
}
