use nostr_sdk::{Filter, Kind, PublicKey, SingleLetterTag, Timestamp};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

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
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CustomSub {
    pub name: String,
    pub relay_set: RelaySet,
    pub filters: Vec<FilterTemp>,
}

impl CustomSub {
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
            filters: vec![FilterTemp::HashTag(CustomHashTag {
                r#type: String::from("hashtag"),
                tags: vec![String::from("#steakstr"), String::from("#steak")],
            })],
        }
    }

    pub fn json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn to_sub(&self) -> Vec<Filter> {
        self.filters
            .iter()
            .map(|x| x.to_sub())
            .collect::<Vec<Filter>>()
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
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RelaySet {
    pub name: String,
    pub relays: Vec<String>,
}

impl RelaySet {
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

#[derive(Debug, PartialEq, Clone)]
pub enum FilterTemp {
    HashTag(CustomHashTag),
    Accounts(CustomAccounts),
    Events(CustomEvents),
    Customize(CustomFilter),
}

impl FilterTemp {
    pub fn to_sub(&self) -> Filter {
        let mut filter = Filter::new();
        match self {
            FilterTemp::HashTag(hashtag) => {
                filter = filter.hashtags(hashtag.tags.clone());
            }
            FilterTemp::Accounts(accounts) => {
                filter = filter.kinds(
                    accounts
                        .kinds
                        .iter()
                        .map(|&x| Kind::from(x))
                        .collect::<Vec<Kind>>(),
                );
                filter = filter.authors(
                    accounts
                        .accounts
                        .iter()
                        .map(|x| PublicKey::parse(&x.npub).unwrap())
                        .collect::<Vec<PublicKey>>(),
                );
            }
            FilterTemp::Events(_events) => {}
            FilterTemp::Customize(customize) => {
                if !customize.kinds.is_empty() {
                    filter = filter.kinds(
                        customize
                            .kinds
                            .iter()
                            .map(|&x| Kind::from(x))
                            .collect::<Vec<Kind>>(),
                    );
                }
                if !customize.accounts.is_empty() {
                    filter = filter.authors(
                        customize
                            .accounts
                            .iter()
                            .map(|x| PublicKey::parse(&x.npub).unwrap())
                            .collect::<Vec<PublicKey>>(),
                    )
                }
                if customize.since > 0 {
                    filter = filter.since(Timestamp::from(customize.since));
                }
                if customize.until > 0 {
                    filter = filter.until(Timestamp::from(customize.until));
                }
                if customize.limit > 0 {
                    filter = filter.limit(customize.limit);
                }
                for tag in customize.tags.clone() {
                    let k: SingleLetterTag = tag.tag.parse().unwrap();
                    let parts: Vec<&str> = tag.value.split(',').map(|s| s.trim()).collect();
                    filter = filter.custom_tag(k, parts);
                }
            }
        }
        filter
    }
}

impl Serialize for FilterTemp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            FilterTemp::HashTag(hashtag) => hashtag.serialize(serializer),
            FilterTemp::Accounts(accounts) => accounts.serialize(serializer),
            FilterTemp::Events(events) => events.serialize(serializer),
            FilterTemp::Customize(custom) => custom.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for FilterTemp {
    fn deserialize<D>(deserializer: D) -> Result<FilterTemp, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;

        match value.get("type").and_then(Value::as_str) {
            Some("hashtag") => {
                let hashtag = serde_json::from_value(value).unwrap();
                Ok(FilterTemp::HashTag(hashtag))
            }
            Some("accounts") => {
                let accounts = serde_json::from_value(value).unwrap();
                Ok(FilterTemp::Accounts(accounts))
            }
            Some("events") => {
                let events = serde_json::from_value(value).unwrap();
                Ok(FilterTemp::Events(events))
            }
            Some("custom") => {
                let custom = serde_json::from_value(value).unwrap();
                Ok(FilterTemp::Customize(custom))
            }
            _ => Err(serde::de::Error::custom("Unknown filter type")),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CustomHashTag {
    pub r#type: String,
    pub tags: Vec<String>,
}

impl CustomHashTag {
    pub fn empty() -> Self {
        Self {
            r#type: String::from("hashtag"),
            tags: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CustomAccounts {
    pub r#type: String,
    pub kinds: Vec<u64>,
    pub accounts: Vec<Account>,
}

impl CustomAccounts {
    pub fn empty() -> Self {
        Self {
            r#type: String::from("accounts"),
            kinds: vec![],
            accounts: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CustomEvents {
    pub r#type: String,
    pub events: Vec<Event>,
}

impl CustomEvents {
    pub fn empty() -> Self {
        Self {
            r#type: String::from("events"),
            events: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Event {
    pub alt_name: String,
    pub nevent: String,
}

impl Event {
    pub fn empty() -> Self {
        Self {
            alt_name: String::from(""),
            nevent: String::from(""),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CustomFilter {
    pub r#type: String,
    pub kinds: Vec<u64>,
    pub accounts: Vec<Account>,
    pub since: u64,
    pub until: u64,
    pub limit: usize,
    pub tags: Vec<Tag>,
}

impl CustomFilter {
    pub fn empty() -> Self {
        Self {
            r#type: String::from("customized"),
            kinds: vec![],
            accounts: vec![],
            since: 0,
            until: 0,
            limit: 0,
            tags: vec![],
        }
    }
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
pub struct Tag {
    pub tag: String,
    pub value: String,
}

impl Tag {
    pub fn empty() -> Self {
        Self {
            tag: String::from(""),
            value: String::from(""),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_json() {
        let custom_sub = CustomSub::default();
        println!("custom_sub: {:?}", custom_sub);

        let json = custom_sub.json();
        println!("json: {}", json);

        let cs = serde_json::from_str::<super::CustomSub>(&json).unwrap();
        println!("--------cs: {:?}", cs);
    }
}
