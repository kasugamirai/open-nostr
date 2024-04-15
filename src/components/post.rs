use dioxus::prelude::*;

use crate::{
    components::icons::*,
    utils::format::{format_content, format_create_at, format_public_key},
};

#[derive(PartialEq, Clone, Props)]
pub struct PostData {
    pub id: String,
    pub author: String,
    pub created_at: u64,
    pub kind: String,
    pub tags: Vec<String>,
    pub content: String,
}

#[derive(PartialEq, Clone, Props)]
pub struct PostProps {
    data: PostData,
}

#[component]
pub fn Post(props: PostProps) -> Element {
    rsx! {
        div {
            class: "com-post",
            div {
                class: "com-post-author",
                div {
                    class: "com-post-author-avatar",
                    img { src: "https://image.baidu.com/search/down?url=https://tvax3.sinaimg.cn//large/0072Vf1pgy1foxkd3ae0mj31hc0u0tsr.jpg" }
                }
                div {
                    class: "com-post-author-profile",
                    span {
                        class: "com-post-author-profile-name",
                        "{format_public_key(&props.data.author)}"
                    }
                    span {
                        class: "com-post-author-profile-created",
                        "{format_create_at(props.data.created_at)}"
                    }
                }
            }
            div {
                class: "com-post-content",
                dangerous_inner_html: "{format_content(&props.data.content)}",
            }
            div {
                class: "com-post-info",
                div {
                    class: "com-post-info-item com-post-info-reply",
                    span {
                        dangerous_inner_html: "{TURN_LEFT}",
                    }
                    span {
                        class: "com-post-info-item-data",
                        "5"
                    }
                }
                div {
                    class: "com-post-info-item com-post-info-share",
                    span {
                        dangerous_inner_html: "{TURN_RIGHT}",
                    }
                    span {
                        class: "com-post-info-item-data",
                        "2"
                    }
                }
                div {
                    class: "com-post-info-item com-post-info-comment",
                    span {
                        dangerous_inner_html: "{MARKS}",
                    }
                    span {
                        class: "com-post-info-item-data",
                        "2"
                    }
                }
                div {
                    class: "com-post-info-item com-post-info-reward",
                    span {
                        dangerous_inner_html: "{FLASH}",
                    }
                    span {
                        class: "com-post-info-item-data",
                        "40k"
                    }
                }
                div {
                    class: "com-post-info-item com-post-info-reply",
                    span {
                        dangerous_inner_html: "{ADD}",
                    }
                }
            }
        }
    }
}
