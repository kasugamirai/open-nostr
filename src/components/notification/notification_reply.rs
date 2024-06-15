use dioxus::prelude::*;
use nostr_sdk::{Event, EventId, Kind};
use crate::components::Avatar;
use crate::nostr::get_event_by_id;
use crate::nostr::MultiClient;
use crate::components::icons::LOADING;
use crate::nostr::TextNote;
use crate::utils::format::parse_notif_content_event;

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
    let mut is_succeed: Signal<bool> = use_signal(||false);
    let multiclient = use_context::<Signal<MultiClient>>();
    let mut root_event: Signal<Event> = use_signal(|| props.event.clone());
    let _relay_name = relay_name.clone();

    //loading notifi root data
    let loading_root = move | event_id: &EventId | {
        let multiclient: Signal<MultiClient> = multiclient.clone();
        let _event_id = event_id.clone();
        spawn(async move {
            let clients = multiclient();
            if let Some(client) = clients.get_client(&relay_name).await {
                let client = client.client();
                //get to db
                match client.database().event_by_id(_event_id).await {
                    Ok(db_events) => {
                        root_event.set(db_events);
                        is_succeed.set(true);
                        is_loading.set(false);
                        return;
                    }
                    Err(_) => {
                        // is_fetch = true;
                    }
                }

                //get to relay
                match get_event_by_id(&client, &_event_id, None).await {
                    Ok(root) => {
                        if let Some(event) = root {
                            root_event.set(event);
                            is_succeed.set(true);
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
        });
      };

    //loading notifi root
    use_effect(use_reactive((&text_note,&event), {
        move |(text_note,event)| {
            let loading_root = loading_root.clone();
            spawn(async move {
                let nt_defualt = "default";
                let nt_qutoe = "quote";
                let content = event().content.clone();

                //check type
                let notif_type = match event().kind {
                    Kind::TextNote => {
                        if content.contains("nostr:") {
                            nt_qutoe
                        } else{
                            nt_defualt
                        }
                    }
                    _ => nt_defualt
                };

                //notif qutoe 
                tracing::info!("note_id: {:?}", notif_type);
                if nt_qutoe.eq(notif_type) {
                    match parse_notif_content_event(&content){
                        Some(nip19Event)=>{
                            let _event_id = nip19Event.event_id.clone();
                            loading_root(&_event_id);
                            return;
                        }
                        None => {
                            is_loading.set(false);
                            tracing::error!("notif root EventId not found"); 
                            return;  
                        }
                    }
                }
                
                //notif (reply actions note)
                if nt_defualt.eq(notif_type) {
                    match text_note.get_root() {
                        Some(event_id) => {
                            let _event_id = event_id.clone();
                            loading_root(&_event_id);
                            return;
                        }   
                        None =>{
                            is_loading.set(false);
                            tracing::error!("notif root EventId not found"); 
                            return;  
                        }
                    }
                }
               
            });
        }
    
    }));

    rsx! {
        div{
            class:"notificatio-box cont-box",
            if !is_loading()&& is_succeed() {
                div {
                    class: "header display-flex-box",
                    Avatar {
                        pubkey: root_event().author(),
                        timestamp: root_event().created_at().as_u64(),
                        relay_name: _relay_name,
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
