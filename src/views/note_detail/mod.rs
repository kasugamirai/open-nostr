use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use dioxus::prelude::*;
use nostr_sdk::{Event, EventId, Timestamp};

use crate::{
    nostr::{
        multiclient::MultiClient,
        note::{self, ReplyTreeManager, ReplyTrees, TextNote},
        {get_event_by_id, get_replies},
    },
    views::note_list::note::Note,
    CustomSub,
};
#[component]
pub fn NoteDetail(sub: String, root_id: String, note_id: String) -> Element {
    let sub_name = use_signal(|| sub.clone());
    let mut rootid = use_signal(|| root_id.clone());
    let mut highlight_note_id = use_signal(|| note_id.clone());
    let multiclient = use_context::<Signal<MultiClient>>();
    let subs_map = use_context::<Signal<HashMap<String, CustomSub>>>();
    let mut replytree_manager = use_context::<Signal<ReplyTreeManager>>();

    use_effect(use_reactive((&root_id, &note_id), move |(root, note)| {
        rootid.set(root);
        highlight_note_id.set(note);
    }));

    let tree_exists = {
        let manager = replytree_manager.read();
        manager
            .get_tree(&EventId::from_hex(rootid()).unwrap())
            .is_some()
    };
    // use__
    use_effect(use_reactive(
        (&rootid(), &sub_name()),
        move |(_root_id, sub_name)| {
            let root_event_id: EventId = EventId::from_hex(_root_id).unwrap();
            let clients = multiclient();
            // let _all_sub = all_sub();
            let _subs_map = subs_map();
            spawn(async move {
                // if tree not exists, fetch it
                if !tree_exists {
                    if !_subs_map.contains_key(&sub_name) {
                        return;
                    }
                    let sub = _subs_map.get(&sub_name).unwrap();
                    if let Some(client) = clients.get_client(&sub.relay_set).await {
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
            });
        },
    ));

    let tree = use_memo({
        let replytree_manager = replytree_manager;
        let _rootid = EventId::from_hex(rootid()).unwrap().clone();
        move || {
            let manager = replytree_manager.read();
            manager.get_tree(&_rootid).cloned()
        }
    });
    let mut render_notes = use_signal(ReplyTrees::default);
    let mut refresh = use_signal(Timestamp::now);
    use_effect(use_reactive(
        (&tree(), &highlight_note_id()),
        move |(newest_tree, newest_highlight_event_id)| {
            render_notes.write().clear();
            if let Some(newest_tree) = newest_tree {
                let highlight_event_id =
                    &EventId::from_hex(newest_highlight_event_id.clone()).unwrap();
                let highlight_note = newest_tree.get_note_by_id(highlight_event_id);

                if let Some(highlight_note) = highlight_note {
                    // group by highlight_note and find the longest chain
                    let replies = newest_tree.get_replies(&highlight_note.inner.id, None);
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
                    all_replies.extend(ancestors.into_iter().map(|note| note.clone().inner));
                    all_replies.extend(chain);
                    // tracing::info!("chain and ancestors: ---------------------- {:?}", all_replies);
                    // tracing::info!("other_replies: ---------------------- {:?}", other_replies);
                    all_replies.extend(other_replies);
                    // all_replies.de
                    all_replies = vec_unique(all_replies, |e| e.id);

                    render_notes.write().accept(all_replies);
                    refresh.set(Timestamp::now());
                }
            }
        },
    ));
    // let mut note_tree = use_signal(|| {
    //     rsx! {
    //         div {
    //             class: "note-detail-mode-content",
    //             div {
    //                 class: "relative z-1",
    //                 {render_note_tree(&render_notes(), rootid(), highlight_note_id(), sub_name())}
    //             }
    //         }
    //     }
    // });
    // use_effect(move || {
    //     // tracing::info!("refresh_time: {:?}", refresh_time);
    //     let (newest_render_notes, newest_highlight_note_id) = (render_notes(), highlight_note_id());
    //     tracing::info!("rerender start: yyds");
    //     note_tree.set(rsx! {
    //         div {
    //             class: "note-detail-mode-content",
    //             div {
    //                 class: "relative z-1",
    //                 {render_note_tree(&newest_render_notes, rootid(), newest_highlight_note_id, sub_name())}
    //             }
    //         }
    //     });
    //     tracing::info!("rerender complete: yyds");
    // });
    rsx! {
        div {
            // onmounted: on_mounted,
            class: "note-detail-mode-box",
            // {note_tree()}
            div {
                class: "note-detail-mode-content",
                div {
                    class: "relative z-1",
                    {render_note_tree(&render_notes(), rootid(), highlight_note_id(), sub_name())}
                }
            }
            div {
                class:"width-500",
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
