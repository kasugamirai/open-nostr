use dioxus::prelude::*;
use crate::storage::*;

#[component]
pub fn Search() -> Element {

    let add_data = move || {
        spawn(async move {
            add_data("DEFAULT_STORE", "Key1", &String::from("Value1")).await.unwrap();
        });
    };

    let delete_data = move || {
        spawn(async move {
            delete_data("DEFAULT_STORE", "Key1").await.unwrap();
        });
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
