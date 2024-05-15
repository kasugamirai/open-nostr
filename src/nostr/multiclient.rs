use crate::store::AccountType;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MultiClient {
    clients: HashMap<String, nostr_sdk::Client>,
}

impl Default for MultiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiClient {
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

    pub fn apply_account_all(&self, _account: AccountType) {

        //todo

        // match account {
        //     AccountType::NotLoggedIn => None,
        //     AccountType::Local(sk) => todo!(),
        //     AccountType::Pub(_) => todo!(),
        // };

        // for client in self.clients.values() {
        //     //todo: apply account to client
        // }
    }
}
