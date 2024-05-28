use dioxus::prelude::*;

use crate::views::note_list::custom_sub::relays::RelaysInput;

use crate::store::subscription::{CustomSub,RelaySet};



#[component]
pub fn Relay() -> Element {
   

    let subs = use_context::<Signal<Vec<CustomSub>>>();

    let current_name: Vec<_> = subs
    .iter()
    .map(|relay| relay.relay_set.clone())
    .collect();
    tracing::info!("subssubssubssubs: {:?}", current_name);

    rsx! {
       div{
        class:"relay-contnet",
        div{
          class:"built-in-function text-center font-size-16",
          "built-in function"
          div{
            class:"built-li radius-26 text-center mt-12 font-size-14 line-height-28 text-overflow",
            "DM"
          }
          div{
            class:"built-li radius-26 text-center mt-12 font-size-14 line-height-28 text-overflow",
            "Channel"
          }
          div{
            class:"built-li radius-26 text-center mt-12 font-size-14 line-height-28 text-overflow",
            "Community"
          }
          div{
            class:"built-li radius-26 text-center mt-12 font-size-14 line-height-28 text-overflow",
            "Group"
          }
          div{
            class:"separate mt-20"
          }
          "Supscription"
          div{
            class:"separate mb-12"
          }
          div{
            class:"built-li radius-26 text-center mb-28 font-size-14 line-height-28 text-overflow built-li-checked",
            "#steakstr"
          }
          div{
            class:"built-li radius-26 text-center mb-28 font-size-14 line-height-28 text-overflow",
            "Movie #my collection"
          }
        }
        div{
          class:"set-content ml-78 px-18 py-18 radius-26",
          RelaysInput {
            on_change: move |v: RelaySet| {
                // let mut sub = sub_current.write();
                // sub.relay_set = v.name.clone();
            },
            relay_name: "{&current_name[0]}",
            is_popup:false
          }
        }
       }
    }
}

