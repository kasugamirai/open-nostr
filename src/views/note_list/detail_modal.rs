// detail modal
// if *show_detail.read() {
//     div {
//         style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background-color: rgba(0, 0, 0, 0.5); z-index: 99999999;",
//         div {
//             style: "width: 50%; height: 60%; max-width: 900px; background-color: #fff; position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); padding: 20px; border-radius: 10px;",
//             button {
//                 class: "btn-icon remove",
//                 style: "position: absolute; top: -12px; left: -12px;",
//                 onclick: move |_| {
//                     show_detail.set(false);
//                 },
//                 dangerous_inner_html: "{FALSE}",
//             }
//             pre {
//                 style: "height: 100%; overflow-y: auto; font-size: 16px;",
//                 "{detail}"
//             }
//         }
//     }
// }
use dioxus::prelude::*;

use crate::components::icons::FALSE;
use crate::components::ModalManager;
#[component]
pub fn DetailModal(detail: String, id: String) -> Element {
    let mut modal_manager = use_context::<Signal<ModalManager>>();
    let on_close = move |_| {
        modal_manager.write().close_modal(&id.clone());
    };
    rsx! {
        div {
            onclick: on_close.clone(),
            style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background-color: rgba(0, 0, 0, 0.5); z-index: 105;",
            div {
                style: "width: 50%; height: 60%; max-width: 900px; background-color: #fff; position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); padding: 20px; border-radius: 10px;",
                button {
                    class: "btn-icon remove",
                    style: "position: absolute; top: -12px; left: -12px;",
                    onclick: on_close.clone(),
                    dangerous_inner_html: "{FALSE}",
                }
                pre {
                    style: "height: 100%; overflow-y: auto; font-size: 16px;",
                    "{detail}"
                }
            }
        }
    }
}
