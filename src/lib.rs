mod components;
mod init;
pub mod nostr;
mod router;
mod utils;
mod views;
pub mod store;

use std::collections::HashMap;

//pub use nostr::get_metadata;
pub use init::App;
pub use router::Route;
pub use store::{User, subscription::CustomSub};

#[derive(Debug, Clone)]
pub struct Clients {
    clients: HashMap<String, nostr_sdk::Client>,
}

impl Clients {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, client: nostr_sdk::Client) {
        self.clients.insert(name, client);
    }

    pub fn get(&self, name: &str) -> Option<&nostr_sdk::Client> {
        self.clients.get(name)
    }
}
