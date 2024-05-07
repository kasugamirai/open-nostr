use nostr_sdk::prelude::*;
use crate::nostr::register::*;
use std::sync::Arc;
use tokio::task;
use tokio::sync::mpsc::Receiver;

pub enum WorkerCommand {
    Subscribe {
        filters: Vec<Filter>,
        handler: NotificationHandler,
    },
    // Add more commands as needed
    Start,
}

pub async fn client_worker(client: Client, client_key: &str, mut command_rx: Receiver<WorkerCommand>) {
    let client_arc = Arc::new(client);

    while let Some(command) = command_rx.recv().await {
        match command {
            WorkerCommand::Subscribe { filters, handler } => {
                let client_clone = client_arc.clone();
                let client_key_clone = client_key.to_string();
                task::spawn(async move {
                    let _ = SUB_REGISTER.add_subscription(&client_clone, &client_key_clone, None, filters, handler, None).await;
                });
            }
            WorkerCommand::Start => {
                let client_clone = client_arc.clone();
                task::spawn(async move {
                    SUB_REGISTER.handle_notifications(&client_clone).await.unwrap();
                });
            },
            // Handle other commands here
        }
    }
}