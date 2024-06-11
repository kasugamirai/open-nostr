use dioxus::prelude::*;
use nostr_sdk::{Event, JsonUtil};

use crate::components::Avatar;
use crate::nostr::get_event_by_id;
use crate::nostr::multiclient::MultiClient;
use crate::nostr::note::TextNote;
/// This is the reply view for the note component
/// /
///
///
///
///

#[derive(PartialEq, Clone, Props)]
pub struct ReplyProps {
    pub event: Event,
    pub sub_name: String,
    pub relay_name: String,
}

#[component]
pub fn Reply(props: ReplyProps) -> Element {
    let text_note = TextNote::try_from(props.event.clone()).unwrap();
    let event = use_signal(|| props.event.clone());
    let root_rsx = use_signal(|| {
        rsx! {
            div {
                class: "pl-52",
                "Root note Loading..."
            }
        }
    });
    let multiclient = use_context::<Signal<MultiClient>>();
    use_effect(use_reactive(&event, move |_| {
        let mut root_rsx = root_rsx;
        // let sub_name = props.sub_name.clone();
        let relay_name = props.relay_name.clone();
        let eventid = text_note.get_reply_to().unwrap();

        spawn(async move {
            let clients = multiclient();
            if let Some(client) = clients.get_client(&relay_name).await {
                match get_event_by_id(&client.client(), &eventid, None).await {
                    Ok(root) => {
                        if let Some(root_event) = root {
                            root_rsx.set(rsx! {
                                div {
                                    class: "quote flex items-center display-flex-box items-center",
                                    div {
                                        class:"quote-box-style  font-weight-bold w-52",
                                        "Re:"
                                    }
                                    div {
                                       class:"qt-text",
                                       Avatar {
                                            pubkey: root_event.author(),
                                            timestamp: root_event.created_at().as_u64(),
                                            relay_name: relay_name.clone(),
                                            is_text_ellipsis: true,
                                        }
                                        div {
                                            class:"relative qt-text-content",
                                            // two-line-truncate
                                            span{
                                              class:"re-text two-line-truncate relative",
                                              dangerous_inner_html: "{root_event.content()}"
                                            }
                                            span{
                                              class:"more-show-style pl-4",
                                              "show more"
                                            }
                                        }
                                        div{
                                            style: "display: none",
                                            {root_event.as_json()}
                                        }
                                    }
                                }
                            })
                        }
                    }
                    Err(e) => {
                        tracing::error!("get_event_by_id error: {:?}", e);
                    }
                }
            } else {
                tracing::error!("client not found");
            }
        });
    }));
    rsx! {
        {root_rsx}
    }
}
