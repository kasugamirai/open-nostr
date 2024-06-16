use dioxus::prelude::*;

use crate::components::icons::*;
use crate::init::MODAL_MANAGER;
use crate::nostr::{get_metadata, get_newest_event, MultiClient};

use nostr_indexeddb::database::Order;
use nostr_sdk::{Event, Filter, JsonUtil, Kind, Metadata, PublicKey, ToBech32};
// kind 10002
use crate::utils::js;
use nostr_sdk::TagStandard;
use qrcode::render::svg;
use qrcode::QrCode;

#[derive(PartialEq, Clone, Props)]
pub struct AuthorProps {
    pubkey: String,
    relay_name: String,
}

#[component]
pub fn Author(props: AuthorProps) -> Element {
    let AuthorProps { pubkey: pubkey_str, relay_name } = props;
    let pubkey = PublicKey::from_hex(&pubkey_str.clone()).unwrap();
    let mut pubkey_npub = use_signal(|| pubkey.to_bech32().unwrap());
    let mut nickname = use_signal(|| "Nostr Account".to_string());

    let mut avatar =
        use_signal(|| "https://avatars.githubusercontent.com/u/1024025?v=4".to_string());
    let mut banner =
        use_signal(|| "".to_string());
    let multiclient = use_context::<Signal<MultiClient>>();

    let mut relays = use_signal(|| "".to_string());
    use_effect(use_reactive(&pubkey_str.clone(), move |new_pubkey| {
        let _pubkey = PublicKey::from_hex(&new_pubkey).unwrap();
        let _pubkey_npub = _pubkey.to_bech32().unwrap();
        pubkey_npub.set(_pubkey_npub);
    }));
    use_effect(use_reactive(
        (&pubkey, &relay_name, &pubkey_npub()),
        move |(pubkey, relay_name,_)| {
          tracing::info!("pubkey: {:?}", pubkey);
            spawn(async move {
                let multiclient = multiclient();
                if let Some(client) = multiclient.get_client(&relay_name).await {
                    let client = client.client();
                    {
                        let filter = Filter::new().author(pubkey).kind(Kind::RelayList);
                        let event_result1 = client.get_events_of(vec![filter], None).await.unwrap();
                        for event in event_result1 {
                            let mut res = String::new();
                            for tag in event.tags() {
                                if let Some(url) = tag.content() {
                                    res.push_str(url);
                                    res.push_str(",");
                                } else {
                                    tracing::info!("Error");
                                }
                            }
                            relays.set(res);
                        }
                    }
                    {
                        let filter = Filter::new().author(pubkey).kind(Kind::Metadata);
                        let event_result = client
                            .database()
                            .query(vec![filter], Order::Desc)
                            .await
                            .unwrap();
                        if let Some(event) = get_newest_event(&event_result) {
                            let metadata = Metadata::from_json(&event.content).unwrap();
                            nickname.set(metadata.display_name.unwrap_or_else(|| {
                                metadata.name.unwrap_or("Nostr Account".to_string())
                            }));
                            avatar.set(metadata.picture.unwrap_or_else(|| {
                                "https://avatars.githubusercontent.com/u/1024025?v=4".to_string()
                            }));
                            banner.set(metadata.banner.unwrap_or_else(|| {
                                "".to_string()
                            }));
                        } else {
                            match get_metadata(&client, &pubkey, None).await {
                                Ok(metadata) => {
                                    nickname.set(metadata.display_name.unwrap_or_else(|| {
                                        metadata.name.unwrap_or("Nostr Account".to_string())
                                    }));
                                    avatar.set(metadata.picture.unwrap_or_else(|| {
                                        "https://avatars.githubusercontent.com/u/1024025?v=4"
                                            .to_string()
                                    }));
                                }
                                Err(e) => {
                                    tracing::error!("get_metadata error: {:?}", e);
                                }
                            }
                        }
                    }

                }
            });
        },
    ));

    let resss = relays.to_string();
    let relaysss: Vec<&str> = resss.trim_end_matches(',').split(',').collect();

    let handle_qrcode = move || {
        let data = pubkey_npub.to_string();
        // 创建二维码
        let code = QrCode::new(data).unwrap();

        // 渲染为SVG字符串
        let image = code.render::<svg::Color>().min_dimensions(200, 200).build();

        MODAL_MANAGER.write().add_modal(
            rsx! {
                div {
                    class: "qrcode-modal",
                    div {
                        class: "qrcode-modal-content",
                        div {
                            class: "qrcode-modal-header",
                            h1 {
                                class: "qrcode-modal-title",
                                "Scan the QR code to follow"
                            }
                            div {
                                class: "qrcode-modal-close btn-circle btn-circle-false",
                                onclick: move |_| {
                                    MODAL_MANAGER.write().close_modal("qrcode");
                                },
                                dangerous_inner_html: "{FALSE}",
                            }
                        }
                        div {
                            class: "qrcode-modal-body",
                            div {
                                class: "qrcode-modal-qrcode",
                                dangerous_inner_html: image,
                            }
                        }
                    }
                }
            },
            "qrcode".to_string(),
        );
        MODAL_MANAGER.write().open_modal("qrcode");
    };
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
                src:"{banner}"
              }
              div{
                class:"information-of-the-author",
                div{
                  class:"author-head",
                  img{
                    class:"author-head-portrait",
                    src:"{avatar}"
                  }
                  div{
                    class:"author-name ml-87 lh-40 text-overflow",
                    "{nickname}"
                  }
                }
                div{
                  class:"author-intro two-line-truncate ml-13",
                  "{pubkey_npub}"
                }
                span{
                  class:"copy-position",
                  onclick: move |_| {
                    let data = js::export_to_clipboard(pubkey_npub.to_string());
                  },
                  dangerous_inner_html: "{COPY_ALL}",
                }
                span{
                  class:"code-position",
                  onclick: move |_| {
                    handle_qrcode();
                  },
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
          for i in 0..=relaysss.len()-1 {
            div{
              class:"seen-on-relays-li display-flex-box mb-6",
              // img{
              //   class:"relays-img",
              //   src:"https://image.nostr.build/13054263e6f3c302503a0c1a807f596a1a87ac24e1e9f4f3dfafc51d5e9f992d.jpg"
              // }
              div{
                class:"relays-text font-size-16 ml-9 text-overflow",
                "{relaysss[i]}"
              }
            }
          }
        }
    }
}
