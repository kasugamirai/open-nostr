use dioxus::prelude::*;

use crate::{
    components::{icons::*, Button, Dropdown},
    views::{
        Bookmark, Channel, CustomSubscription, Group, Home, Message, Profile, Relay, Search,
        Settings, Test, Topic,
    },
    CustomSub,
};

struct RouterItem {
    to: Route,
    icon: String,
    name: String,
}

#[component]
fn Layout() -> Element {
    let subs = use_context::<Signal<Vec<CustomSub>>>();
    let mut cur = use_context::<Signal<usize>>();
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
            icon: PROFILE.to_string(),
            name: "Home".to_string(),
        },
        RouterItem {
            to: Route::Profile {},
            icon: PROFILE.to_string(),
            name: "Profile".to_string(),
        },
        RouterItem {
            to: Route::Search {},
            icon: SEARCH.to_string(),
            name: "Search".to_string(),
        },
        RouterItem {
            to: Route::Relay {},
            icon: RELAY.to_string(),
            name: "Relay".to_string(),
        },
        RouterItem {
            to: Route::Message {},
            icon: MESSAGE.to_string(),
            name: "Message".to_string(),
        },
        RouterItem {
            to: Route::Channel {},
            icon: SIGNAL.to_string(),
            name: "Channel".to_string(),
        },
        RouterItem {
            to: Route::Group {},
            icon: CHAT.to_string(),
            name: "Group".to_string(),
        },
        RouterItem {
            to: Route::Bookmark {},
            icon: STAR.to_string(),
            name: "Bookmark".to_string(),
        },
        RouterItem {
            to: Route::Settings {},
            icon: SETTING.to_string(),
            name: "Settings".to_string(),
        },
        // RouterItem {
        //     to: Route::Test { id: 1 },
        //     icon: PROFILE.to_string(),
        //     name: "Test".to_string(),
        // },
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
    rsx! {
        div{
            class: "layout-left",
            div {
                class: "menu",
                div {
                    class: "user",
                    Dropdown {
                        trigger: rsx! {
                            div {
                                class: "user-trigger",
                                div {
                                    dangerous_inner_html: "{SVG}"
                                }
                                span {
                                    "Username"
                                }
                            }
                        },
                        children: rsx! {
                            div {
                                class: "user-content",
                                "Content"
                            }
                        }
                    }
                }
                div {
                    class: "nav",
                    for router in routers.iter() {
                        Link {
                            active_class: "active",
                            class: "nav-item",
                            to: router.to.clone(),
                            div {
                                class: "nav-item-content",
                                span {
                                    dangerous_inner_html: "{router.icon}",
                                }
                                span {
                                    "{router.name}"
                                }
                            }
                        }
                    }
                }
                div {
                    class: "subscriptions",
                    for (i, sub) in subs.read().iter().enumerate() {
                        button {
                            class: format!("subscriptions-btn{}", if i == cur() { " active" } else { "" }),
                            onclick: move |_| cur.set(i),
                            "#{sub.name}",
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
        div {
            class: "layout-right",
            // CustomSubscription {}
        }
    }
}

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},

    #[route("/topic/:id")]
    Topic { id: String },

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
