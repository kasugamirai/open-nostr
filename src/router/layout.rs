use crate::store::subscription::CustomSub;
use dioxus::prelude::*;
use wasm_bindgen::closure;
struct UserItem {
    avatar: &'static str,
    username: &'static str,
}
use crate::{
  utils::format::{splite_by_replys},
};
// use crate::views::

use crate::components::icons::*;
use crate::components::Button;
use crate::components::Message;
use crate::router::*;

#[component]
pub fn Layout() -> Element {
    let subs = use_context::<Signal<Vec<CustomSub>>>();
    let mut edit = use_signal(|| false);
    let mut theme = use_context::<Signal<String>>();
    let toggle_theme = move |_| {
        if theme() == "light" {
            theme.set("dark".to_string());
        } else {
            theme.set("light".to_string());
        }
    };
    let mut messageContent = use_signal(||String::from(""));

    let users = [UserItem{
            avatar: "https://img.alicdn.com/imgextra/i2/O1CN01fI8HqB20dQg3rqybI_!!6000000006872-2-tps-2880-120.png",
            username: "James LisaLisaLisaLisaLisaLisaLisa"
        },
        UserItem{
            avatar: "https://img.alicdn.com/imgextra/i2/O1CN01fI8HqB20dQg3rqybI_!!6000000006872-2-tps-2880-120.png",
            username: "Tom"
        },
        UserItem{
            avatar: "https://img.alicdn.com/imgextra/i2/O1CN01fI8HqB20dQg3rqybI_!!6000000006872-2-tps-2880-120.png",
            username: "Lisa"
        },
    ];

    let mut contentText = use_signal(|| String::from(""));

    let mut show = use_signal(|| false);

    rsx! {
        div{
            class: "layout-left",
            div {
                class: "menu",
                h1 {
                    class: "pro-title text-ellipsis text-ellipsis-1",
                    "CapyBastr"
                },
                div {
                    class: "user-trigger user-trigger-{show} account-wrapper",
                    div{
                        class: "user-item flex items-center justify-between overflow-hidden cursor-pointer",
                        onclick: move |_| {
                            show.set(!show());
                        },
                        div {
                            class: "user-avatar flex items-center",
                            img{
                                class: "user-avatar-img",
                                src: "https://img.alicdn.com/imgextra/i2/O1CN01fI8HqB20dQg3rqybI_!!6000000006872-2-tps-2880-120.png",
                            }
                            h1{
                                class: "user-name text-overflow",
                                "User Test "
                            }
                        }
                        if !show(){
                            span{
                                dangerous_inner_html: "{DOWN}",
                            }
                        }else{
                            span{
                                dangerous_inner_html:  "{ARROW_UP}",
                            }
                        }
                    }
                    if show() {
                        div{
                            class: "user-trigger-item flex items-center justify-between",
                            div{
                                class: "flex items-center",
                                button {
                                    class: "user-trigger-item-button log-button",
                                    "Logout"
                                },
                                button {
                                    class: "user-trigger-item-button pro-button",
                                    "Profile"
                                }
                            }
                            div{
                                class: "copy-btn cursor-pointer",
                                dangerous_inner_html: "{COPY_ALL}",
                            }
                        }
                        for user in users.iter() {
                            div{
                                class: "user-trigger-item flex items-center justify-between",
                                div {
                                    class: "user-info flex items-center",
                                    img{
                                        class: "user-avatar-img",
                                        src: "{user.avatar}",
                                    },
                                    h1{
                                        class: "user-name text-overflow",
                                        "{user.username}"
                                    }
                                }
                                div {
                                    class: "copy-btn cursor-pointer",
                                    dangerous_inner_html: "{COPY_ALL}",
                                }
                            }
                        }
                    }
                }
                div {
                    class: "nav",
                    for router in ROUTERS.iter() {
                        Link {
                            active_class: "active",
                            class: "nav-item block",
                            to: router.to.clone(),
                            div {
                                class: "nav-item-content flex items-center",
                                span {
                                    dangerous_inner_html: "{router.icon}",
                                }
                                span {
                                    class: "nav-item-text",
                                    "{router.name}"
                                }
                            }
                        }
                    }
                    div {
                        class: "nav-item-content add-note-btn cursor-pointer text-center",
                    onclick: move |_| {
                          edit.set(!edit());
                        },
                        "New Note"
                    }
                    div{
                      class:"show-{edit}",
                      div{
                        class:"relay-edit-mask",
                        onclick: move |_| {
                            edit.set(false);
                        },
                      }
                      div{
                        class:"note-pop-up",
                        textarea{
                          class:"text-area-style",
                          value:"{contentText}",
                          onchange: move |event| {
                            contentText.set(event.value());
                          }
  
                        }
                        span{
                          class:"img-svg-style",
                          dangerous_inner_html: "{IMGICON}",
                        }
                        div{
                          class:"preview",
                          div{
                            "Preview:"
                            div{
                              class:"previewContent",
                              // NoteEdit {
                              //   content: contentText.read().clone()
                              // }
                              div {
                                class: "event-note",
                                for i in splite_by_replys(&contentText()) {
                                  if i.starts_with("nostr:") {
                                      div {
                                          class: "quote",
                                          div {
                                              class: "title",
                                              "Qt:"
                                          }
                                          div {
                                              class: "note",
                                              EventLess {content: i }
                                          }
                                      }
                                  } else {
                                      div {
                                          class: "content",
                                          dangerous_inner_html: "{i}"
                                      }
                                  }
                                }
                              }
                            }
                          }
                        }
                        button{
                          class:"noteButton sendStyle",
                          "Send"
                        }
                        button{
                          class:"noteButton cancelStyle",
                          onclick: move |_| {
                            edit.set(false);
                          },
                          "Cancel"
                        }
                      }
                  
                    }
                }
                div {
                    class: "subscriptions",
                    h1{
                        style: "color:var(--txt-1)",
                        "Subscriptions:"
                    }
                    div{
                        class: "subscriptions-item",
                        for (_i, sub) in subs.read().iter().enumerate() {
                            Link {
                                active_class: "active",
                                class: "nav-item",
                                to: Route::NoteList { name: urlencoding::encode(&sub.name.clone()).to_string() },
                                "{sub.name}"
                            }
                        }
                    }
                }
                // div{
                //   h1{
                //     style: "color:var(--txt-1)",
                //     onclick: move |event| {
                //       messageContent.set("Received 10 New Events !!".to_string());
                //     },
                //     "获取新消息"
                //   }
                // }
                Button { on_click: toggle_theme, "{theme}" }
            }
            Message{content:"{messageContent.clone()}"}
        }
        div {
            class: "layout-main",
            Outlet::<Route> {}
        }
    }
}
#[component]
fn EventLess(content: String) -> Element {
    rsx! {
        div {
            class: "event-less",
            div {
              class: "post-avatar flex items-center",
              img {
                  class: "square-40 radius-20 mr-12",
                  src: "https://avatars.githubusercontent.com/u/1024025?v=4",
                  alt: "avatar",
              }
              div {
                  class: "profile flex flex-col",
                    span {
                        class: "nickname font-size-16 txt-1",
                        "dioxus"
                    }
                    span {
                        class: "created txt-3 font-size-12",
                      "123"
                    }
                }
            }
            div {
                class: "text",
                dangerous_inner_html: "{content}",
            }
        }
    }
}
