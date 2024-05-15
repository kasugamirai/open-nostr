use dioxus::prelude::*;

use crate::components::icons::*;
use crate::views::{
    Bookmark, Channel, Group, Home, Message, NoteDetail, NoteList, Profile, Relay, Search,
    Settings, Test,
};
mod layout;
mod page_not_found;
pub use layout::Layout;
pub use page_not_found::PageNotFound;

pub struct RouterItem {
    to: Route,
    icon: &'static str,
    name: &'static str,
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

    #[route("/note/:sub/:id")]
    NoteDetail { sub: String, id: String },

    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

pub static ROUTERS: &[RouterItem] = &[
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
