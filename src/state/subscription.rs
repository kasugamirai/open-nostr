use nostr_sdk::Filter;
use serde::{Deserialize, Serialize};

/// CustomSub
///
/// name: name of the subscription
/// relay_set: relay set
/// filters: list of filters
///
/// # Example:
///
/// ```
/// let custom_sub = CustomSub::new();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomSub {
    pub name: String,
    pub relay_set: RelaySet,
    pub filters: Vec<FilterTemp>,
}

impl CustomSub {
    pub fn new() -> Self {
        Self {
            name: String::from(""),
            relay_set: RelaySet::new(),
            filters: vec![],
        }
    }

    pub fn default() -> Self {
        Self {
            name: String::from("#steakstr"),
            relay_set: RelaySet {
                name: String::from("Default"),
                relays: vec![
                    String::from("wss://nostr-pub.wellorder.net"),
                    String::from("wss://relay.damus.io"),
                ],
            },
            filters: vec![FilterTemp::HashTag(vec![String::from("steak")])],
        }
    }

    pub fn json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

/// RelaySet
///
/// name: name of the relay set
/// relays: list of relays
///
/// # Example:
///
/// ```
/// let relay_set = RelaySet::new();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelaySet {
    pub name: String,
    pub relays: Vec<String>,
}

impl RelaySet {
    pub fn new() -> Self {
        Self {
            name: String::from("Default"),
            relays: vec![],
        }
    }

    pub fn push(&mut self, v: String) {
        self.relays.push(v);
    }

    pub fn remove(&mut self, index: usize) -> String {
        self.relays.remove(index)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, String> {
        self.relays.iter()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterTemp {
    HashTag(Vec<String>),
    Aaccounts(Vec<u64>, Vec<Account>),
    Events(Vec<Vec<String>>),
    Customize(CustomFilter),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFilter {
    pub kinds: Vec<u64>,
    pub accounts: Vec<Account>,
    pub time: (u64, u64),
    pub limit: u64,
    pub tags: Vec<CustomTag>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Account {
    pub alt_name: String,
    pub npub: String,
}

impl Account {
    pub fn empty() -> Self {
        Self {
            alt_name: String::from(""),
            npub: String::from(""),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CustomTag {
    pub tag: String,
    pub value: String,
}

impl CustomTag {
    pub fn empty() -> Self {
        Self {
            tag: String::from(""),
            value: String::from(""),
        }
    }
}
