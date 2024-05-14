use dioxus::prelude::*;
use crate::store::subscription::CustomSub;
struct UserItem {
    avatar: &'static str,
    username: &'static str,
}

use crate::components::icons::*;
use crate::router::*;
use crate::components::Button;
#[component]
pub fn Layout() -> Element {
    let subs = use_context::<Signal<Vec<CustomSub>>>();
    let mut theme = use_context::<Signal<String>>();
    let toggle_theme = move |_| {
        if theme() == "light" {
            theme.set("dark".to_string());
        } else {
            theme.set("light".to_string());
        }
    };

    let users = vec![
        UserItem{
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
                        "New Note"
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
                                to: Route::NoteList { name: sub.name.clone() },
                                "#{sub.name}"
                            }
                        }
                    }
                }
                Button { on_click: toggle_theme, "{theme}" }
            }
        }
        div {
            class: "layout-main",
            Outlet::<Route> {}
        }
    }
}