use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use dioxus::prelude::*;
use nostr_sdk::{Event, EventId};

use crate::components::Author;
use crate::nostr::MultiClient;
use crate::nostr::{ReplyTreeManager, ReplyTrees, TextNote};
use crate::nostr::{get_event_by_id, get_replies};
use crate::views::note_list::note::Note;
use crate::CustomSub;
#[component]
pub fn NoteDetail(sub: String, root_id: String, note_id: String) -> Element {
    let mut sub_name = use_signal(|| sub.clone());
    let mut rootid = use_signal(|| root_id.clone());
    let mut highlight_note_id = use_signal(|| note_id.clone());
    let multiclient = use_context::<Signal<MultiClient>>();
    let subs_map = use_context::<Signal<HashMap<String, CustomSub>>>();
    let mut replytree_manager = use_context::<Signal<ReplyTreeManager>>();
    let mut sub_current = use_signal(|| CustomSub::empty());
    let mut pubkey_str = use_signal(|| "".to_string());

    let mut render_notes = use_signal(ReplyTrees::default);

    let tree_exists = {
        let manager = replytree_manager.read();
        manager
            .get_tree(&EventId::from_hex(root_id.clone()).unwrap())
            .is_some()
    };

    let tree = use_memo({
        let replytree_manager = replytree_manager;
        let _rootid = EventId::from_hex(root_id.clone()).unwrap().clone();
        move || {
            let manager = replytree_manager.read();
            manager.get_tree(&_rootid).cloned()
        }
    });
    use_effect(use_reactive(
        (&root_id, &note_id.clone(), &sub),
        move |(_root_id, new_note_id, new_sub_name)| {
            {
                rootid.set(_root_id.clone());
                highlight_note_id.set(new_note_id.clone());
                sub_name.set(new_sub_name.clone());
                // pubkey_str.set("".to_string());
                // refresh.set(Timestamp::now());
                tracing::info!("root_id: {:?}", _root_id);
            }
            {
                // let _all_sub = all_sub();
                let _subs_map = subs_map();
                spawn(async move {
                    // if tree not exists, fetch it
                    if !tree_exists {
                        if !_subs_map.contains_key(&new_sub_name) {
                            return;
                        }
                        let root_event_id: EventId = EventId::from_hex(_root_id.clone()).unwrap();
                        let clients = multiclient();
                        let _sub = _subs_map.get(&new_sub_name).unwrap();
                        sub_current.set(_sub.clone());
                        if let Some(client) = clients.get_client(&_sub.relay_set).await {
                            let client = client.client();
                            match get_event_by_id(&client, &root_event_id, None).await {
                                Ok(Some(event)) => {
                                    replytree_manager
                                        .write()
                                        .accept_event(root_event_id, vec![event]);
                                }
                                Ok(None) => {
                                    tracing::error!("event not found");
                                }
                                Err(e) => {
                                    tracing::error!("error: {:?}", e);
                                }
                            };
                            match get_replies(&client, &root_event_id, None).await {
                                Ok(replies) => {
                                    replytree_manager
                                        .write()
                                        .accept_event(root_event_id, replies.clone());
                                    // refresh.set(!refresh());
                                }
                                Err(e) => {
                                    tracing::error!("error: {:?}", e);
                                }
                            };
                        } else {
                            tracing::error!("client not found");
                        }
                    }
                    {
                        render_notes.write().clear();
                        if let Some(newest_tree) = tree() {
                            let highlight_event_id =
                                &EventId::from_hex(new_note_id.clone()).unwrap();
                            let highlight_note = newest_tree.get_note_by_id(highlight_event_id);

                            if let Some(highlight_note) = highlight_note {
                                pubkey_str.set(highlight_note.inner.author().to_string().clone());
                                // group by highlight_note and find the longest chain
                                let replies =
                                    newest_tree.get_replies(&highlight_note.inner.id, None);
                                let chain = find_longest_chain(&newest_tree, highlight_note);
                                let chain: Vec<Event> =
                                    chain.into_iter().map(|note| note.clone().inner).collect();
                                let other_replies = replies
                                    .into_iter()
                                    .flat_map(|note| {
                                        let res = find_longest_chain(&newest_tree, note);
                                        res.into_iter()
                                            .map(|note| note.clone().inner)
                                            .collect::<Vec<Event>>()
                                    })
                                    .collect::<Vec<Event>>();
                                let ancestors = newest_tree.get_ancestors(&highlight_note.inner.id);
                                let mut all_replies = Vec::<Event>::new();
                                all_replies
                                    .extend(ancestors.into_iter().map(|note| note.clone().inner));
                                all_replies.extend(chain);
                                all_replies.extend(other_replies);
                                all_replies = vec_unique(all_replies, |e| e.id);

                                render_notes.write().accept(all_replies);
                            }
                        }
                    }
                });
            }
        },
    ));

    rsx! {
        div {
            class: "note-detail-mode-box",
            div {
                class: "note-detail-mode-content",
                div {
                    key: "{highlight_note_id()}",
                    class: "relative z-1",
                    {render_note_tree(&render_notes(), rootid(), highlight_note_id(), sub_name())}
                }
            }
            div{
                class:"overflow-y-auto",
                if pubkey_str().is_empty() {
                    div { "Loading..." }
                } else {
                    Author{
                        key: "{pubkey_str()}",
                        pubkey: pubkey_str.to_string(),
                        relay_name: sub_current().relay_set.clone(),
                    }
                }
              }
        }
    }
}

fn render_note_tree(
    tree: &ReplyTrees,
    root_id: String,
    highlight_note_id: String,
    sub_name: String,
) -> Element {
    let root_node = tree.get_note_by_id(&EventId::from_hex(root_id).unwrap());
    if let Some(root_note) = root_node {
        let root_id = root_note.get_root().unwrap_or(root_note.inner.id);
        let root_node = tree.get_note_by_id(&root_id).unwrap();
        return render_note_node(
            tree,
            root_node,
            true,
            highlight_note_id,
            sub_name.clone(),
            false,
        );
    }
    rsx! { div { "Loading..." } }
}

fn render_note_node(
    tree: &ReplyTrees,
    note: &TextNote,
    is_root: bool,
    highlight_note_id: String,
    sub_name: String,
    show_pt_size: bool,
) -> Element {
    let _ = is_root;
    let children = tree.get_replies(&note.inner.id, None);
    let is_highlight = note.inner.id.to_string() == highlight_note_id;
    let mut show = use_signal(|| true);
    let children_len = children.len();

    rsx! {
            Note {
                key: "{note.inner.id.to_hex()}",
                on_expand: move |_| {
                    show.set(!show());
                },
                sub_name: sub_name.clone(),
                event: note.inner.clone(),
                is_expand: !children.is_empty(),
                is_tree: true,
                clsname: format!("relative {} z-{} {} mb-12", if is_highlight {
                    "com-post--active"
                } else {
                    ""
                }, if children_len > 0 {children_len} else {0}, if show_pt_size {"pt-16"} else {""})
            }
            if !children.is_empty() && show() {
                div {

                    class: format!("relative z-{}", if children_len > 0 {children_len-1} else {0}),
                    style: format!("margin-top: -28px;"),
                    for (i, reply) in children.iter().enumerate() {
                        {render_note_node(
                            tree,
                            reply,
                            false,
                            highlight_note_id.clone(),
                            sub_name.clone(),
                            i == 0
                        )}
                    }
                }
            }
    }
}

fn find_longest_chain<'a>(tree: &'a ReplyTrees, note: &'a TextNote) -> Vec<&'a TextNote> {
    tracing::info!(
        "find_longest_chain {:?} {:?}",
        note.inner.id.to_hex(),
        note.inner.content
    );
    let mut longest_chain: Vec<&TextNote> = Vec::new();
    for reply in tree.get_replies(&note.inner.id, None) {
        let chain = find_longest_chain(tree, reply);
        if chain.len() > longest_chain.len() {
            longest_chain = chain;
        }
    }

    let mut result = Vec::with_capacity(longest_chain.len() + 1);
    result.push(note);
    result.extend(longest_chain);

    result
}
fn vec_unique<T, F, K>(data: Vec<T>, key_extractor: F) -> Vec<T>
where
    F: Fn(&T) -> K,
    K: Eq + Hash,
{
    let mut seen = HashSet::new();
    let mut combined = Vec::new();

    for item in data.into_iter() {
        let key = key_extractor(&item);
        if seen.insert(key) {
            combined.push(item);
        }
    }

    combined
}
