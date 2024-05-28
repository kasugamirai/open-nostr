use crate::nostr::multiclient::MultiClient;
use crate::{
    components::Avatar,
    nostr::{fetch::get_event_by_id, note::TextNote},
};
use dioxus::prelude::*;
use nostr_sdk::Event;
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
        let mut root_rsx = root_rsx.clone();
        // let sub_name = props.sub_name.clone();
        let relay_name = props.relay_name.clone();
        let eventid = text_note.get_root().unwrap();

        spawn(async move {
            let clients = multiclient();
            let client = clients.get_client(&relay_name).unwrap();
            match get_event_by_id(&client.client(), &eventid, None).await {
                Ok(root) => {
                    if let Some(root_event) = root {
                        root_rsx.set(rsx! {
                            div {
                                class: "quote flex items-center display-flex-box items-center",
                                div {
                                    class:"font-weight-bold display-flex-box items-center justify-content-center w-52",
                                    "Re:"
                                }
                                div {
                                   class:"qt-text",
                                   Avatar {
                                        pubkey: root_event.author(),
                                        timestamp: root_event.created_at().as_u64(),
                                        relay_name: relay_name.clone(),
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
                                }
                            }
                        })
                    }
                }
                Err(e) => {
                    tracing::error!("get_event_by_id error: {:?}", e);
                }
            }
        });
    }));
    rsx! {
        {root_rsx}
    }
}
