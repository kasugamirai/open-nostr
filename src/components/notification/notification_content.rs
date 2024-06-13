use dioxus::prelude::*;
use nostr_sdk::{Event,JsonUtil,Kind};
use crate::components::{
  Avatar,
  notification::notification_reply::NotifReply
};
use crate::components::icons::{LIKEICON,TURN_LEFT,TURN_RIGHT,QUTE,ZAP};
#[derive(PartialEq, Clone, Props)]
pub struct NotifContenteProps {
    pub notif_event: Event,
    pub relay_name: String,
}

#[component]
pub fn NotificationContent(props: NotifContenteProps) -> Element {
  let event = props.notif_event.clone();
  let kind: Kind = props.notif_event.kind().clone();
  let relay_name=  props.relay_name.clone();

  //get icon by event kind
  let _event = event.clone();
  let get_icon = move || -> String{
    match kind {
      Kind::Reaction => LIKEICON.to_string(),
      Kind::TextNote => {
          if _event.content.contains("nostr:") {
              QUTE.to_string()
          } else {
              TURN_LEFT.to_string()
          }
      }
      Kind::Repost => TURN_RIGHT.to_string(),
      Kind::ZapReceipt => ZAP.to_string(),
      _ => "".to_string()
    }
  };
  
  rsx! {
    //content 
    div{
      class:"notificatio-box",
      Avatar {
        pubkey: event.pubkey.clone(),
        timestamp: event.created_at.as_u64(),
        relay_name: relay_name.clone(),
        repost_event: match event.kind() {
            Kind::Repost => {
                let repost_event = Event::from_json(&event.content).unwrap();
                Some(repost_event)
            }
            _=> None
        },
      }

      span{
        class:"notificatio-rigth-icon",
        dangerous_inner_html: "{get_icon()}"
      }
      
      div{
        class:"content",
        "{event.content}"
      }
      
      NotifReply {
        event: event.clone(),
        relay_name: props.relay_name.clone(),
      }
    }
  }
}
