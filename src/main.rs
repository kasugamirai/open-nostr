#![allow(non_snake_case)]

mod components;
mod nostr;
mod router;
mod state;
mod utils;
mod views;

use dioxus::prelude::*;
use tracing::Level;

use router::Route;

use crate::state::subscription::CustomSub;

fn main() {
    // Init debug
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    log::info!("Starting Dioxus");
    launch(App);
}

fn App() -> Element {
    let state = use_context_provider(|| Signal::new(String::from("light")));
    let _custom_sub_global = use_context_provider(|| Signal::new(CustomSub::default()));

    rsx! {
        div {
            id: "app",
            class: "{state}",
            Router::<Route> {}
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use nostr_sdk::{Client, Filter, Keys};

    #[tokio::test]
    async fn test() {
        let pk = "nsec1dmvtj7uldpeethalp2ttwscy32jx36hr9jslskwdqreh2yk70anqhasx64";
        // pk to hex
        let my_keys = Keys::parse(pk).unwrap();

        let client = Client::new(&my_keys);

        // wss://relayable.org,wss://nos.lol,wss://offchain.pub,wss://nostr-pub.wellorder.net,wss://relay.damus.io,wss://relay.snort.social,wss://freerelay.xyz,wss://relay.current.fyi,wss://nostr.zoel.network,wss://relay.nostrcn.com,wss://relay.proxymana.net,wss://crayon.nostr-demo.relayaas.com,wss://bitcoiner.social,wss://relay.beta.fogtype.com,wss://tictac.nostr1.com,wss://blastr.f7z.xyz,wss://relay.primal.net,wss://th1.nostr.earnkrub.xyz,wss://nostr.zoel.network,wss://relay.nostrcn.com,wss://relay.proxymana.net,wss://crayon.nostr-demo.relayaas.com,wss://bitcoiner.social"
        let relays = vec![
            String::from("wss://btc.klendazu.com"),
            // String::from("wss://relayable.org"),
            // String::from("wss://nos.lol"),
            // String::from("wss://offchain.pub"),
            // String::from("wss://nostr-pub.wellorder.net"),
            // String::from("wss://relay.damus.io"),
            // String::from("wss://relay.snort.social"),
            // String::from("wss://freerelay.xyz"),
            // String::from("wss://relay.current.fyi"),
            // String::from("wss://nostr.zoel.network"),
            // String::from("wss://relay.nostrcn.com"),
            // String::from("wss://relay.proxymana.net"),
            // String::from("wss://crayon.nostr-demo.relayaas.com"),
            // String::from("wss://bitcoiner.social"),
            // String::from("wss://relay.beta.fogtype.com"),
        ];

        for i in relays {
            client.add_relay(i.clone().as_str()).await.unwrap();
        }

        client.connect().await;

        let mut filter = Filter::new();
        filter = filter.hashtags(vec![String::from("dog")]);

        let events = client
            .get_events_of(vec![filter], Some(Duration::from_secs(300)))
            .await
            .unwrap();

        println!("Got {} events", events.len());
        println!("Events: {events:?}");
    }
}
