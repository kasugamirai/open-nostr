use std::time::Duration;

use dioxus::prelude::*;
use nostr_sdk::prelude::*;

use crate::components::{Button, Post, PostData};

#[component]
pub fn Home() -> Element {
    let mut post_datas = use_signal(Vec::<PostData>::new);
    let get_events = move || {
        spawn(async move {
            println!("pk: npub156jqdy53ceqahp25jjh5ky6u959ldk3h2eu5h4clhmu8vdhucvnqrrt7yt");
            // pk: npub156jqdy53ceqahp25jjh5ky6u959ldk3h2eu5h4clhmu8vdhucvnqrrt7yt
            // sk: nsec1dmvtj7uldpeethalp2ttwscy32jx36hr9jslskwdqreh2yk70anqhasx64
            let pk = "nsec1dmvtj7uldpeethalp2ttwscy32jx36hr9jslskwdqreh2yk70anqhasx64";
            // pk to hex
            let my_keys = Keys::parse(pk).unwrap();

            let client = Client::new(&my_keys);

            client.add_relay("wss://nostr.pjv.me").await.unwrap();

            client.connect().await;

            let filter: Filter = Filter::new().limit(10).kind(Kind::TextNote).author(
                PublicKey::parse("npub1pjvcwasj9ydasx9nmkf09pftsg640vm5fs7tzprssew8544yj2ds6e0h42")
                    .unwrap(),
            );

            let events = client
                .get_events_of(vec![filter], Some(Duration::from_secs(30)))
                .await
                .unwrap();

            for event in events {
                let post_data = PostData {
                    id: event.id().to_hex(),
                    author: event.author().to_hex(),
                    created_at: event.created_at().as_u64(),
                    kind: "".to_string(),
                    tags: vec![],
                    content: event.content.to_string(),
                };
                post_datas.push(post_data);
            }
        });
    };

    let handle_get_events = move |_| {
        get_events();
    };

    rsx! {
        ul {
            style: "display: flex; flex-direction: column; gap: 10px;",
            for i in post_datas.iter() {
                Post { data: i.clone() }
            }
        }
        Button { on_click: handle_get_events, "Get Events" }
    }
}
