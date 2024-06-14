use std::collections::HashMap;

use dioxus::prelude::*;

use crate::components::ModalManager;
use crate::nostr::ReplyTreeManager;
use crate::store::subscription::CustomSub;
struct UserItem {
    avatar: &'static str,
    username: &'static str,
}
// use crate::views::
use crate::components::icons::*;
use crate::components::{Button, Message};
use crate::router::*;
use crate::utils::format::splite_by_replys;

#[component]
pub fn Layout() -> Element {
    let subs_map = use_context::<Signal<HashMap<String, CustomSub>>>();
    let mut edit = use_signal(|| false);
    let mut theme = use_context::<Signal<String>>();
    let toggle_theme = move |_| {
        if theme() == "light" {
            theme.set("dark".to_string());
        } else {
            theme.set("light".to_string());
        }
    };
    let messageContent = use_signal(|| String::from(""));
    // golbal replytree manager cache
    use_context_provider(|| Signal::new(ReplyTreeManager::new(200)));
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
    let path: Route = use_route();
    let mut modal_manager = use_context::<Signal<ModalManager>>();
    let mut contentText = use_signal(|| String::from(""));

    let mut show = use_signal(|| false);

    use_effect(use_reactive(&path, move |_| {
        modal_manager.write().destory_all_modals();
    }));
    rsx! {
        aside {
            class: "menu-bar",
            div {
                class: "menu",
                h1 {
                    class: "pro-title text-ellipsis text-ellipsis-1",
                    "CapyBastr"
                },
                div {
                    class: "user-trigger user-trigger-{show} account-wrapper",
                    div{
                        class: "user-item flex items-center justify-between overflow-hidden cursor-pointer pr-13",
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
                                class: "user-trigger-item flex items-center justify-between pl-0",
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
                              class:"preview-content",
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
                          class:"note-button send-style",
                          "Send"
                        }
                        button{
                          class:"note-button cancel-style",
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
                        class:"subscriptions-text mb-8 ",
                        "Subscriptions:"
                    }
                    div{
                        class: "subscriptions-item",
                        // for (_i, sub) in subs.read().iter().enumerate() {
                        //     Link {
                        //         active_class: "active",
                        //         class: "nav-item",
                        //         to: Route::Subscription { name: urlencoding::encode(&sub.name.clone()).to_string() },
                        //         "#{sub.name}"
                        //     }
                        // }
                        for (name, sub) in subs_map.read().iter() {
                            Link {
                                active_class: "active",
                                class: "nav-item",
                                to: Route::Subscription { name: name.clone() },
                                "#{sub.name}"
                            }
                        }
                        Link {
                            active_class: "active",
                            class: "nav-item new-subscription-btn",
                            to: Route::Subscription { name: "new".to_string() },
                            // href:"/newSubscription",
                            "New Subscription +"
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
        main {
            class: "content-feed",
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
              class: "post-avatar flex items-center min-width-120",
              img {
                  class: "square-40 radius-20 mr-12",
                  src: "https://avatars.githubusercontent.com/u/1024025?v=4",
                  alt: "avatar",
              }
              div {
                  class: "profile flex flex-col max-width-80",
                    span {
                        class: "nickname font-size-16 txt-1 text-overflow",
                        "dioxus"
                    }
                    span {
                        class: "created txt-3 font-size-12 text-overflow",
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
