use nostr_sdk::prelude::*;

#[component]
pub fn Quote(event: nostr_sdk::Event) -> Element {
    // getQuoteEvent by EventId ....... 
    rsx! {
        div {
            class: "event-less",
            Avatar { pubkey: event.author(),  timestamp: event.created_at().as_u64() }
            div {
                class: "text",
                dangerous_inner_html: "{event.content.to_string()}",
            }
        }
    }
}