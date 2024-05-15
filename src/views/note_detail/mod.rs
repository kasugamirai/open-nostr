// use std::time::Duration;

// use dioxus::prelude::*;
// use nostr_sdk::{Alphabet, Event, EventId, Filter, JsonUtil, Kind, SingleLetterTag};
// use serde::Serialize;

// use crate::{
//     nostr::{
//         multiclient::MultiClient,
//         note::{DisplayOrder, ReplyTrees, TextNote},
//     },
//     views::note_list::note::{Note, NoteData},
// };

// #[derive(Debug, Clone, Serialize)]
// struct NoteTree {
//     content: String,
//     children: Vec<NoteTree>,
//     event: Event,
// }

// impl PartialEq for NoteTree {
//     fn eq(&self, other: &Self) -> bool {
//         self.content == other.content
//     }
// }

// fn get_notetree(id: EventId, reply_tree: &ReplyTrees) -> Vec<NoteTree> {
//     let r_children: Vec<&TextNote> = reply_tree.get_replies(&id, Some(DisplayOrder::NewestFirst));
//     r_children
//         .iter()
//         .map(|n| NoteTree {
//             content: n.inner_ref.content.clone(),
//             children: get_notetree(n.inner_ref.id, reply_tree),
//             event: Event::from(n.inner_ref.clone()),
//         })
//         .collect()
// }

// #[component]
// pub fn NoteDetail(sub: String, id: String) -> Element {
//     let multiclient = use_context::<Signal<MultiClient>>();
//     let mut data = use_signal(|| vec![]);
//     let mut count = use_signal(|| 0);
//     let get_event = move |event_id: String| {
//         let sub_name = sub.clone();
//         spawn(async move {
//             let clients = multiclient();
//             let client = clients.get(&sub_name).unwrap();

//             let mut reply_filter = Filter::new().kind(Kind::TextNote);
//             reply_filter = reply_filter
//                 .custom_tag(SingleLetterTag::lowercase(Alphabet::E), [event_id.clone()]);
//             let reply_events = client
//                 .get_events_of(vec![reply_filter], Some(Duration::from_secs(30)))
//                 .await
//                 .unwrap();

//             let mut note_filter = Filter::new().kind(Kind::TextNote);
//             note_filter =
//                 note_filter.ids(reply_events.iter().map(|e| e.id).collect::<Vec<EventId>>());
//             let note_events = client
//                 .get_events_of(vec![note_filter], Some(Duration::from_secs(30)))
//                 .await
//                 .unwrap();

//             let mut filter: Filter = Filter::new();
//             filter = filter.limit(1);
//             filter = filter.id(EventId::from_hex(event_id.clone()).unwrap());

//             let mut events = client
//                 .get_events_of(vec![filter], Some(Duration::from_secs(30)))
//                 .await
//                 .unwrap();
//             events.extend(note_events);

//             let e = r#"{"created_at":1715673885,"content":"R -> A -> 1","tags":[["p","00a1fc288605b95dac49aad52d2031697d6424ee78dabf37414a33ea7e340cee"],["e","f3f00d33096a12cde40c5c30a08d1b52296b51f11ea1e7aa3252495cae4225fb","wss://bostr.nokotaro.com/","root"],["client","Lume"]],"kind":1,"pubkey":"00a1fc288605b95dac49aad52d2031697d6424ee78dabf37414a33ea7e340cee","id":"19df1ebe8d5143c0b8d76a126dcd97327ca3d6ef2a2b0d9e6cf513e2f934eaf8","sig":"9ce3366c13b763b34635b07b01123efb3a05834c3db35557ae7ff0fa12392602c4f26da1ce1894400e44170cede12ff06b4fe0cbaac49dabedd2036b5a33ba8f"}"#;
//             let e = Event::from_json(e).unwrap();
//             events.push(e);

//             if events.len() > 0 {
//                 count.set(events.len());
//                 let event_refs: Vec<&Event> = events.iter().collect();
//                 let mut reply_tree = ReplyTrees::default();
//                 tracing::info!("==> events json: {:?}", serde_json::to_string(&event_refs));
//                 reply_tree.accept(&event_refs);
//                 let notetree = vec![NoteTree {
//                     content: "This is the Root!".to_string(),
//                     children: get_notetree(EventId::parse(event_id).unwrap(), &reply_tree),
//                     event: events[0].clone(),
//                 }];
//                 tracing::info!("==> notetree json: {:?}", serde_json::to_string(&notetree));
//                 data.set(notetree);
//             }
//         });
//     };

//     let on_mounted = move |_| {
//         get_event(id.clone());
//     };

//     let scirpts: &str = r#"
//             var expandList = document.querySelectorAll('.note-action-expand');
//             expandList.forEach(function(expand) {
//                 expand.addEventListener('click', function() {
//                     var parent = expand.parentElement.parentElement;
//                     const expandItem = parent.nextElementSibling;
//                     if (expandItem.classList.contains('expand-list-open')) {
//                         expandItem.classList.remove('expand-list-open');
//                         expandItem.classList.add('expand-list-close');
//                     } else {
//                         expandItem.classList.add('expand-list-open');
//                         expandItem.classList.remove('expand-list-close');
//                     }
//                 });
//             });
//         "#;

//     rsx! {
//         div {
//             // onmounted: on_mounted,
//             button {
//                 onclick: on_mounted,
//                 "Get Event"
//             }
//             Layer {
//                 notes: data(),
//                 index: count + 1,
//                 root: true,
//                 events_len: Some(count() as u64), // Convert usize to Option<u64>
//             }
//             script {
//                 "{scirpts}"
//             }
//         }
//     }
// }

// #[derive(PartialEq, Clone, Props)]
// pub struct LayerProps {
//     notes: Vec<NoteTree>,
//     #[props(default = usize::MAX)]
//     index: usize,
//     #[props(default = false)]
//     root: bool,
//     events_len: Option<u64>,
//     clsname: Option<&'static str>,
// }

// #[component]
// fn Layer(props: LayerProps) -> Element {
//     rsx! {
//         for (index, note) in props.notes.iter().enumerate() {
//             Item {
//                 event: Event::from(note.event.clone()),
//                 reply: false,
//                 index: props.index,
//                 is_expand: if note.children.len() > 0 {Some(true)} else {None},
//                 clsname: if index == 0 {props.clsname.unwrap_or("")} else {""},
//             }
//             if note.children.len() > 0 {
//                 div {
//                     class: format!("expand-list-open expand-list z-{} relative", props.index - 1),
//                     Layer {
//                         notes: note.children.clone(),
//                         index: props.index,
//                         root: false,
//                         clsname: "pt-20",
//                     }
//                 }
//             }
//         }
//     }
// }

// #[derive(PartialEq, Clone, Props)]
// pub struct ItemProps {
//     event: Event,
//     reply: bool,
//     index: usize,
//     events_len: Option<u64>,
//     clsname: &'static str,
//     is_expand: Option<bool>,
// }

// #[component]
// fn Item(props: ItemProps) -> Element {
//     rsx! {
//         Note {
//             sub_name: "".to_string(),
//             data: NoteData::from(&props.event, props.index),
//             clsname: format!("z-{} mb-20 bgc-0 relative {}", props.index, props.clsname),
//             is_expand: props.is_expand
//         }
//     }
// }
