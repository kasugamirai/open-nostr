use dioxus::prelude::*;
use nostr_sdk::{Event, Kind};

use crate::nostr::note::TextNote;
use super::note::Note;
use super::reply::Reply;


#[derive(PartialEq, Clone, Props)]
pub struct NoteWrapperProps {
    sub_name: String,
    event: Event,
    relay_name: String,
}


#[component]
pub fn Note_wrapper(props: NoteWrapperProps) -> Element {
    let text_note: Option<TextNote> = if props.event.kind == Kind::TextNote {
        Some(TextNote::try_from(props.event.clone()).unwrap())
    } else {
        None
    };
    if let Some(text_note) = text_note {
        let is_reply = text_note.is_reply();
        if is_reply {
            rsx! {
                Reply {
                    sub_name: props.sub_name.clone(),
                    event: props.event,
                    relay_name: props.relay_name.clone(),
                }
            }
        } else {
            rsx! {
                Note {
                    sub_name: props.sub_name.clone(),
                    event: props.event,
                    relay_name: props.relay_name.clone(),
                }
            }
        }
    } else {
        rsx! {
            Note {
                sub_name: props.sub_name.clone(),
                event: props.event,
                relay_name: props.relay_name.clone(),
            }
        }
    }
}
