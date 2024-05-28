use std::cmp::min;
use std::collections::{HashMap, VecDeque};

use indextree::{Arena, NodeId};
use nostr::nips::nip10::Marker;
use nostr_sdk::{Alphabet, Event, EventId, Kind, Tag, TagKind};
use nostr_sdk::{SingleLetterTag, TagStandard};
use std::fmt;

use super::utils::{self, get_children};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    KindNotMatch,
    NotEnoughElements,
    NormalizationFailed,
    NodeIdNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::KindNotMatch => write!(f, "Kind does not match"),
            Error::NotEnoughElements => write!(f, "Not enough elements in no_marker_array"),
            Error::NormalizationFailed => write!(f, "Normalization failed"),
            Error::NodeIdNotFound => write!(f, "Node ID not found"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextNote {
    pub inner: Event,
    pub root: Option<EventId>,
    pub reply_to: Option<EventId>,
}

impl TextNote {
    pub fn new(event: Event) -> Self {
        TextNote {
            inner: event,
            root: None,
            reply_to: None,
        }
    }

    pub fn is_root(&self) -> bool {
        matches!((&self.root, &self.reply_to), (None, None))
    }
    pub fn is_reply(&self) -> bool {
        matches!((&self.root, &self.reply_to), (Some(_), Some(_)))
            || matches!((&self.root, &self.reply_to), (Some(_), None))
    }

    fn process_tags(event: &Event, text_note: &mut Self) -> Result<(), Error> {
        let mut no_marker_array: Vec<EventId> = vec![];

        for tag in event.iter_tags() {
            let tag_standard = tag.as_standardized();
            let new_tag = match tag_standard {
                Some(tag) => tag.clone(),
                None => match normalize_e_tag(tag) {
                    Ok(Some(normalized_tag)) => normalized_tag,
                    _ => continue,
                },
            };

            if let TagStandard::Event {
                event_id, marker, ..
            } = new_tag
            {
                match marker {
                    Some(Marker::Root) => text_note.root = Some(event_id),
                    Some(Marker::Reply) => text_note.reply_to = Some(event_id),
                    None => no_marker_array.push(event_id),
                    _ => {}
                }
            }
        }

        if let (None, Some(reply)) = (&text_note.root, &text_note.reply_to) {
            text_note.root = Some(*reply);
        }
        // Assign root to reply_to if it is a reply to root
        if let (Some(root), None) = (&text_note.root, &text_note.reply_to) {
            text_note.reply_to = Some(*root);
        }

        // Handle case where no marker is present
        if text_note.reply_to.is_none() {
            match no_marker_array.len() {
                1 => {
                    text_note.root = no_marker_array.first().cloned();
                    text_note.reply_to = no_marker_array.first().cloned();
                }
                n if n >= 2 => {
                    text_note.root = no_marker_array.first().cloned();
                    text_note.reply_to = no_marker_array.get(1).cloned();
                }
                _ => {
                    return Err(Error::NotEnoughElements);
                }
            }
        }
        Ok(())
    }
}

fn normalize_e_tag(t: &Tag) -> Result<Option<TagStandard>, Error> {
    match t.kind() {
        TagKind::SingleLetter(SingleLetterTag {
            character: Alphabet::E,
            uppercase: false,
        }) => {
            let t_vec = <nostr::Tag as Clone>::clone(t).to_vec();
            let at_most_4 = &t_vec[..min(4, t_vec.len())];
            let normalized_t = at_most_4.to_vec();
            match TagStandard::parse(&normalized_t) {
                Ok(tag) => Ok(Some(tag)),
                Err(_) => Err(Error::NormalizationFailed),
            }
        }
        _ => Ok(None),
    }
}

impl TryFrom<Event> for TextNote {
    type Error = Error;

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        if event.kind == Kind::TextNote {
            let mut text_note = TextNote::new(event.clone());
            let _ = TextNote::process_tags(&event, &mut text_note); // pass event directly
            Ok(text_note)
        } else {
            Err(Error::KindNotMatch)
        }
    }
}

#[derive(Debug)]
pub struct ReplyTrees {
    id2id: HashMap<EventId, NodeId>,
    arena: Arena<TextNote>,
    notes: Vec<TextNote>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DisplayOrder {
    NewestFirst,
    DeepestFirst,
}

impl Default for ReplyTrees {
    fn default() -> Self {
        Self {
            id2id: HashMap::new(),
            arena: Arena::new(),
            notes: Vec::new(),
        }
    }
}

impl ReplyTrees {
    pub fn accept(&mut self, events: Vec<Event>) -> Result<(), Error> {
        let text_notes: Vec<TextNote> = events
            .into_iter()
            .filter_map(|e| TextNote::try_from(e).ok())
            .collect();

        for event in text_notes.into_iter() {
            let node_id = self.arena.new_node(event.clone());
            self.id2id.insert(event.inner.id, node_id);
            self.notes.push(event);
        }

        for tn in &self.notes {
            if let Some(node_id) = self.id2id.get(&tn.inner.id) {
                if let Some(reply_to) = &tn.reply_to {
                    if let Some(p_node_id) = self.id2id.get(reply_to) {
                        p_node_id.append(*node_id, &mut self.arena);
                    }
                }
            } else {
                return Err(Error::NodeIdNotFound);
            }
        }
        Ok(())
    }

    pub fn get_note_by_id(&self, id: &EventId) -> Option<&TextNote> {
        self.id2id
            .get(id)
            .and_then(|node_id| self.arena.get(*node_id).map(|node| node.get()))
    }

    pub fn get_replies(&self, id: &EventId, order: Option<DisplayOrder>) -> Vec<&TextNote> {
        if let Some(node_id) = self.id2id.get(id) {
            let mut results = get_children(&self.arena, *node_id);
            match order {
                Some(DisplayOrder::NewestFirst) => {
                    results.sort_by(|b, a| a.inner.created_at.cmp(&b.inner.created_at));
                    results
                }
                _ => results,
            }
        } else {
            vec![]
        }
    }

    pub fn get_ancestors(&self, id: &EventId) -> Vec<&TextNote> {
        if let Some(node_id) = self.id2id.get(id) {
            utils::get_ancestors(&self.arena, *node_id)
        } else {
            vec![]
        }
    }
    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }
    pub fn clear(&mut self) {
        self.id2id.clear();
        self.arena.clear();
        self.notes.clear();
    }
}

// use std::collections::{HashMap, VecDeque};
// use std::time::{SystemTime, UNIX_EPOCH};
// use nostr_sdk::{Event, EventId};

#[derive(Debug)]
pub struct ReplyTreeManager {
    trees: HashMap<EventId, ReplyTrees>,
    order: VecDeque<EventId>, // 用来记录添加顺序
    max_entries: usize,
}
impl ReplyTreeManager {
    pub fn new(max_entries: usize) -> Self {
        ReplyTreeManager {
            trees: HashMap::new(),
            order: VecDeque::new(),
            max_entries,
        }
    }

    pub fn add_tree(&mut self, root_id: EventId, tree: ReplyTrees) {
        // 如果超过最大条目，删除最早的
        if self.order.len() >= self.max_entries {
            if let Some(oldest_id) = self.order.pop_front() {
                self.trees.remove(&oldest_id);
            }
        }
        
        self.trees.insert(root_id, tree);
        self.order.push_back(root_id);
    }

    pub fn get_or_create_tree(&mut self, root_id: EventId) -> &mut ReplyTrees {
        // 如果存在就返回，如果不存在则创建
        if !self.trees.contains_key(&root_id) {
            let new_tree = ReplyTrees::default();
            self.add_tree(root_id.clone(), new_tree);
        }
        self.trees.get_mut(&root_id).unwrap()
    }
    pub fn get_tree(&self, root_id: &EventId) -> Option<&ReplyTrees> {
        self.trees.get(root_id)
    }

    pub fn clear(&mut self) {
        self.trees.clear();
        self.order.clear();
    }

    pub fn accept_event(&mut self, root_id: EventId, events: Vec<Event>) {
        let tree = self.get_or_create_tree(root_id.clone());
        tree.accept(events);
    }

    pub fn modify_tree_with_event<F>(&mut self, root_id: &EventId, event: Event, modify: F)
    where
        F: FnOnce(&mut ReplyTrees, Event),
    {
        let tree = self.get_or_create_tree(root_id.clone());
        modify(tree, event);
    }
}



#[cfg(test)]
mod tests {
    use crate::testhelper::{event_from, test_data::*};
    use super::*;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_no_note() {
        let event = event_from(NOT_NOTE);
        assert!(
            TextNote::try_from(event).is_err(),
            "Expect an event with kind 1"
        );
    }

    #[wasm_bindgen_test]
    fn test_reply_with_marker() {
        let event = event_from(REPLY_WITH_MARKER);
        let text_note = TextNote::try_from(event).unwrap();

        assert!(
            text_note.root.unwrap().to_hex()
                == *"39413ed0400101a45abb82dd8949306790234f785ea224717d0f68fa1b36e935"
        );
        assert!(
            text_note.reply_to.unwrap().to_hex()
                == *"3cacfcc0afa9d1daf798291b8d8b31fd0b471303f501e188191444ff4cdf1345"
        );
    }

    #[wasm_bindgen_test]
    fn test_reply_with_no_marker() {
        let event = event_from(REPLY_WITH_NO_MARKER);
        let text_note = TextNote::try_from(event).unwrap();
        assert!(
            text_note.root.unwrap().to_hex()
                == *"a200b725177cc2fcbb0c40c5103695da6a8cbd9e73c5a9293c8bfd45521a84bc"
        );
        assert!(
            text_note.reply_to.unwrap().to_hex()
                == *"cfab5dabf95fa14c21a611a3eff120132a470201407bd6799ae1c5058b88b430"
        );
    }

    #[wasm_bindgen_test]
    fn test_reply_to_root_no_marker() {
        let event = event_from(REPLY_TO_ROOT_WITH_NO_MARKER);
        let text_note = TextNote::try_from(event).unwrap();
        assert!(
            text_note.root.unwrap().to_hex()
                == *"1c556c3a9e892841bef2bfae13ca5fdc50f81054d031a6a16b060a2e5113ae24"
        );
        assert!(
            text_note.reply_to.unwrap().to_hex()
                == *"1c556c3a9e892841bef2bfae13ca5fdc50f81054d031a6a16b060a2e5113ae24"
        );
    }

    #[wasm_bindgen_test]
    fn test_reply_to_root_with_marker() {
        let event = event_from(REPLY_TO_ROOT_WITH_MARKER);
        let text_note = TextNote::try_from(event).unwrap();
        assert!(
            text_note.root.unwrap().to_hex()
                == *"ff25d26e734c41fa7ed86d28270628f8fb2f6fb03a23eed3d38502499c1a7a2b"
        );
        assert!(
            text_note.reply_to.unwrap().to_hex()
                == *"ff25d26e734c41fa7ed86d28270628f8fb2f6fb03a23eed3d38502499c1a7a2b"
        );
    }

    #[wasm_bindgen_test]
    fn test_is_root() {
        let event = event_from(ROOT_NOTE);
        let text_note = TextNote::try_from(event).unwrap();
        assert!(text_note.is_root());
    }

    #[wasm_bindgen_test]
    fn test_get_note() {
        let event = event_from(ROOT_NOTE);
        let mut reply_tree = ReplyTrees::default();
        reply_tree.accept(vec![event]).unwrap();
        let event_id =
            EventId::parse("c3d8e01d3884d8914583ef1da76e3e1732824228e89cfda3b5fe1164bbb9dd38")
                .unwrap();
        assert!(reply_tree.get_note_by_id(&event_id).unwrap().inner.id == event_id);
        assert!(reply_tree.get_note_by_id(&event_id).unwrap().inner.content == *"If i do createElement and rhen appendChild for a lot of number of time, It took a lot of RAM compared to writting the entire HTML manually.");
    }

    #[wasm_bindgen_test]
    fn test_get_replies_ordered() {
        let events: Vec<Event> = [R, R_A, R_A_B, R_X, R_Z, R_Z_O]
            .iter()
            .map(|raw: &&str| event_from(raw))
            .collect();
        let mut reply_tree = ReplyTrees::default();
        reply_tree.accept(events).unwrap();
        let r_children = reply_tree.get_replies(
            &EventId::parse("9a708c373de54236d7707feb8c7ae21aa8a204eb9f6dc289de05f90a9e311651")
                .unwrap(),
            Some(DisplayOrder::NewestFirst),
        );
        assert!(r_children.len() == 3);
        assert!(r_children.first().unwrap().inner.content == "R -> Z");
        assert!(r_children.last().unwrap().inner.content == "R -> A");
        //pick a child
        let r_a_children = reply_tree.get_replies(
            &EventId::parse("9421678017349485b5ac0cd8d6de4907f34b00338e8b255c6fcfe6790fb09511")
                .unwrap(),
            Some(DisplayOrder::NewestFirst),
        );
        assert!(r_a_children.len() == 1);
        assert!(r_a_children.first().unwrap().inner.content == "R -> A -> B");
    }

    #[wasm_bindgen_test]
    fn test_get_replies_with_orphan() {
        let events: Vec<Event> = [R, R_A, R_A_B, R_X, R_Z_O]
            .iter()
            .map(|raw: &&str| event_from(raw))
            .collect();
        let mut reply_tree = ReplyTrees::default();
        reply_tree.accept(events).unwrap();
        let r_children = reply_tree.get_replies(
            &EventId::parse("9a708c373de54236d7707feb8c7ae21aa8a204eb9f6dc289de05f90a9e311651")
                .unwrap(),
            Some(DisplayOrder::NewestFirst),
        );
        assert!(r_children.len() == 2);
        assert!(r_children.last().unwrap().inner.content == "R -> A");
    }

    #[wasm_bindgen_test]
    fn test_get_ancestors() {
        let events: Vec<Event> = [R, R_A, R_A_B, R_X, R_Z, R_Z_O]
            .iter()
            .map(|raw: &&str| event_from(raw))
            .collect();
        let mut reply_tree = ReplyTrees::default();
        reply_tree.accept(events).unwrap();
        let ancestors = reply_tree.get_ancestors(
            &EventId::parse("b916e11013514ad0d8c5d8005e2c760c4557cc3c261f4f98ec6f1748c7c8b541")
                .unwrap(),
        );
        assert!(ancestors.first().unwrap().inner.content == "R -> A");
        assert!(ancestors.last().unwrap().inner.content == "This is the Root!");
    }

    #[wasm_bindgen_test]
    fn test_get_in_batch() {
        //assume we already have root
        let root: Vec<Event> = [R].iter().map(|raw: &&str| event_from(raw)).collect();
        //assume the following data is fetched by get_replies
        let replies: Vec<Event> = [R_A, R_A_B, R_X, R_Z, R_Z_O]
            .iter()
            .map(|raw: &&str| event_from(raw))
            .collect();
        let mut reply_tree = ReplyTrees::default();
        reply_tree.accept(root).unwrap();
        reply_tree.accept(replies).unwrap();
        let ancestors = reply_tree.get_ancestors(
            &EventId::parse("b916e11013514ad0d8c5d8005e2c760c4557cc3c261f4f98ec6f1748c7c8b541")
                .unwrap(),
        );
        assert!(ancestors.first().unwrap().inner.content == "R -> A");
        assert!(ancestors.last().unwrap().inner.content == "This is the Root!");
        //pick a child
        let r_a_children = reply_tree.get_replies(
            &EventId::parse("9421678017349485b5ac0cd8d6de4907f34b00338e8b255c6fcfe6790fb09511")
                .unwrap(),
            Some(DisplayOrder::NewestFirst),
        );
        assert!(r_a_children.len() == 1);
        assert!(r_a_children.first().unwrap().inner.content == "R -> A -> B");
    }

    #[wasm_bindgen_test]
    fn test_failed_process_tags() {
        let event = event_from(ERROR_EVENT);
        let mut text_note = TextNote::new(event);
        let event = text_note.inner.clone();
        let result = TextNote::process_tags(&event, &mut text_note);
        assert!(result.is_err());
        assert!(result.err().unwrap() == Error::NotEnoughElements);
    }
}
