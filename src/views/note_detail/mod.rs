use dioxus::prelude::*;
use nostr_sdk::EventId;

use crate::{
    nostr::{
        fetch::{get_event_by_id, get_replies},
        multiclient::MultiClient,
        note::{ReplyTreeManager, ReplyTrees, TextNote},
    },
    views::note_list::note::Note,
    CustomSub,
};
#[component]
pub fn NoteDetail(sub: String, root_id: String, note_id: String) -> Element {
    let sub_name = use_signal(|| sub.clone());
    let root_id = use_signal(|| root_id.clone());
    let multiclient = use_context::<Signal<MultiClient>>();
    let all_sub = use_context::<Signal<Vec<CustomSub>>>();
    let mut replytree_manager = use_context::<Signal<ReplyTreeManager>>();

    let rootid: EventId = EventId::from_hex(&root_id()).unwrap();
    let tree_exists = {
        let manager = replytree_manager.read();
        manager.get_tree(&rootid).is_some()
    };
    use_effect(use_reactive(
        (&root_id(), &sub_name()),
        move |(_, sub_name)| {
            let clients = multiclient();
            let _all_sub = all_sub();
            spawn(async move {
                // if tree not exists, fetch it
                if !tree_exists {
                    let sub = _all_sub.iter().find(|s| s.name == sub_name).unwrap();
                    if let Some(client) = clients.get_client(&sub.relay_set) {
                        let client = client.client();
                        match get_event_by_id(&client, &rootid, None).await {
                            Ok(Some(event)) => {
                                replytree_manager.write().accept_event(rootid, vec![event]);
                            }
                            Ok(None) => {
                                tracing::error!("event not found");
                            }
                            Err(e) => {
                                tracing::error!("error: {:?}", e);
                            }
                        };
                        match get_replies(&client, &rootid, None).await {
                            Ok(replies) => {
                                replytree_manager
                                    .write()
                                    .accept_event(rootid.clone(), replies.clone());
                                tracing::info!("replies: {:?}", replies);
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
    let manager_lock = replytree_manager.read();

    let tree = manager_lock.get_tree(&EventId::from_hex(&root_id()).unwrap());

    rsx! {
        div {
            // onmounted: on_mounted,
            class: "note-detail-mode-box",
            div {
                class: "note-detail-mode-content",
                div {
                    class: "relative z-1",
                    {render_note_tree(tree, note_id.clone(), sub_name())}
                }
            }
            div {
                class:"width-500",
            }
        }
    }
}

fn render_note_tree(
    tree: Option<&ReplyTrees>,
    highlight_note_id: String,
    sub_name: String,
) -> Element {
    if let Some(tree) = tree {
        let root_node = tree.get_note_by_id(&EventId::from_hex(&highlight_note_id).unwrap());
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
                is_expand: children.len() > 0,
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
