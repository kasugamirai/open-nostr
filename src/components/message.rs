use dioxus::prelude::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn setTimeout(closure: &Closure<dyn FnMut()>, millis: u32) -> i32;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[component]
pub fn Message(content: String) -> Element {
    // let mut str = use_signal(||String::from(""));
    // str.set(content.clone());

    if !content.is_empty() {
        let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(|| {
            // content.set("");
            log("清空")
        }) as Box<dyn FnMut()>);

        setTimeout(&closure, 2000);

        closure.forget();

        rsx! {
          div {
              class: "message-content font-size-16 lh-20",
              {content}

            }
        }
    } else {
        rsx! {}
    }
}
