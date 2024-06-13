use dioxus::prelude::*;
use nostr_sdk::Event;
use crate::components::Avatar;
use crate::nostr::get_event_by_id;
use crate::nostr::multiclient::MultiClient;
use crate::components::icons::LOADING;
use crate::nostr::note::TextNote;

#[derive(PartialEq, Clone, Props)]
pub struct NotifReplyProps {
    pub event: Event,
    pub relay_name: String,
}

#[component]
pub fn NotifReply(props: NotifReplyProps) -> Element {
    let text_note = TextNote::try_from(props.event.clone()).unwrap();
    let relay_name = props.relay_name.clone();
    let event: Signal<Event> = use_signal(|| props.event.clone());
    let mut is_loading: Signal<bool> = use_signal(|| true);
    let multiclient = use_context::<Signal<MultiClient>>();
    let mut root_event: Signal<Event> = use_signal(|| props.event.clone());

    //loading notifi root
    use_effect(use_reactive((&event,&relay_name,&text_note), {
        let multiclient: Signal<MultiClient> = multiclient.clone();
        move |(event,relay_name,text_note)| {
            spawn(async move {
                let clients = multiclient();
                match text_note.get_root() {
                    Some(event_id) => {
                        if let Some(client) = clients.get_client(&relay_name).await {
                            let client = client.client();
                            //get to data
                            match client.database().event_by_id(event_id).await {
                                Ok(db_events) => {
                                    // tracing::info!("db events are: {:?}", db_events);
                                    root_event.set(db_events);
                                    is_loading.set(false);
                                    return;
                                }
                                Err(_) => {
                                    // is_fetch = true;
                                }
                            }

                            //get to relay
                            match get_event_by_id(&client, &event_id, None).await {
                                Ok(root) => {
                                    if let Some(event) = root {
                                        // tracing::info!("relay events are: {:?}", event);
                                        root_event.set(event);
                                        is_loading.set(false);
                                        return;
                                    }
                                }
                                Err(e) => {
                                    is_loading.set(false);
                                    tracing::error!("get_event_by_id error: {:?}", e);
                                }
                            }
                        } else {
                            is_loading.set(false);
                            tracing::error!("client not found");   
                        }
                    }
                    None =>{
                        is_loading.set(false);
                        tracing::error!("notif root EventId not found");   
                    }
                }
            });
        }
    
    }));

    rsx! {

        div{
            class:"notificatio-box cont-box",
            if !is_loading() {
                div {
                    class: "header display-flex-box",
                    Avatar {
                        pubkey: root_event().author(),
                        timestamp: root_event().created_at().as_u64(),
                        relay_name: relay_name,
                        is_text_ellipsis: true,
                    }
                }
                div{
                    class:"content font-size-14",
                    "{root_event().content()}"
                }
            }


            if is_loading() {
                div {
                    class: "laoding-box",
                    dangerous_inner_html: "{LOADING}"
                }
            }
            
        }
        // {root_rsx}
    }
}
