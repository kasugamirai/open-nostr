use dioxus::prelude::*;

use crate::components::icons::*;

#[component]
pub fn CustomSub() -> Element {
    rsx! {
        // Custom Sub component
        div {
            class: "custom-sub-wrapper not-style display-flex-box relative flex-col",
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
            div{
              class:"day-box ml-16 display-flex-box relative",
              div{
                class:"border000 mt-11"
              }
              span{
                class:"absoulte",
                "3 days ago"
              }
            }
            div{
              class:"notificatio-box",
              div {
                class: "header display-flex-box",
                div {
                    class: "user display-flex-box",
                    div {
                      class: "avatar avatar-left",
                        img {
                            class: "image radius-20 mr-12",
                            src: "https://avatars.githubusercontent.com/u/1024025?v=4"
                        }
                    }
                    div {
                      class: "profile display-flex-box",
                      span {
                          class: "nickname font-size-16",
                          "Wendy"
                      }
                      span {
                          class: "created font-size-14",
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
                class:"notificatio-box cont-box",
                div {
                  class: "header display-flex-box",
                  div {
                      class: "user display-flex-box",
                      div {
                        class: "avatar avatar-left",
                          img {
                              class: "image radius-20 mr-12",
                              src: "https://avatars.githubusercontent.com/u/1024025?v=4"
                          }
                      }
                      div {
                        class: "profile display-flex-box flex-col",
                        span {
                            class: "nickname font-size-16",
                            "Annie Wang"
                        }
                        span {
                            class: "created font-size-14",
                            "18 Aug 2023"
                        }
                    }
                  }
                }
                div{
                  class:"content font-size-14",
                  "If there are conflicts between the branches, Git willpause the merge and ask you to resolve them. Conflictsoccur when changes in the two branc... show more"
                }

              }
            }
        }
    }
}
