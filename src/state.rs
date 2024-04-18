pub mod subscription;

use std::collections::HashSet;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{from_str, to_string};

#[derive(Clone, Serialize, Deserialize)]
pub struct CustomSubs {
    pub name: String,
    pub relays: Relay,
    pub filters: Vec<Filter>,
}

impl CustomSubs {
    pub fn new() -> Self {
        Self {
            name: String::from("#steakstr"),
            relays: Relay::new(),
            filters: vec![],
        }
    }

    pub fn json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from(json: String) -> Self {
        serde_json::from_str(&json).unwrap()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Relay {
    pub name: String,
    pub relays: Vec<String>,
}

impl Relay {
    pub fn new() -> Self {
        Self {
            name: String::from("Default"),
            relays: vec![],
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Filter {
    pub r#type: String,
    pub accounts: Option<HashSet<Account>>,

    #[serde(
        serialize_with = "serialize_kinds_field",
        deserialize_with = "deserialize_kinds_field"
    )]
    pub kinds: Option<HashSet<Kind>>,

    #[serde(
        serialize_with = "serialize_u64_field",
        deserialize_with = "deserialize_u64_field"
    )]
    pub since: Option<u64>,

    #[serde(
        serialize_with = "serialize_u64_field",
        deserialize_with = "deserialize_u64_field"
    )]
    pub until: Option<u64>,

    pub limit: Option<usize>,
    pub tags: Option<HashSet<String>>,
    pub events: Option<HashSet<Event>>,
}

impl Filter {
    pub fn new_tag() -> Self {
        Self {
            r#type: "hashtag".to_string(),
            kinds: None,
            accounts: None,
            since: None,
            until: None,
            limit: None,
            tags: Some(HashSet::new()),
            events: None,
        }
    }

    pub fn new_account() -> Self {
        Self {
            r#type: "accounts".to_string(),
            kinds: Some(HashSet::new()),
            accounts: Some(HashSet::new()),
            since: None,
            until: None,
            limit: None,
            tags: None,
            events: None,
        }
    }

    pub fn new_event() -> Self {
        Self {
            r#type: "events".to_string(),
            kinds: None,
            accounts: None,
            since: None,
            until: None,
            limit: None,
            tags: None,
            events: Some(HashSet::new()),
        }
    }

    pub fn new_custom() -> Self {
        Self {
            r#type: "customized".to_string(),
            kinds: Some(HashSet::new()),
            accounts: Some(HashSet::new()),
            since: Some(0),
            until: None,
            limit: Some(500),
            tags: Some(HashSet::new()),
            events: None,
        }
    }

    pub fn kind(self, k: Kind) -> Self {
        let mut kinds = self.kinds.unwrap_or_default();
        kinds.insert(k);
        Self {
            kinds: Some(kinds),
            ..self
        }
    }

    pub fn remove_kind(self, k: Kind) -> Self {
        let mut kinds = self.kinds.unwrap_or(HashSet::new());
        kinds.remove(&k);
        Self {
            kinds: Some(kinds),
            ..self
        }
    }

    pub fn empty_account(self) -> Self {
        let mut accounts = self.accounts.unwrap_or(HashSet::new());
        accounts.insert(Account {
            alt_name: String::from(""),
            npub: String::from(""),
        });
        Self {
            accounts: Some(accounts),
            ..self
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Kind {
    pub index: u64,
    pub text: String,
}

impl Kind {
    pub fn from(k: nostr_sdk::Kind) -> Self {
        Self {
            index: k.as_u64(),
            text: match k {
                nostr_sdk::Kind::Metadata => "Metadata".to_string(),
                nostr_sdk::Kind::TextNote => "TextNote".to_string(),
                nostr_sdk::Kind::Repost => "Repost".to_string(),
                _ => "Unknown".to_string(),
            },
        }
    }
}

impl std::hash::Hash for Kind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl Eq for Kind {}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub alt_name: String,
    pub nevent: String,
}

impl std::hash::Hash for Event {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.nevent.hash(state);
    }
}

impl Eq for Event {}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub alt_name: String,
    pub npub: String,
}

impl std::hash::Hash for Account {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.npub.hash(state);
    }
}

impl Eq for Account {}

fn serialize_kinds_field<S>(kinds: &Option<HashSet<Kind>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match kinds {
        Some(kinds) => {
            let indexes: Vec<u64> = kinds.iter().map(|k| k.index).collect();
            serializer.serialize_str(&to_string(&indexes).unwrap())
        }
        None => serializer.serialize_none(),
    }
}

fn deserialize_kinds_field<'de, D>(deserializer: D) -> Result<Option<HashSet<Kind>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        let indexes: Vec<u64> = serde_json::from_str(&s).map_err(serde::de::Error::custom)?;
        Ok(Some(
            indexes
                .iter()
                .map(|i| Kind {
                    index: *i,
                    text: String::from(""),
                })
                .collect(),
        ))
    }
}

fn serialize_u64_field<S>(u: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match u {
        Some(u) => serializer.serialize_str(&to_string(u).unwrap()),
        None => serializer.serialize_none(),
    }
}

fn deserialize_u64_field<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(from_str(&s).map_err(serde::de::Error::custom)?))
    }
}
