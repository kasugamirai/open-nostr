
// let mut emoji = use_signal(|| HashMap::new());


// match get_reactions(&client, &eid(), None).await {
//     Ok(emojis) => {
//         emoji.set(emojis);
//     }
//     Err(_) => {
//         tracing::info!("metadata not found");
//     }
// }

// div {
//     class: "note-action-item cursor-pointer flex items-center",
//     span {
//         class: "note-action-icon",
//         dangerous_inner_html: "{ADD}"
//     }
// }
// for (k, v) in emoji().iter() {
//     div {
//         class: "note-action-item cursor-pointer flex items-center",
//         span {
//             class: "note-action-icon",
//             "{k}"
//         }
//         span {
//             class: "note-action-count font-size-12 txt-1",
//             "{v}"
//         }
//     }
// }