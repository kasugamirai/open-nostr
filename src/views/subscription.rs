use dioxus::prelude::*;

use crate::{
    storage::CapybastrDb,
    views::{CustomSubscription, NoteList},
    CustomSub,
};

#[component]
pub fn Subscription(subscription: String) -> Element {
    // all custom subscriptions
    let mut sub_all = use_context::<Signal<Vec<CustomSub>>>();

    let mut sub_current = use_signal(CustomSub::default);
    let mut sub_index = use_signal(|| 0);

    use_effect(use_reactive((&subscription,), move |(s,)| {
        for (i, sub) in sub_all.read().iter().enumerate() {
            if sub.name == s {
                sub_current.set(sub.clone());
                sub_index.set(i);
            }
        }
    }));

    let handle_save = move |value: CustomSub| {
        sub_current.set(value);
        let index = *sub_index.read();
        let mut subs = sub_all.write();
        subs[index] = sub_current.read().clone();

        let s = sub_current();

        spawn(async move {
            let db = CapybastrDb::new("subscription".to_string()).await.unwrap();
            db.delete_data(&s.name).await.unwrap();
            db.add_data(&s.name, &s.json()).await.unwrap();
        });
    };

    let handle_reload = move |value: CustomSub| {
        tracing::info!("handle_reload: {value:?}");
        sub_current.set(value);
    };

    rsx! {
        div {
            style: "display: flex; width: 100%; height: 100%; gap: 20px;",
            div {
                style: "flex: 1; overflow-y: scroll; width: 100%;",
                NoteList {
                    subscription: sub_current.read().clone(),
                }
            }
            div {
                style: "width: 600px; height: 100%; position: relative; display: flex; flex-direction: column; gap: 10px;",
                CustomSubscription {
                    on_save: handle_save,
                    on_reload: handle_reload,
                    subscription: sub_current.read().clone(),
                }
            }
        }
    }
}
