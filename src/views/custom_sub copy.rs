use dioxus::prelude::*;

use crate::{
    components::{icons::*, Dropdown, InputCard},
    state::{CustomSubs, Filter, Kind},
    utils::format::{format_public_key, format_timestamp},
};

#[component]
pub fn CustomSub() -> Element {
    let mut custom_subs_signal = consume_context::<Signal<CustomSubs>>();
    let mut relay_name = use_signal(|| custom_subs_signal().relays.name);
    let mut relays = use_signal(|| custom_subs_signal().relays.relays);
    let mut new_relay = use_signal(|| String::new());
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
    let mut handle_kind = move |index: usize, add: bool, kind: nostr_sdk::Kind| {
        // TODO: Remove first and then insert to update the page, should be better
        let mut f = subs.remove(index).clone();
        if add {
            f = f.kind(Kind::from(kind));
        } else {
            f = f.remove_kind(Kind::from(kind));
        }
        subs.insert(index, f);
    };
    let mut handle_add_account = move |index: usize| {
        let mut f = subs.remove(index).clone();
        f = f.empty_account();
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
        tmp.filters = subs();
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
                    InputCard {
                        edit: false,
                        on_change: move |v| {
                            let mut tmp = custom_subs_signal();
                            tmp.name = v;
                            custom_subs_signal.set(tmp);
                        },
                        placeholder: None,
                        value: custom_subs_signal().name,
                    }
                }
                div {
                    class: "custom-sub-relays",
                    "Relays:"
                    div {
                        style: "display: inline-block;",
                        Dropdown {
                            pos: "left".to_string(),
                            trigger: rsx! {
                                div {
                                    class: "card disabled",
                                    "{custom_subs_signal().relays.name}"
                                }
                            },
                            div {
                                style: r#"
                                    display: flex;
                                    flex-direction: column;
                                    gap: 10px;
                                    padding: 10px;
                                    border-radius: var(--radius-1);
                                    border: 1px solid var(--boc-1);
                                    background-color: var(--bgc-0);
                                "#,
                                div {
                                    style: "display: flex; gap: 10px;",
                                    input {
                                        style: r#"
                                            border: none;
                                            border-bottom: 1px solid var(--boc-1);
                                            font-size: 16px;
                                        "#,
                                        r#type: "text",
                                        value: "{relay_name}",
                                        oninput: move |event| {
                                            relay_name.set(event.value());
                                        }
                                    }
                                    button {
                                        class: "btn-icon right",
                                        onclick: move |_| {
                                            let mut tmp = custom_subs_signal();
                                            tmp.relays.name = relay_name();
                                            custom_subs_signal.set(tmp);
                                        },
                                        div {
                                            dangerous_inner_html: "{TRUE}"
                                        }
                                    }
                                }
                                for (i, relay) in relays.iter().enumerate() {
                                    div {
                                        style: "display: flex; gap: 10px;",
                                        input {
                                            style: r#"
                                                border: none;
                                                border-bottom: 1px solid var(--boc-1);
                                                font-size: 16px;
                                            "#,
                                            r#type: "text",
                                            value: "{relay}",
                                            oninput: move |event| {
                                                relays.write()[i] = event.value();
                                            }
                                        }
                                        button {
                                            class: "btn-icon remove",
                                            onclick: move |_| {
                                                relays.remove(i);
                                            },
                                            div {
                                                dangerous_inner_html: "{FALSE}"
                                            }
                                        }
                                    }
                                }
                                div {
                                    style: "display: flex; gap: 10px;",
                                    input {
                                        style: r#"
                                            border: none;
                                            border-bottom: 1px solid var(--boc-1);
                                            font-size: 16px;
                                        "#,
                                        r#type: "text",
                                        value: "{new_relay}",
                                        oninput: move |event| {
                                            new_relay.set(event.value());
                                        }
                                    }
                                    button {
                                        class: "btn-icon add",
                                        onclick: move |_| {
                                            relays.push(new_relay());
                                            new_relay.set("".to_string());
                                        },
                                        div {
                                            dangerous_inner_html: "{ADD}"
                                        }
                                    }
                                }
                            }
                        }
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
                                    div {
                                        "{author.npub}"
                                    }
                                    // InputCard {
                                    //     on_change: move |_| {},
                                    //     placeholder: None,
                                    //     value: format_public_key(&author.npub, Some(6)),
                                    // }
                                }
                                button {
                                    class: "btn-add {edit}",
                                    onclick: move |_| handle_add_account(i) ,
                                    dangerous_inner_html: "{ADD}"
                                }
                            }
                        }
                        if let Some(kinds) = &sub.kinds {
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
                                Dropdown {
                                    trigger: rsx! {
                                        button {
                                            class: "btn-add {edit}",
                                            dangerous_inner_html: "{ADD}"
                                        }
                                    },
                                    children: rsx! {
                                        div {
                                            class: "btn-add-content",
                                            input {
                                                r#type: "checkbox",
                                                oninput: move |event| {
                                                    let is_enabled = event.value() == "true";
                                                    handle_kind(i, is_enabled, nostr_sdk::Kind::TextNote)
                                                }
                                            }
                                            "Note"
                                            input {
                                                r#type: "checkbox",
                                                oninput: move |event| {
                                                    let is_enabled = event.value() == "true";
                                                    handle_kind(i, is_enabled, nostr_sdk::Kind::Repost)
                                                }
                                            }
                                            "Repost"
                                        }
                                    }
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
                                if let Some(since) = sub.since {
                                    div {
                                        class: "card custom-sub-time",
                                        "{format_timestamp(since, None)}"
                                        span {
                                            dangerous_inner_html: "{RIGHT}"
                                        }
                                    }
                                }
                                if let Some(until) = sub.until {
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
                        if let Some(limit) = &sub.limit {
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
                        if let Some(tags) = &sub.tags {
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

#[component]
fn EditRelays() -> Element {
    rsx! {
        div {
            "Relays"
        }
    }
}
