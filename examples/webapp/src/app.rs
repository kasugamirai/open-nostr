use std::vec;

use nostr_indexeddb::database::{NostrDatabase, Order};
use nostr_indexeddb::nostr::prelude::*;
use nostr_indexeddb::WebDatabase;
use nostr_sdk::client::Client;
use nostr_sdk::RelayPoolNotification;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    spawn_local(async {
        let key = Keys::generate();
        let private_key = key.secret_key().unwrap();
        let public_key = key.public_key();

        console::log_1(&format!("Pubkey A: {}", public_key).into());
        console::log_1(&format!("privkey A: {}", private_key).into());

        let database = WebDatabase::open("nostr-sdk-indexeddb-test").await.unwrap();

        let metadata = Metadata::new().name("Name");
        let event = EventBuilder::metadata(&metadata).to_event(&key).unwrap();
        database.save_event(&event).await.unwrap();

        let events = database
            .query(
                vec![Filter::new()
                    .kinds(vec![Kind::Metadata, Kind::Custom(123), Kind::TextNote])
                    .limit(20)
                    .author(key.public_key())],
                Order::Desc,
            )
            .await
            .unwrap();
        console::log_1(&format!("Events: {events:?}").into());
        console::log_1(&format!("Got {} events", events.len()).into());

        let client: Client = Client::new(&key);
        client.add_relay("wss://relay.damus.io").await.unwrap();
        client.connect().await;

        let filter = Filter::new().kinds(vec![Kind::Metadata]).limit(20);
        let sub_id = client.subscribe(vec![filter.clone()], None).await;
        client
            .subscribe_with_id(sub_id.clone(), vec![filter], None)
            .await;
        client
            .handle_notifications(|notification| async {
                if let RelayPoolNotification::Event {
                    subscription_id,
                    event,
                    ..
                } = notification
                {
                    // Check subscription ID
                    if subscription_id == sub_id {
                        database.save_event(&event).await.unwrap();
                    }

                    // Check kind
                    if event.kind() == Kind::EncryptedDirectMessage {
                        if let Ok(msg) =
                            nip04::decrypt(key.secret_key()?, event.author_ref(), event.content())
                        {
                            println!("DM: {msg}");
                            database.save_event(&event).await.unwrap();
                        } else {
                            println!("Impossible to decrypt direct message");
                        }
                    } else if event.kind() == Kind::TextNote {
                        println!("TextNote: {:?}", event);
                    } else {
                        println!("{:?}", event);
                        database.save_event(&event).await.unwrap();
                    }
                }
                Ok(false) // Set to true to exit from the loop
            })
            .await
            .unwrap();

        let filter = Filter::new().kinds(vec![Kind::Metadata]).limit(20);
        let events = database.query(vec![filter], Order::Asc).await.unwrap();
        console::log_1(&format!("Events: {events:?}").into());
        console::log_1(&format!("Got {} events", events.len()).into());

        //delete event
        let filter = Filter::new().kinds(vec![Kind::Metadata]).limit(10);
        database.delete(filter).await.unwrap();
    });

    html! {
        <main>
            <img class="logo" src="https://pbs.twimg.com/media/GJz0TM_aAAAX28k?format=jpg&name=4096x4096" alt="Yew logo" />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
