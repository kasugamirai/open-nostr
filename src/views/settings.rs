use dioxus::prelude::*;
use serde_json::Value;

use crate::components::{InputCard, Switch};

#[component]
pub fn Settings() -> Element {
    let mut text = use_signal(|| "Hello".to_string());
    let handle_change = move |value| {
        text.set(if value {
            "Goodbye".to_string()
        } else {
            "Hello".to_string()
        });
    };
    let handle_input = move |value| {
        text.set(value);
    };
    rsx! {
        h1 { "Settings" }
        Switch { on_change: handle_change, value: false, close: rsx!{ "Goodbye" }, open: rsx! { "{text}" } }
        p { "{text}" }
        InputCard { on_change: handle_input, value: false }
        Dropdown {}
    }
}

#[component]
pub fn Dropdown() -> Element {
    let mut show = use_signal(|| true);
    let test = use_signal(|| Value::Bool(false));

    let get_events = move || {
        spawn(async move {
            let mut eval = eval(
                r#"
                    // Listens for clicks on the 'document' element
                    let eid = await dioxus.recv()
                    const handle = (e) => {
                        let target = e.target
                        while (true) {
                            if (target.classList.contains(eid)) {
                                // The element is a child of the dropdown
                                dioxus.send("")
                                return
                            } else {
                                if (target === document.documentElement) {
                                    break
                                }
                            }
                            target = target.parentNode
                        }
                        
                        // The element is outside the dropdown
                        dioxus.send(false)

                        // Remove the event listener
                        // document.removeEventListener('click', handle)
                    }
                    document.addEventListener('click', handle)
                "#,
            );
            eval.send("test-dropdown".into()).unwrap();
            if let Value::Bool(res) = eval.recv().await.unwrap() {
                show.set(res);
            }
        });
    };
    get_events();
    rsx! {
        div {
            class: "test-dropdown",
            id: "test-dropdown",
            style: r#"
                position: relative;
            "#,
            div {
                class: "test-dropdown-trigger",
                style: r#"
                    width: 100px;
                    height: 40px;
                    background-color: #ccc;
                    cursor: pointer;
                "#,
                onclick: move |_| {
                    // get_events();
                    show.set(!show());
                }
            }
            div {
                class: "test-dropdown-content test-dropdown",
                style: if show() {
                    r#"
                        position: absolute;
                        width: 200px;
                        height: 150px;
                        background-color: #eee;
                        display: block;
                    "#
                } else {
                    r#"
                        position: absolute;
                        width: 200px;
                        height: 150px;
                        background-color: #eee;
                        display: none;
                    "#
                },
                "{test}"
            }
        }
    }
}
