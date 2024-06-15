use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;
use serde_json::Value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use nostr_sdk::{PublicKey,EventId};
use crate::store::subscription::FilterTemp;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct ScrollInfo {
    pub scroll_top: i32,
    pub scroll_height: i32,
    pub client_height: i32,
}

#[wasm_bindgen]
impl ScrollInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(scroll_top: i32, scroll_height: i32, client_height: i32) -> ScrollInfo {
        ScrollInfo {
            scroll_top,
            scroll_height,
            client_height,
        }
    }
}

pub fn export_to_clipboard(text: String) -> UseEval {
    let eval: UseEval = eval(
        r#"
            let c = navigator.clipboard;
            if (!c) {
                console.error('Clipboard not supported');
                return false;
            }
            let msg = await dioxus.recv();
            console.log(msg);
            await c.writeText(msg);
            alert("Copied to clipboard");
            return true;
        "#,
    );
    eval.send(text.into()).unwrap();
    eval
}

pub async fn import_from_clipboard() -> String {
    let mut eval: UseEval = eval(
        r#"
            let c = navigator.clipboard;
            if (!c) {
                console.error('Clipboard not supported');
                return false;
            }
            let msg = await c.readText();
            console.log(msg);
            await dioxus.send(msg);
            return true;
        "#,
    );
    let res = eval.recv().await.unwrap();
    if let Value::String(res) = res {
        res
    } else {
        "".into()
    }
}

pub async fn alert(msg: String) {
    let eval: UseEval = eval(
        r#"
        let msg = await dioxus.recv();
        alert(msg);
        "#,
    );
    eval.send(msg.into()).unwrap();
}

pub async fn note_srcoll_into_view(node_id: &str) {
    let eval: UseEval = eval(
        r#"
        let data = await dioxus.recv();
        let node = document.querySelector(`#note-${data.get("nodeId")}`);
        if (!node) {
            console.error('Node not found');
            return false;
        }
        node.scrollIntoView({ behavior: 'smooth', block: 'start'});
        "#,
    );
    eval.send({
        let mut map = serde_json::Map::new();
        map.insert("nodeId".into(), node_id.into());
        Value::Object(map)
    })
    .unwrap();
}


pub async fn verify_filters( _filters: &Vec<FilterTemp>) -> Result<String,String> {
    if _filters.len()<=0 {
        return Err("Filters cannot be empty!".to_string());
    }
    for filter in _filters.iter(){
        match filter {
            FilterTemp::Accounts(_accounts) =>{
                if _accounts.kinds.len()<=0 {
                    // alert().await;
                    return Err("Kinds cannot be empty!".to_string());
                }else if _accounts.accounts.len() <= 0 {
                    // alert("Accounts cannot be empty!".to_string()).await;
                    return Err("Accounts cannot be empty!".to_string());
                }

                
                for account in _accounts.accounts.iter() {
                    tracing::info!("loading accounts: {:?}", account);
                    if account.npub.is_empty() {
                        return Err(format!("The {} value in Accounts is empty", account.alt_name));
                    }else if let Err(e) = PublicKey::parse(&account.npub) {
                        return Err(format!("The format of the Aoounts->npub/pubkey is incorrect (alt name->:{},id/EventId:{})", account.alt_name,account.npub));
                    }
                }
            },
            FilterTemp::Events(_events) =>{
                if _events.events.len()<=0 {
                    return Err("Notes cannot be empty!".to_string());
                }

                for event in _events.events.iter() {
                    if event.nevent.is_empty() {
                        // alert(format!("The {} value in Notes is empty", event.alt_name)).await;
                        return Err(format!("The {} value in Notes is empty", event.alt_name));
                    }else if let Err(e) = EventId::parse(&event.nevent) {
                        return Err(format!("The format of the Notes->id/EventId is incorrect (alt name->:{},id/EventId:{})", event.alt_name,event.nevent));
                    }
                    
                }
            },
            FilterTemp::HashTag(_tags) =>{
                let _tags = _tags.clone();
                if _tags.tags.len()<=0 {
                    return Err("Tags cannot be empty!".to_string());
                }
            },
            FilterTemp::Customize(customize) =>{
            }
        }
    }
    return Ok("ok".to_string());
}

// 定义节流函数
#[wasm_bindgen]
pub fn throttle(callback: JsValue, delay: u32) -> JsValue {
    let last_call_time = Rc::new(RefCell::new(0.0));
    let is_throttling = Rc::new(RefCell::new(false));

    let throttled = Closure::wrap(Box::new(move || {
        let window = window().expect("no global `window` exists");
        let now = window
            .performance()
            .expect("should have `performance` on window")
            .now();

        if *is_throttling.borrow() {
            return;
        }

        *is_throttling.borrow_mut() = true;

        let is_throttling_clone = is_throttling.clone();
        let reset_throttling = Closure::wrap(Box::new(move || {
            *is_throttling_clone.borrow_mut() = false;
        }) as Box<dyn Fn()>);

        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                reset_throttling.as_ref().unchecked_ref(),
                delay as i32,
            )
            .expect("should register `setTimeout` OK");

        if now - *last_call_time.borrow() >= delay as f64 {
            *last_call_time.borrow_mut() = now;
            let callback_func = callback.clone();
            let func: &js_sys::Function = callback_func.as_ref().unchecked_ref();
            func.call0(&JsValue::NULL).unwrap();
        }

        reset_throttling.forget();
    }) as Box<dyn Fn()>);

    throttled.into_js_value()
}

#[wasm_bindgen]
pub fn get_scroll_info(scrollid: &str) -> Result<ScrollInfo, JsValue> {
    let res = { window() };
    match res {
        None => {
            log("no global `window` exists");
            Err(JsValue::from_str("no global `window` exists"))
        }
        Some(window) => {
            let document = window.document();
            match document {
                None => {
                    log("should have a document on window");
                    Err(JsValue::from_str("should have a document on window"))
                }
                Some(document) => {
                    let content = document.get_element_by_id(scrollid);
                    match content {
                        None => {
                            log("should have #content on the page");
                            Err(JsValue::from_str("should have #content on the page"))
                        }
                        Some(content) => {
                            let scroll_top = content.scroll_top();
                            let scroll_height = content.scroll_height();
                            let client_height = content.client_height();

                            let scroll_info =
                                ScrollInfo::new(scroll_top, scroll_height, client_height);
                            Ok(scroll_info)
                        }
                    }
                }
            }
        }
    }
}
