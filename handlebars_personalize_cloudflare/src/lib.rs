extern crate js_sys;

use cfg_if::cfg_if;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Number, Map, Error};
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

    let site = "http://miguel-gouveia.me";

    let url = urlparse(request.url()); //retourne un string de type https://example.com/example

    let mut result = "".to_string();
    let mut json = "".to_string();
    let mut json_obj:Value;
    let mut template = "".to_string();
    let mut target_template= "".to_string();
    let mut template_name = "default".to_string();



    if url.query.is_some() {
        let query = url.get_parsed_query().unwrap();
        //traité query
    }

    /*if url.path != "/" { //path is_empty() ne marche pas, vu qu'il y a toujours un /
        let mut v: Vec<&str> = url.path.rsplit(".").collect();

        let target_json = format!("{}/content{}.json", site, v.pop().unwrap());

        json = fetch_rust_wasm(&target_json).await.unwrap();
        if json.starts_with("<") { //detect 404
            result = json; //return 404
        } else {
            //on deserialize le JSON
            let json_obj: Value = serde_json::from_str(&json).unwrap();

            let mut template_directory = "".to_string();

            //on check la template
            if json_obj.get("ressourceType").is_some() {
                template_directory = format!("{}/",json_obj.get("ressourceType").unwrap().as_str().unwrap().to_string());
            }

            if !v.is_empty() {
                template_name = v.pop().unwrap().to_string();
            }
            let target_template = format!("{}/template/{}{}.hbs", site, template_directory, template_name);

            template = fetch_rust_wasm(&target_template).await.unwrap();


            handlebars.register_template_string("hello", template); //bind le template source2 avec le nom "hello"
            result = handlebars.render("hello", &json_obj).unwrap() //render le template nommer "hello" avec l'objet test1
        }

    }*/

    let mut v: Vec<&str> = url.path.rsplit(".").collect();

    let target_json = format!("{}/content{}.json", site, v.pop().unwrap());

    json = fetch_rust_wasm(&target_json).await.unwrap();


    //on deserialize le JSON
    let json_obj_result: Result<Value, Error> = serde_json::from_str(&json);

    if json_obj_result.is_ok() {
        json_obj= json_obj_result.unwrap();
        let mut template_directory = "".to_string();

        //on check le ressourceType
        if json_obj.get("ressourceType").is_some() {
            template_directory = format!("{}/",json_obj.get("ressourceType").unwrap().as_str().unwrap().to_string());
        }

        if !v.is_empty() {
            template_name = v.pop().unwrap().to_string();
        }
        target_template = format!("{}/template/{}{}.hbs", site, template_directory, template_name);


    } else {
        target_template = "http://miguel-gouveia.me/template/error-404.hbs".to_string();
        
        let error = format!("{{ \"error-message\": \"Le fichier {} n'est pas présent!\" }}", target_json);
        json_obj = serde_json::from_str(&error).unwrap();
    }

    template = fetch_rust_wasm(&target_template).await.unwrap();

    handlebars.register_template_string("hello", template); //bind le template source2 avec le nom "hello"
    result = handlebars.render("hello", &json_obj).unwrap(); //render le template nommer "hello" avec l'objet test1


    result
}
