use dioxus::prelude::*;
use crate::storage::*;

#[component]
pub fn Search() -> Element {
    let db = CapybastrDb::new("DEFAULT_STORE");

    let db_for_add = db.clone();
    let add_data = move || {
        let db_clone = db_for_add.clone();
        spawn(async move {
            db_clone.add_data("Key1", &String::from("Value1")).await.unwrap();
        });
    };

    let db_for_delete = db.clone();
    let delete_data = move || {
        let db_clone = db_for_delete.clone();
        spawn(async move {
            db_clone.delete_data("Key1").await.unwrap();
        });
    };

    rsx! {
        button { onclick: move |_| {
            add_data.clone()();
        }, "add data" }

        button { onclick: move |_| {
            delete_data.clone()();
        }, "delete data" }
    }
}