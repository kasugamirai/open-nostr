use dioxus::{html::text, prelude::*};
use nostr::event;
use nostr_sdk::Event;
use crate::{
    components::Avatar, 
    nostr::{fetch::get_event_by_id, note::TextNote}, 
    views::note_list::note::Note};
use crate::nostr::multiclient::MultiClient;
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
    use_effect(use_reactive(&event, move |event| {
        let mut root_rsx = root_rsx.clone();
        let sub_name = props.sub_name.clone();
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
                                class: "quote flex items-center",
                                style: "display: flex; align-items: center;",
                                div {
                                    style: "font-weight: bold; width: 52px; display: flex; align-items: center; justify-content: center;",
                                    "Re:"
                                }
                                div {
                                    style: "flex: 1; border: 1px solid #333; border-radius: 20px; overflow: hidden; padding: 4px; display: flex; gap: 12px; background: #fff; height: 50px;",
                                    Avatar {
                                        pubkey: root_event.author(),
                                        timestamp: root_event.created_at().as_u64(),
                                        relay_name: relay_name.clone(),
                                    }
                                    div {
                                        style: "flex: 1; font-size: 14px; line-height: 20px; border-left: 2px solid #b4b4b4; padding: 0 12px;",
                                        dangerous_inner_html: "{root_event.content()}"
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