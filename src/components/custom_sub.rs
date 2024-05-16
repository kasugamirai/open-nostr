use dioxus::prelude::*;

use crate::components::icons::*;

#[component]
pub fn CustomSub() -> Element {
    rsx! {
        // Custom Sub component
        div {
            class: "custom-sub-wrapper NotStyle",
            div {
                class: "custom-sub-header",
                h1 {
                    class: "title custom-sub-title font-Raleway-800 font-size-20",
                    "Notification"
                }
                button {
                    class: "icon",
                    dangerous_inner_html: "{MORE}"
                }
            }
            div{
              class:"dayBox",
              div{
                class:"border000"
              }
              span{
                "3 days ago"
              }
            }
            div{
              class:"NotificatioBox",
              div {
                class: "header",
                div {
                    class: "user",
                    div {
                      class: "avatar avatarLeft",
                        img {
                            class: "image radius-20 mr-12",
                            src: "https://avatars.githubusercontent.com/u/1024025?v=4"
                        }
                    }
                    div {
                      class: "profile",
                      span {
                          class: "nickname",
                          "Wendy"
                      }
                      span {
                          class: "created",
                          "17 hours ago"
                      }
                  }
                }
                span{
                  dangerous_inner_html: "{ADD}"
                }
              }
              div{
                class:"content",
                "😭"
              }
              div{
                class:"NotificatioBox contBox",
                div {
                  class: "header",
                  div {
                      class: "user",
                      div {
                        class: "avatar avatarLeft",
                          img {
                              class: "image radius-20 mr-12",
                              src: "https://avatars.githubusercontent.com/u/1024025?v=4"
                          }
                      }
                      div {
                        class: "profile",
                        span {
                            class: "nickname",
                            "Annie Wang"
                        }
                        span {
                            class: "created",
                            "18 Aug 2023"
                        }
                    }
                  }
                }
                div{
                  class:"content",
                  "If there are conflicts between the branches, Git willpause the merge and ask you to resolve them. Conflictsoccur when changes in the two branc... show more"
                }

              }
            }
        }
    }
}
