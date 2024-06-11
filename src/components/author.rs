use dioxus::prelude::*;

use crate::components::icons::*;

#[component]
pub fn Author() -> Element {
    rsx! {
        // Custom Sub component
        div {
            class: "author-wrapper author-not-style display-flex-box relative flex-col mb-15",
            div {
                class: "author-header mb-6",
                h1 {
                    class: "title author-title font-raleway-800 font-size-20",
                    "Author"
                }
            }
            div{
              class:"author-content",
              img{
                class:"author-background",
                src:"https://image.nostr.build/13054263e6f3c302503a0c1a807f596a1a87ac24e1e9f4f3dfafc51d5e9f992d.jpg"
              }
              div{
                class:"information-of-the-author",
                div{
                  class:"author-head",
                  img{
                    class:"author-head-portrait",
                    src:"https://image.nostr.build/13054263e6f3c302503a0c1a807f596a1a87ac24e1e9f4f3dfafc51d5e9f992d.jpg"
                  }
                  div{
                    class:"author-name ml-87 lh-40 text-overflow",
                    "Sarah Smith"
                  }
                }
                div{
                  class:"author-intro two-line-truncate ml-13",
                  "dsadasdasddasdasdadfsdvnfjknvkjnjasn cjksack cnsjndjsndjsanjan"
                }
                span{
                  class:"copy-position",
                  dangerous_inner_html: "{COPY_ALL}",
                }
                span{
                  class:"code-position",
                  dangerous_inner_html: "{QRCODE}",
                }

              }
            }

        }
        div {
          class: "relays-wrapper author-not-style display-flex-box relative flex-col mb-15",
          div {
              class: "author-header mb-13",
              h1 {
                  class: "title author-title font-raleway-800 font-size-20",
                  "Seen on Relays"
              }
          }
          div{
            class:"seen-on-relays-li display-flex-box mb-6",
            img{
              class:"relays-img",
              src:"https://image.nostr.build/13054263e6f3c302503a0c1a807f596a1a87ac24e1e9f4f3dfafc51d5e9f992d.jpg"
            }
            div{
              class:"relays-text font-size-16 ml-9 text-overflow",
              "wss://relay.damous.io"
            }
          }
          div{
            class:"seen-on-relays-li display-flex-box mb-6",
            img{
              class:"relays-img",
              src:"https://image.nostr.build/13054263e6f3c302503a0c1a807f596a1a87ac24e1e9f4f3dfafc51d5e9f992d.jpg"
            }
            div{
              class:"relays-text font-size-16 ml-9 text-overflow",
              "wss://relay.da"
            }
          }
        }
    }
}
