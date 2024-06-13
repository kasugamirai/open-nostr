mod notification_content;
mod notification_reply;
use notification_content::NotificationContent;
use dioxus::prelude::*;
use crate::components::icons::*;
use crate::components::icons::LOADING;
use crate::components::ModalManager;
use std::time::Duration;
use nostr_sdk::{PublicKey,Event};
use crate::nostr::multiclient::MultiClient;
use crate::nostr::{NotificationPaginator,NotificationMsg};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use crate::utils::js::{get_scroll_info, throttle};
use crate::utils::format::format_create_at;
#[derive(PartialEq, Clone, Props)]
pub struct NotifContenteProps {
    pub public_key: PublicKey,
    pub relay_name: String,
}

#[component]
pub fn Notification(props: NotifContenteProps) -> Element {
    let multiclient = use_context::<Signal<MultiClient>>();
    let public_key = props.public_key.clone();
    let relay_name = props.relay_name.clone();
    let mut notif_events: Signal<Vec<NotificationMsg>> = use_signal(|| Vec::new());
    let mut paginator = use_signal(|| None);
    let mut is_loading = use_signal(|| false);
    let mut modal_manager = use_context::<Signal<ModalManager>>();
    let mut before_present_time_str = use_signal(|| "".to_string());

    //parse as event 
    let get_event =  move | notif_msg: &NotificationMsg| -> Event{
      match notif_msg {
        NotificationMsg::Emoji(event)
        | NotificationMsg::Reply(event)
        | NotificationMsg::Repost(event)
        | NotificationMsg::Quote(event)
        | NotificationMsg::ZapReceipt(event) => {
          let event_copy = event.clone();
          return event_copy;
        },
      }
    };

    // loading page data
    let handle_fetch = move || {
      spawn(async move {
          if !is_loading() {
              is_loading.set(true);
              let mut paginator_write: Write<Option<NotificationPaginator>, UnsyncStorage> =
                  paginator.write();
              let result = paginator_write.as_mut();
              if let Some(paginator) = result {
                  let notifi_msgs: Option<Vec<NotificationMsg>> = paginator.next_page().await;
                  match notifi_msgs {
                      Some(notifi_msgs) => {
                        notif_events.extend(notifi_msgs);
                        
                        //
                        let last_msg = notif_events.last();
                        match last_msg {
                          Some(notifi_msgs)=>{
                            let last_event: Event = get_event(&notifi_msgs);
                            before_present_time_str.set(format_create_at(last_event.created_at.as_u64()));
                          }
                          None =>{}
                        }
                        is_loading.set(false);
                      }
                      None => {
                        is_loading.set(false);
                      }
                  }
              }
          }
      });
    };

    // init paginator
    use_effect(use_reactive(
      (&public_key,&relay_name),
      move |(public_key,relay_name)| {
          let multiclient: Signal<MultiClient> = multiclient.clone();
          spawn(async move {
              let clients: MultiClient = multiclient();
              let client_result = clients.get_or_create(&relay_name.clone()).await;
              match client_result {
                  Ok(hc) => {
                    let client = hc.client();
                    let timeout: Option<Duration> = Some(std::time::Duration::from_secs(5));
                    let paginator_result: NotificationPaginator =
                        NotificationPaginator::new(client, public_key.clone(), timeout, 20, false);
                    paginator.set(Some(paginator_result));
                    handle_fetch();
                  }
                  Err(e) => {
                    tracing::error!("notification client Error: {:?}", e);
                  }
              }

          });
      },
    ));

    

    rsx! {
        // Custom Sub component
        div {
            class: "custom-sub-wrapper not-style display-flex-box relative flex-col",
            //title
            div {
                class: "custom-sub-header ml-16",
                h1 {
                    class: "title custom-sub-title font-raleway-800 font-size-20",
                    "Notification"
                }
                button {
                    class: "icon",
                    dangerous_inner_html: "{MORE}"
                }
            }

            

            div {
              class:"notificatio-contents",
              id: "notifi-list",
              onscroll: move |_| {
                let callback = Closure::wrap(Box::new({
                    move || {
                        match get_scroll_info("notifi-list") {
                            Ok(scroll_info) => {
                                let scroll_top = scroll_info.scroll_top;
                                let scroll_height = scroll_info.scroll_height;
                                let client_height = scroll_info.client_height;
                                if scroll_height - scroll_top - client_height <= 100 {
                                    handle_fetch();
                                }
                            }
                            Err(e) => {
                                tracing::error!("Error: {:?}", e);
                            }
                        }
                    }
                })  as Box<dyn Fn()>);
                let callback_js = callback.into_js_value();
                let throttled_callback = throttle(callback_js, 1000); // 300ms throttling delay
                let func: &js_sys::Function = throttled_callback.as_ref().unchecked_ref();
                func.call0(&JsValue::NULL).unwrap();

                modal_manager.write().destory_all_modals_by_level(4);
              },
               //data group 
               if !before_present_time_str().is_empty() {
                div{
                  class:"day-box ml-16 display-flex-box relative",
                  div{
                    class:"border000 mt-11"
                  }
                  span{
                    class:"absoulte",
                    "{before_present_time_str()}"
                  }
                }
              }

              //content 
              for notif_msg in notif_events.read().iter() {
                NotificationContent {
                  notif_event: get_event(&notif_msg),
                  relay_name: relay_name.clone(),
                }
              }

              if is_loading() {
                div {
                    class: "laoding-box",
                    dangerous_inner_html: "{LOADING}"
                }
              }
            }
        }
    }
}
