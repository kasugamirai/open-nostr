use nostr_sdk::prelude::*;
use crate::nostr::register::*;

pub enum WorkerCommand {
    Subscribe {
        filters: Vec<Filter>,
        handler: NotificationHandler,
    },
    // Add more commands as needed
}

pub async fn client_worker(client: Client, client_key: &str, mut command_rx: tokio::sync::mpsc::Receiver<WorkerCommand>) {
    while let Some(command) = command_rx.recv().await {
        match command {
            WorkerCommand::Subscribe { filters, handler } => {
                // Handle the subscription using the handler
                let _ = SUB_REGISTER.add_subscription(&client, client_key, None, filters, handler, None).await;
                SUB_REGISTER.handle_notifications(&client).await.unwrap();
            }
            // Handle other commands here
        }
    }
}
