
use icons::*;


#[component]
pub fn NoteEdit(content: String) -> Element {
    

  let multiclient = use_context::<Signal<MultiClient>>(); //
  let clients = multiclient();
  let client = clients.get("Car").unwrap();
   

    let notetext = use_signal(|| content);
    let _future = use_resource(move || async move {

        // tracing::error!("error:");

        let re = Regex::new(r"(nostr:note[a-zA-Z0-9]{64})").unwrap();

        let data = &notetext();

        let mut parts = Vec::new();
        let mut last_end = 0;

        for mat in re.find_iter(data) {
            if mat.start() > last_end {
                parts.push(&data[last_end..mat.start()]);
            }
            parts.push(mat.as_str());
            last_end = mat.end();
        }

        if last_end < data.len() {
            parts.push(&data[last_end..]);
        }

        let mut elements = vec![];

        let mut elements = vec![];
        for i in parts {
            if i.starts_with("nostr:note") {
                let id = i.strip_prefix("nostr:note").unwrap();

                let filter = Filter::new().id(EventId::from_hex(id).unwrap());
                let events = client.get_events_of(vec![filter], None).await.unwrap();

                if events.len() > 0 {
                    let pk = events[0].author();
                    let content = events[0].content.to_string();
                    let timestamp = events[0].created_at.as_u64();

                    let mut nickname = "".to_string();
                    let mut avatar = "".to_string();

                    match get_metadata(&client, &pk, None).await {
                        Ok(metadata) => {
                            nickname = metadata.name.unwrap_or("Default".to_string());
                            avatar = match metadata.picture {
                                Some(picture) => {
                                    if picture.is_empty() {
                                        "https://avatars.githubusercontent.com/u/1024025?v=4"
                                            .to_string()
                                    } else {
                                        picture
                                    }
                                }
                                None => "https://avatars.githubusercontent.com/u/1024025?v=4"
                                    .to_string(),
                            }
                        }
                        Err(_) => {
                            tracing::info!("metadata not found");
                        }
                    }

                    elements.push(rsx! {
                        div {
                            class: "quote display-flex-box items-center",
                            div {
                                class:"font-weight-bold quote-box-style",
                                "Qt:"
                            }
                            div {
                              class:"display-flex-box qt-box",
                                div {
                                    class: "qt-img-box",
                                    img {
                                        class: "square-40 radius-20 mr-12",
                                        src: avatar,
                                        alt: "avatar",
                                    }
                                    div {
                                        class: "profile flex flex-col",
                                        span {
                                            class: "nickname font-size-16 txt-1",
                                            {nickname}
                                        }
                                        span {
                                            class: "created txt-3 font-size-12",
                                            {format_create_at(timestamp)}
                                        }
                                    }
                                }
                                div {
                                    class:"qt-content-box font-size-14",
                                    dangerous_inner_html: "{content}"
                                }
                            }
                        }
                    });
                }
            } else {
                elements.push(rsx! {
                    div {
                        class: "text pl-52",
                        dangerous_inner_html: "{format_content(i)}"
                    }
                });
            }
        }

        element.set(rsx! {
            for element in elements {
                {element}
            }
        });
        // elements
      });

    // let
    rsx! {
        div {
            class: "note-content font-size-16 word-wrap lh-26",
            // dangerous_inner_html: "{format_content(&props.data.content)}",
            // for el in _future.value().into_iter() {
            //   {el}
            // }
            {element}
        }
    }
}
