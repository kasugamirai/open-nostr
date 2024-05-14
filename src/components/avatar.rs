use dioxus::prelude::*;

use crate::utils::format::format_create_at;

#[derive(PartialEq, Clone, Props)]
pub struct AvatarProps {
    // event: Event,
    pubkey: String,
    timestamp: u64,
    nickname: Option<String>,
    avatar: Option<String>,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    // if nickname is not provided, use the by id serch in the relay to get the nickname
    rsx! {
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
                    {format_create_at(props.timestamp)}
                }
            }
        }
    }
}
