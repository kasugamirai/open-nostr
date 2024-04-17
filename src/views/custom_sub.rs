use dioxus::prelude::*;

use crate::{
    components::{icons::*, Dropdown, InputCard},
    state::{CustomSubs, Filter, Kind},
    utils::format::{format_public_key, format_timestamp},
};

#[component]
pub fn CustomSub() -> Element {
    let mut custom_subs_signal = consume_context::<Signal<CustomSubs>>();
    let mut subs = use_signal(|| custom_subs_signal().clone().filters);
    let mut edit = use_signal(|| false);

    // let get_filters = move || {
    //     spawn(async move {
    //         let database = WebDatabase::open("capybastr-filters").await.unwrap();
    //         subs.set(Vec::<Filter>::new());
    //     });
    // };
    // get_filters();

    // let set_filters = move || {
    //     spawn(async move {
    //         let database = WebDatabase::open("capybastr-filters").await.unwrap();
    //         subs.set(Vec::<Filter>::new());
    //     });
    // };

    let mut handle_click = move |t| {
        let f = match t {
            0 => Filter::new_tag(),
            1 => Filter::new_account(),
            2 => Filter::new_event(),
            _ => Filter::new_custom(),
        };
        subs.push(f);
    };
    let mut handle_add_kind = move |index: usize| {
        // TODO: Remove first and then insert to update the page, should be better
        let mut f = subs.remove(index).clone();
        f = f.kind(Kind::from(nostr_sdk::Kind::Metadata));
        subs.insert(index, f);
    };
    let mut handle_remove = move |index: usize| {
        subs.remove(index);
    };
    let mut handle_export = move || {
        let eval = eval(
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
        let mut tmp = custom_subs_signal();
        tmp.filters = subs().clone();
        custom_subs_signal.set(tmp.clone());
        eval.send(tmp.json().into()).unwrap();
    };

    rsx! {
        div {
            class: "custom-sub",
            div {
                class: "custom-sub-header",
                h2 { "Custom Sub" }
                div {
                    class: "custom-sub-header-more",
                    Dropdown {
                        pos: "right".to_string(),
                        trigger: rsx! {
                            button {
                                class: "trigger",
                                dangerous_inner_html: "{MORE}"
                            }
                        },
                        children: rsx! {
                            div {
                                class: "content",
                                button {
                                    class: "content-btn",
                                    "Import"
                                }
                                button {
                                    class: "content-btn",
                                    onclick: move |_| handle_export(),
                                    "Export"
                                }
                                if edit() {
                                    button {
                                        class: "content-btn",
                                        onclick: move |_| edit.set(false),
                                        "Save"
                                    }
                                    button {
                                        class: "content-btn",
                                        onclick: move |_| edit.set(false),
                                        "Reset"
                                    }
                                } else {
                                    button {
                                        class: "content-btn",
                                        onclick: move |_| edit.set(true),
                                        "Edit"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "custom-sub-content",
                div {
                    class: "custom-sub-name",
                    "Name:"
                    div {
                        class: "card disabled",
                        "{custom_subs_signal().name}"
                    }
                }
                div {
                    class: "custom-sub-relays",
                    "Relays:"
                    div {
                        class: "card disabled",
                        "{custom_subs_signal().relays.name}"
                    }
                }
                div {
                    class: "custom-sub-filters",
                    "Filters:"
                }
                for (i, sub) in subs.iter().enumerate() {
                    div {
                        class: "custom-sub-item",
                        button {
                            class: "custom-sub-item-remove {edit}",
                            onclick: move |_| handle_remove(i) ,
                            dangerous_inner_html: "{ADD}"
                        }
                        if let Some(authors) = sub.accounts.clone() {
                            div {
                                class: "custom-sub-filter-item",
                                span {
                                    class: "title",
                                    "Authors:"
                                }
                                for author in authors {
                                    // div {
                                    //     class: "card custom-sub-author",
                                    //     "{format_public_key(&author.to_hex(), Some(6))}"
                                    // }
                                    InputCard {
                                        on_change: move |_| {},
                                        placeholder: None,
                                        value: format_public_key(&author.npub, Some(6)),
                                    }
                                }
                                button {
                                    class: "btn-add {edit}",
                                    onclick: move |_| handle_add_kind(i) ,
                                    dangerous_inner_html: "{ADD}"
                                }
                            }
                        }
                        if let Some(kinds) = sub.kinds.clone() {
                            div {
                                class: "custom-sub-filter-item",
                                span {
                                    class: "title",
                                    "Kinds:"
                                }
                                for kind in kinds {
                                    div {
                                        class: "card custom-sub-kind",
                                        "{kind.text}"
                                    }
                                }
                                button {
                                    class: "btn-add {edit}",
                                    onclick: move |_| handle_add_kind(i) ,
                                    dangerous_inner_html: "{ADD}"
                                }
                            }
                        }
                        if sub.since.is_some() || sub.until.is_some() {
                            div {
                                class: "custom-sub-filter-item",
                                span {
                                    class: "title",
                                    "Time:"
                                }
                                if let Some(since) = sub.since.clone() {
                                    div {
                                        class: "card custom-sub-time",
                                        "{format_timestamp(since, None)}"
                                        span {
                                            dangerous_inner_html: "{RIGHT}"
                                        }
                                    }
                                }
                                if let Some(until) = sub.until.clone() {
                                    div {
                                        class: "card custom-sub-time",
                                        span {
                                            dangerous_inner_html: "{LEFT}"
                                        }
                                        "{format_timestamp(until, None)}"
                                    }
                                }
                            }
                        }
                        if let Some(limit) = sub.limit.clone() {
                            div {
                                class: "custom-sub-filter-item",
                                span {
                                    class: "title",
                                    "Limits:"
                                }
                                div {
                                    class: "card custom-sub-limits",
                                    "{limit}"
                                }
                            }
                        }
                        if let Some(tags) = sub.tags.clone() {
                            div {
                                class: "custom-sub-filter-item",
                                span {
                                    class: "title",
                                    "Tags:"
                                }
                                for tag in tags {
                                    div {
                                        class: "card custom-sub-tag",
                                        "#{tag}"
                                    }
                                }
                            }
                        }
                    }
                }
                div {
                    class: "custom-sub-add",
                    span {
                        Dropdown {
                            trigger: rsx! {
                                button {
                                    class: "btn-add",
                                    dangerous_inner_html: "{ADD}"
                                }
                            },
                            children: rsx! {
                                div {
                                    class: "btn-add-content",
                                    button {
                                        class: "btn-add-item",
                                        onclick: move |_| handle_click(0),
                                        "Only Tags"
                                    }
                                    button {
                                        class: "btn-add-item",
                                        onclick: move |_| handle_click(1),
                                        "Follow People"
                                    }
                                    button {
                                        class: "btn-add-item",
                                        onclick: move |_| handle_click(2),
                                        "Follow Notes"
                                    }
                                    button {
                                        class: "btn-add-item",
                                        onclick: move |_| handle_click(3),
                                        "Customize"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
