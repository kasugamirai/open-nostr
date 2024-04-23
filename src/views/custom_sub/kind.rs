use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct KindInputProps {
    on_change: EventHandler<Vec<u64>>,
    value: Vec<u64>,
}

fn kind_to_text(i: u64) -> String {
    match i {
        1 => "Note".to_string(),
        6 => "Repost".to_string(),
        _ => "Unknown".to_string(),
    }
}

#[component]
pub fn KindInput(props: KindInputProps) -> Element {
    let mut value = use_signal(|| props.value);
    let mut edit = use_signal(|| false);
    rsx! {
        div {
            style: "position: relative;",
            div {
                style: "background-color: var(--bgc-0); height: 42px; padding: 10px 20px; border-radius: var(--radius-circle); cursor: pointer; display: flex; align-items: center; justify-content: center; white-space: nowrap;",
                onclick: move |_| {
                    edit.set(!edit());
                    props.on_change.call(value.read().clone());
                },
                r#"{value().iter().map(|v| kind_to_text(*v)).collect::<Vec<String>>().join(" & ")}"#
            }
            div {
                class: "show-{edit}",
                style: "position: absolute; background-color: var(--bgc-0); border-radius: var(--radius-1); display: flex; flex-direction: column; gap: 10px; padding: 10px; 20px; border: 1px solid var(--boc-1); z-index: 100;",
                label {
                    style: "display: flex; align-items: center; gap: 10px;",
                    span {
                        "Note"
                    }
                    input {
                        r#type: "checkbox",
                        checked: value().contains(&1),
                        oninput: move |event| {
                            let enable = event.value() == "true";
                            let mut v = value.write();
                            if enable && !v.contains(&1) {
                                v.push(1);
                            } else {
                                v.retain(|v| *v != 1);
                            }
                            props.on_change.call(v.clone());
                        }
                    }
                }
                label {
                    style: "display: flex; align-items: center; gap: 10px;",
                    span {
                        "Repost"
                    }
                    input {
                        r#type: "checkbox",
                        checked: value().contains(&6),
                        oninput: move |event| {
                            let enable = event.value() == "true";
                            let mut v = value.write();
                            if enable && !v.contains(&6) {
                                v.push(6);
                            } else {
                                v.retain(|v| *v != 6);
                            }
                            props.on_change.call(v.clone());
                        }
                    }
                }
            }
        }
    }
}
