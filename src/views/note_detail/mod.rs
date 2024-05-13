use std::time::Duration;

use dioxus::prelude::*;
use nostr_sdk::{Client, Event, EventId, Filter, JsonUtil};

use crate::{
    nostr::note::{DisplayOrder, ReplyTrees, TextNote}, views::note_list::note::{Note, NoteData},
};

#[derive(Debug, Clone)]
struct NoteTree {
    content: String,
    children: Vec<NoteTree>,
    event: Event
}

impl PartialEq for NoteTree {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

#[component]
pub fn NoteDetail(id: String) -> Element {
    let mut data = use_signal(Vec::<Event>::new);
    let _get_events = move |id: String| {
        spawn(async move {
            let client = Client::default();

            client.add_relay("wss://btc.klendazu.com").await.unwrap();

            client.connect().await;

            let mut filter: Filter = Filter::new();
            filter = filter.limit(1);
            filter = filter.id(EventId::from_hex(id).unwrap());

            let events = client
                .get_events_of(vec![filter], Some(Duration::from_secs(30)))
                .await
                .unwrap();
            data.set(events);

            let _ = client.disconnect().await;
        });
    };

    const R: &str = r#"{"content":"This is the Root!","created_at":1713517255,"id":"9a708c373de54236d7707feb8c7ae21aa8a204eb9f6dc289de05f90a9e311651","kind":1,"pubkey":"eba1300e9189ef52f89ddd365b8d172d234275b2288c8fbad4a18306ae13562b","sig":"d082581cb2570adc0b0b124e8b72561b22521d7efc8aca28959e7522a55c78c74420cb57440f07ff8ebe741760c417acd0b489c60ff7e4845ea23a3d98414256","tags":[]}"#;
    const R_A: &str = r#"{"content":"R -> A","created_at":1713517325,"id":"9421678017349485b5ac0cd8d6de4907f34b00338e8b255c6fcfe6790fb09511","kind":1,"pubkey":"eba1300e9189ef52f89ddd365b8d172d234275b2288c8fbad4a18306ae13562b","sig":"4a84b9e1a0b2e567f2db542aae076f58de854eca4f88e2f2f8fa9fbc8cbdfa6753e39e04481bb7dd6279d7ec427741c679c51468288b5839c50ab1cfea6eaee3","tags":[["e","9a708c373de54236d7707feb8c7ae21aa8a204eb9f6dc289de05f90a9e311651","wss://relay.damus.io/","root"],["e","9a708c373de54236d7707feb8c7ae21aa8a204eb9f6dc289de05f90a9e311651","wss://relay.damus.io/","reply"]]}"#;
    const R_A_B: &str = r#"{"content":"R -> A -> B","created_at":1713517509,"id":"b916e11013514ad0d8c5d8005e2c760c4557cc3c261f4f98ec6f1748c7c8b541","kind":1,"pubkey":"eba1300e9189ef52f89ddd365b8d172d234275b2288c8fbad4a18306ae13562b","sig":"cee8db81d4aba889681f25c5358789f2f37da67a39ca7082cdc62c8cabff439f3a2f0f424e86361960169abf4ddb73ee79c7fd4a203a94dbebd8ce477a323b13","tags":[["e","9a708c373de54236d7707feb8c7ae21aa8a204eb9f6dc289de05f90a9e311651","wss://relay.damus.io/","root"],["e","9421678017349485b5ac0cd8d6de4907f34b00338e8b255c6fcfe6790fb09511","wss://relay.damus.io/","reply"]]}"#;
    const R_X: &str = r#"{"content":"R -> X","created_at":1713517591,"id":"c1d15b70fb1cb48792cac33949e4daf74148ef58e23a254a947ae11b1a0b89cc","kind":1,"pubkey":"eba1300e9189ef52f89ddd365b8d172d234275b2288c8fbad4a18306ae13562b","sig":"8035bb03c41851be82bae370fcdfafd8af666206b8cd3b2e7788a00d1ef4335c14f919ca4eb7fa3ed1e0614f41f15389d0439099e466dbe9bf0d3fe205269ca5","tags":[["e","9a708c373de54236d7707feb8c7ae21aa8a204eb9f6dc289de05f90a9e311651","","root"],["e","9a708c373de54236d7707feb8c7ae21aa8a204eb9f6dc289de05f90a9e311651","","reply"]]}"#;
    const R_Z: &str = r#"{"content":"R -> Z","created_at":1713517740,"id":"e9356a18293d8122c233d19b405ab8523773fa9419db0bd634bd592ebd250a87","kind":1,"pubkey":"eba1300e9189ef52f89ddd365b8d172d234275b2288c8fbad4a18306ae13562b","sig":"5a4c8c02a75b2fb9ffb567995366629d28c2d131b0e5359bbdc008211b400c265384a5d743cedb794526f54f6474ac6151ca02a5ca150a464d0b11840e0c2ffe","tags":[["e","9a708c373de54236d7707feb8c7ae21aa8a204eb9f6dc289de05f90a9e311651","","root"],["e","9a708c373de54236d7707feb8c7ae21aa8a204eb9f6dc289de05f90a9e311651","","reply"]]}"#;
    const R_Z_O: &str = r#"{"content":"R -> Z -> O","created_at":1713517783,"id":"b3ec05726a7b456a7a2212981c7278ccb08d366c5caa9d1e29f2b5d652b00cf5","kind":1,"pubkey":"eba1300e9189ef52f89ddd365b8d172d234275b2288c8fbad4a18306ae13562b","sig":"63ea4e6e43006c0dc7501a111eebf348006813d9abb359a317214a6941bb6eceb889b57fca2c57b1deef568f10ca9e3f2105b43da814644612466b04185f7033","tags":[["e","9a708c373de54236d7707feb8c7ae21aa8a204eb9f6dc289de05f90a9e311651","","root"],["e","e9356a18293d8122c233d19b405ab8523773fa9419db0bd634bd592ebd250a87","wss://relay.damus.io/","reply"]]}"#;
    let events: Vec<Event> = [R, R_A, R_A_B, R_X, R_Z, R_Z_O]
        .iter()
        .map(|raw: &&str| Event::from_json(raw).unwrap())
        .collect();
    let event_refs: Vec<&Event> = events.iter().collect();
    let mut reply_tree = ReplyTrees::default();
    reply_tree.accept(&event_refs);

    fn get_notetree(id: EventId, reply_tree: &ReplyTrees) -> Vec<NoteTree> {
        let r_children: Vec<&TextNote> =
            reply_tree.get_replies(&id, Some(DisplayOrder::NewestFirst));
        r_children
            .iter()
            .map(|n| NoteTree {
                content: n.inner_ref.content.clone(),
                children: get_notetree(n.inner_ref.id, reply_tree),
                event: Event::from(n.inner_ref.clone())
            })
            .collect()
    }


    let notetree = vec![NoteTree {
        content: "This is the Root!".to_string(),
        children: get_notetree(
            EventId::parse("9a708c373de54236d7707feb8c7ae21aa8a204eb9f6dc289de05f90a9e311651")
                .unwrap(),
            &reply_tree,
        ),
        event: Event::from_json(R).unwrap()
    }];
    rsx! {
        div {
            onmounted: move |_cx| {},
            Layer {
                notes: notetree,
                index: events.len() + 1,
                root: true,
                events_len: Some(events.len() as u64), // Convert usize to Option<u64>
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct LayerProps {
    notes: Vec<NoteTree>,
    #[props(default = usize::MAX)]
    index: usize,
    #[props(default = false)]
    root: bool,
    events_len: Option<u64>,
    clsname: Option<&'static str>
}

#[component]
fn Layer(props: LayerProps) -> Element {
    rsx! {
        for (index, note) in props.notes.iter().enumerate() {
            Item {
                event: Event::from(note.event.clone()),
                reply: false,
                index: props.index,
                clsname: if index == 0 {props.clsname.unwrap_or("")} else {""},
            }
            if note.children.len() > 0 {
                div {
                    style: "margin-top: -34px",
                    class: format!("z-{} relative", props.index - 1),
                    Layer {
                        notes: note.children.clone(),
                        index: props.index,
                        root: false,
                        clsname: "pt-20",
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct ItemProps {
    event: Event,
    reply: bool,
    index: usize,
    events_len: Option<u64>,
    clsname: &'static str
}

#[component]
fn Item(props: ItemProps) -> Element {
    rsx! {
        Note {
            data: NoteData::from(&props.event, props.index),
            clsname: format!("z-{} mb-20 bgc-0 relative {}", props.index, props.clsname),
        }
    }
}
