use crate::storage::*;
use dioxus::prelude::*;

#[component]
pub fn Search() -> Element {
    let add_data = move || {
        spawn(async move {
            let db = CapybastrDb::new("DEFAULT_STORE".to_string()).await.unwrap();
            db.add_data("Key1", &String::from("Value1")).await.unwrap();
        });
    };

    let delete_data = move || {
        spawn(async move {
            let db = CapybastrDb::new("DEFAULT_STORE".to_string()).await.unwrap();
            db.delete_data("Key1").await.unwrap();
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
