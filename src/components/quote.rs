use dioxus::prelude::*;
use nostr_sdk::*;

use crate::components::Avatar;
use crate::nostr::fetch::get_event_by_id;
use crate::nostr::multiclient::MultiClient;

#[derive(PartialEq, Clone, Props)]
pub struct QouteProps {
    event_id: EventId,
    relay_name: String,
    quote_nostr: String,
}

#[component]
pub fn Quote(props: QouteProps) -> Element {
    // let mut pk = use_signal(|| );
    let multiclient = use_context::<Signal<MultiClient>>();
    let mut ele = use_signal(|| {
        rsx! {
            div {
                class: "pl-52",
                "Quote note Loading..."
            }
        }
    });
    use_effect(use_reactive(
        (&props.event_id, &props.relay_name, &props.quote_nostr),
        move |(event_id, relay_name, quote_nostr)| {
            spawn(async move {
                let clients = multiclient();
                // Await the get_client function
                let client = clients.get_client(&relay_name).await;
                let client = if let Some(client) = client {
                    client
                } else {
                    tracing::error!("client not found");
                    return;
                };

                // Fetch the event by ID
                match get_event_by_id(&client.client(), &event_id, None).await {
                    Ok(Some(event)) => {
                        let pk = event.author();
                        let content = event.content.to_string();
                        let timestamp = event.created_at.as_u64();
                        ele.set(rsx! {
                        div {
                            class: "quote flex items-center display-flex-box items-center",
                            style: "margin-left: -52px;",
                            div {
                                class: "font-weight-bold quote-box-style",
                                "Qt:"
                            }
                            div {
                                class: "qt-text",
                                Avatar {
                                    pubkey: pk,
                                    timestamp: timestamp,
                                    relay_name: relay_name.clone(),
                                    is_text_ellipsis: true,
                                }
                                div {
                                    class: "relative qt-text-content",
                                    span {
                                        class:"re-text two-line-truncate relative",
                                        dangerous_inner_html: "{content}"
                                    }
                                    span {
                                        class: "more-show-style pl-4",
                                        "show more"
                                    }
                                }
                            }
                        }
                    });
                    }
                    Ok(None) => {
                        tracing::info!("event not found");
                        ele.set(rsx! {
                            span {
                                class: "pl-52",
                                {quote_nostr}
                            }
                        });
                    }
                    Err(e) => {
                        tracing::error!("{:?}", e);
                        ele.set(rsx! {
                            span {
                                class: "pl-52",
                                {quote_nostr}
                            }
                        });
                    }
                }
            });
        },
    ));
    rsx! {
        {ele}
    }
}
