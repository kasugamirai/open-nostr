use dioxus::prelude::*;

use crate::{
    components::{icons::*, Dropdown},
    views::CustomSubscription,
    CustomSub,
};

#[component]
pub fn Subscription(subscription: String) -> Element {
    // all custom subscriptions
    let mut sub_all = use_context::<Signal<Vec<CustomSub>>>();

    let mut sub_current = use_signal(CustomSub::default);
    let mut sub_index = use_signal(|| 0);

    use_effect(use_reactive((&subscription,), move |(s,)| {
        tracing::info!("Current sub changed: {:?}", s);
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
    };

    rsx! {
        div {
            style: "flex: 1; height: 100%;",
            "1"
        }
        div {
            style: "width: 600px; height: 100%; position: relative; display: flex; flex-direction: column; gap: 10px;",
            CustomSubscription {
                on_save: handle_save,
                subscription: sub_current.read().clone(),
            }
        }
    }
}