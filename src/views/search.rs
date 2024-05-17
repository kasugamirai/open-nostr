use dioxus::prelude::*;

#[component]
pub fn Search() -> Element {
    let add_data = move || {
        spawn(async move {});
    };

    let delete_data = move || {
        spawn(async move {});
    };

    rsx! {
        button { onclick: move |_| {
            add_data();
        }, "add data" }

        button { onclick: move |_| {
            delete_data();
        }, "delete data" }
    }
}
