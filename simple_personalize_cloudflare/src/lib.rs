extern crate js_sys;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use log::{info, Level};
use js_sys::{JsString, Boolean};

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

fn getCookie(request: Request, cookie_name: String) -> bool{
    let mut result = false;
    let cookieStringOption = request.headers().get("Cookie").unwrap();
    if cookieStringOption.is_some() {
        let cookieString = cookieStringOption.unwrap();
        let cookies: Vec<&str> = cookieString.rsplit(";").collect(); //vecteur
        for cookie in cookies.iter() {
            if cookie.rsplit("=").into_iter().nth(0).unwrap() == cookie_name {
                result = true
            }
        }
    }
    result
}

#[wasm_bindgen]
pub fn test(request:Request) -> Result<String, JsValue> {
    let url = request.url();
    let met = request.method();
    let header = request.headers();
    let all = header.get("Accept-Language").unwrap().unwrap();

    let mut testt= "C'est votre première visite.";
    if getCookie(request, "visite".parse().unwrap()) {
        testt = "Bon retour.";
    }



    Ok(testt.parse().unwrap())
}
