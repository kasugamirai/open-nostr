// use nostr_sdk::prelude::*;
// use capybastr::nostr::register::*;
// use std::sync::Arc;
// use std::error::Error as StdError;
// use std::io::{self, Read};

// const BECH32_SK: &str = "nsec1przf9ascez0rty5yyflh5lk6hfu2pc0e2tyh8ed97esf25gg7zrsneae83";

// async fn handler_text_note(notification: RelayPoolNotification) -> Result<bool, Box<dyn StdError>> {
//     if let RelayPoolNotification::Message {
//         message: RelayMessage::Event { event, .. },
//         ..
//     } = notification
//     {
//         println!("TextNote: {:?}", event);
//     }
//     Ok(false)
// }

// async fn handler_text_note2(notification: RelayPoolNotification) -> Result<bool, Box<dyn StdError>> {
//     if let RelayPoolNotification::Message {
//         message: RelayMessage::Event { event, .. },
//         ..
//     } = notification
//     {
//         println!("TextNote again !!!: {:?}", event);
//     }
//     Ok(false)
// }

// async fn handler_repost(notification: RelayPoolNotification) -> Result<bool, Box<dyn StdError>> {
//     if let RelayPoolNotification::Message {
//         message: RelayMessage::Event { event, .. },
//         ..
//     } = notification
//     {
//         println!("Repost: {:?}", event);
//     }
//     Ok(false)
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     tracing_subscriber::fmt::init();

//     let secret_key = SecretKey::from_bech32(BECH32_SK)?;
//     let keys = Keys::new(secret_key);
//     let public_key = keys.public_key();

//     let client_for_text_note = ClientBuilder::new().opts(Options::new().wait_for_send(false)).build();
//     client_for_text_note.add_relay("wss://relay.damus.io").await?;
//     client_for_text_note.connect().await;

//     let client_for_repost = ClientBuilder::new().opts(Options::new().wait_for_send(false)).build();
//     client_for_repost.add_relay("wss://relay.damus.io").await?;
//     client_for_repost.connect().await;

//     let filter_text_note = Filter::new()
//         .author(public_key)
//         .kind(Kind::TextNote)
//         .since(Timestamp::now());

//     // let filter_repost = Filter::new()
//     //     .author(public_key)
//     //     .kind(Kind::Repost)
//     //     .since(Timestamp::now());

//     SUB_REGISTER.add_subscription(
//         &client_for_text_note,
//         SubscriptionId::new("text-note"),
//         vec![filter_text_note.clone()],
//         Arc::new(|notification| Box::pin(handler_text_note(notification))),
//         None,
//     ).await?;

//     SUB_REGISTER.add_subscription(
//         &client_for_text_note,
//         SubscriptionId::new("text-note2"),
//         vec![filter_text_note],
//         Arc::new(|notification| Box::pin(handler_text_note2(notification))),
//         None,
//     ).await?;

//     let handle1 = tokio::spawn(async move {
//         SUB_REGISTER.handle_notifications(&client_for_text_note).await.unwrap();
//     });

//     //stop a subscription by id

//     SUB_REGISTER.remove_subscription(&SubscriptionId::new("text-note")).await;

//     // Wait for user input to exit
//     println!("Press any key to exit...");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();

//     Ok(())
// }