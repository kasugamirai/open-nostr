use nostr_sdk::{EventId, Filter, Kind, PublicKey, SingleLetterTag, Timestamp};
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
        let now: u64 = Timestamp::now().as_u64();
        Self {
            name: String::from("#steakstr"),
            relay_set: RelaySet {
                name: String::from("Default"),
                relays: vec![
                    String::from("wss://btc.klendazu.com"),
                    // String::from("wss://nostr.pjv.me"),
                ],
            },
            filters: vec![
                FilterTemp::HashTag(CustomHashTag {
                    r#type: String::from("hashtag"),
                    tags: vec![String::from("dog")],
                }),
                FilterTemp::Accounts(CustomAccounts {
                    r#type: String::from("accounts"),
                    kinds: vec![1],
                    accounts: vec![Account {
                        alt_name: "AltName".to_string(),
                        npub: "npub1pjvcwasj9ydasx9nmkf09pftsg640vm5fs7tzprssew8544yj2ds6e0h42"
                            .to_string(),
                    }],
                }),
                FilterTemp::Events(CustomEvents {
                    r#type: String::from("events"),
                    events: vec![Event {
                        alt_name: "EventName".to_string(),
                        nevent: "nevent hash".to_string(),
                    }],
                }),
                FilterTemp::Customize(CustomFilter {
                    r#type: String::from("customized"),
                    kinds: vec![Kind::TextNote.as_u64()],
                    accounts: vec![Account {
                        alt_name: "AltName".to_string(),
                        npub: "npub1pjvcwasj9ydasx9nmkf09pftsg640vm5fs7tzprssew8544yj2ds6e0h42"
                            .to_string(),
                    }],
                    since: 0,
                    until: now,
                    limit: 10,
                    tags: vec![],
                }),
            ],
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
                filter = filter.hashtags(&hashtag.tags);
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
            FilterTemp::Events(events) => {
                for x in &events.events {
                    filter = filter.event(EventId::from_hex(&x.nevent).unwrap());
                }
            }
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
    use nostr_sdk::EventId;
    use nostr_sdk::Filter;
    use nostr_sdk::PublicKey;
    use std::str::FromStr;

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

    #[test]
    fn test_account_sub() {
        let public_key: &str = "npub1dvxmgeq0w7t44ejvhgu6r0yrtthtwmtlfftlg230ecc9l9e3fqgshca58l";
        let custom_sub = CustomSub {
            name: String::from("Test"),
            relay_set: RelaySet {
                name: String::from("TestRelaySet"),
                relays: vec![String::from("wss://relay.damus.io")],
            },
            filters: vec![FilterTemp::Accounts(CustomAccounts {
                r#type: String::from("accounts"),
                kinds: vec![1],
                accounts: vec![Account {
                    alt_name: String::from("User1"),
                    npub: String::from(public_key),
                }],
            })],
        };
        let custom_filter = custom_sub.filters[0].to_sub();

        let filter = Filter::new()
            .author(PublicKey::from_str(public_key).unwrap())
            .kind(Kind::TextNote);

        assert_eq!(filter, custom_filter);
    }

    #[test]
    fn test_default_sub() {
        let custom_sub = CustomSub::default();
        println!("custom_sub: {:?}", custom_sub);
        let filters = custom_sub.to_sub();
        println!("filters: {:?}", filters);

        let filter =
            Filter::new().hashtags(vec![String::from("#steakstr"), String::from("#steak")]);

        assert_eq!(filters[0], filter);
    }

    #[test]
    fn test_custom_sub() {
        let public_key: &str = "npub1dvxmgeq0w7t44ejvhgu6r0yrtthtwmtlfftlg230ecc9l9e3fqgshca58l";
        let custom_sub = CustomSub {
            name: String::from("Test"),
            relay_set: RelaySet {
                name: String::from("TestRelaySet"),
                relays: vec![String::from("wss://relay.damus.io")],
            },
            filters: vec![FilterTemp::Customize(CustomFilter {
                r#type: String::from("customized"),
                kinds: vec![1],
                accounts: vec![Account {
                    alt_name: String::from("User1"),
                    npub: String::from(public_key),
                }],
                since: 0,
                until: 0,
                limit: 0,
                tags: vec![],
            })],
        };
        let custom_filter = custom_sub.filters[0].to_sub();

        let filter = Filter::new()
            .author(PublicKey::from_str(public_key).unwrap())
            .kind(Kind::TextNote);

        assert_eq!(filter, custom_filter);
    }

    #[test]
    fn test_event_sub() {
        let event_id =
            EventId::from_hex("70b10f70c1318967eddf12527799411b1a9780ad9c43858f5e5fcd45486a13a5")
                .unwrap();
        let custom_sub = CustomSub {
            name: String::from("Test"),
            relay_set: RelaySet {
                name: String::from("TestRelaySet"),
                relays: vec![String::from("wss://relay.damus.io")],
            },
            filters: vec![FilterTemp::Events(CustomEvents {
                r#type: String::from("events"),
                events: vec![Event {
                    alt_name: String::from("Event1"),
                    nevent: String::from(
                        "70b10f70c1318967eddf12527799411b1a9780ad9c43858f5e5fcd45486a13a5",
                    ),
                }],
            })],
        };
        let custom_filter = custom_sub.filters[0].to_sub();

        let filter = Filter::new().event(event_id);
        println!("filter: {:?}", filter);
        println!("custom_filter: {:?}", custom_filter);

        assert_eq!(filter, custom_filter);
    }

    #[test]
    fn test_multiple_filters_sub() {
        let public_key: &str = "npub1dvxmgeq0w7t44ejvhgu6r0yrtthtwmtlfftlg230ecc9l9e3fqgshca58l";
        let custom_sub = CustomSub {
            name: String::from("Test"),
            relay_set: RelaySet {
                name: String::from("TestRelaySet"),
                relays: vec![String::from("wss://relay.damus.io")],
            },
            filters: vec![
                FilterTemp::Accounts(CustomAccounts {
                    r#type: String::from("accounts"),
                    kinds: vec![1],
                    accounts: vec![Account {
                        alt_name: String::from("User1"),
                        npub: String::from(public_key),
                    }],
                }),
                FilterTemp::HashTag(CustomHashTag {
                    r#type: String::from("hashtag"),
                    tags: vec![String::from("#rust"), String::from("#programming")],
                }),
            ],
        };
        let filters = custom_sub.to_sub();

        let filter1 = Filter::new()
            .author(PublicKey::from_str(public_key).unwrap())
            .kind(Kind::TextNote);
        let filter2 =
            Filter::new().hashtags(vec![String::from("#rust"), String::from("#programming")]);

        assert_eq!(filters.len(), 2);
        assert_eq!(filters[0], filter1);
        assert_eq!(filters[1], filter2);
    }

    #[test]
    fn test_empty_filters() {
        let custom_sub = CustomSub {
            name: String::from("EmptyTest"),
            relay_set: RelaySet {
                name: String::from("EmptyRelaySet"),
                relays: vec![String::from("wss://relay.damus.io")],
            },
            filters: vec![],
        };

        let filters = custom_sub.to_sub();
        assert!(
            filters.is_empty(),
            "Filters should be empty but found {:?}",
            filters
        );
    }
}
