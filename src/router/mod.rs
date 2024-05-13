use dioxus::prelude::*;
use crate::store::subscription::CustomSub;

use crate::{
    // Dropdown
    components::{icons::*, Button},
    views::{
        Bookmark, Channel, Group, Home, Message, NoteDetail, NoteList, Profile, Relay, Search,
        Settings, Test,
    },
};

struct RouterItem {
    to: Route,
    icon: &'static str,
    name: &'static str,
}
struct UserItem {
    avatar: &'static str,
    username: &'static str,
}
#[component]
fn Layout() -> Element {
    let subs = use_context::<Signal<Vec<CustomSub>>>();
    let mut theme = use_context::<Signal<String>>();
    let toggle_theme = move |_| {
        if theme() == "light" {
            theme.set("dark".to_string());
        } else {
            theme.set("light".to_string());
        }
    };
    let routers = vec![
        RouterItem {
            to: Route::Home {},
            icon: HOME,
            name: "Home",
        },
        RouterItem {
            to: Route::Search {},
            icon: SEARCH,
            name: "Search",
        },
        RouterItem {
            to: Route::Relay {},
            icon: RELAY,
            name: "Relay",
        },
        RouterItem {
            to: Route::Message {},
            icon: MESSAGE,
            name: "Message",
        },
        RouterItem {
            to: Route::Channel {},
            icon: CHANNEL,
            name: "Channel",
        },
        RouterItem {
            to: Route::Group {},
            icon: GROUP,
            name: "Group",
        },
        RouterItem {
            to: Route::Bookmark {},
            icon: BOOKMARK,
            name: "Bookmark",
        },
        RouterItem {
            to: Route::Settings {},
            icon: SETTINGS,
            name: "Settings",
        },
        RouterItem {
            to: Route::Test { id: 1 },
            icon: TRUE,
            name: "Subscriptions:",
        },
    ];
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
    const SVG: &str = r#"
        <svg class="svg" t="1712897153675" class="icon" viewBox="0 0 1024 1024" version="1.1"
            xmlns="http://www.w3.org/2000/svg" p-id="5176" xmlns:xlink="http://www.w3.org/1999/xlink"
            width="40" height="40">
            <path
                d="M480 64A416.64 416.64 0 0 0 64 480 416.64 416.64 0 0 0 480 896 416.64 416.64 0 0 0 896 480 416.64 416.64 0 0 0 480 64z m0 64C674.752 128 832 285.248 832 480a351.36 351.36 0 0 1-81.024 225.024 289.408 289.408 0 0 0-162.944-171.776A159.36 159.36 0 0 0 640 416C640 328 568 256 480 256A160.448 160.448 0 0 0 320 416c0 46.272 20.224 88 52.032 117.248a289.024 289.024 0 0 0-162.752 171.776A350.208 350.208 0 0 1 128 480C128 285.248 285.248 128 480 128z m0 192C533.504 320 576 362.496 576 416S533.504 512 480 512A95.36 95.36 0 0 1 384 416C384 362.496 426.496 320 480 320z m0 256c108.8 0 198.016 77.248 218.752 179.776A350.528 350.528 0 0 1 480 832a349.248 349.248 0 0 1-218.496-76.224A222.72 222.72 0 0 1 480 576z"
                p-id="5177"
            >
            </path>
        </svg>
    "#;
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
                    for router in routers.iter() {
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

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},

    #[route("/profile")]
    Profile {},

    #[route("/search")]
    Search {},

    #[route("/relay")]
    Relay {},

    #[route("/message")]
    Message {},

    #[route("/channel")]
    Channel {},

    #[route("/group")]
    Group {},

    #[route("/bookmark")]
    Bookmark {},

    #[route("/settings")]
    Settings {},

    #[route("/test/:id")]
    Test { id: i32 },

    #[route("/subscription/:name")]
    NoteList { name: String },

    #[route("/note/:id")]
    NoteDetail { id: String },

    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
