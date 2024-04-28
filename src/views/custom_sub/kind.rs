use dioxus::prelude::*;
use serde_json::Value;

#[derive(PartialEq, Clone, Props)]
pub struct KindInputProps {
    on_change: EventHandler<Vec<u64>>,
    value: Vec<u64>,
    #[props(default = 0)]
    index: usize,
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

    let click_outside = move |cn: String| {
        spawn(async move {
            let mut eval = eval(
                r#"
                    // Listens for clicks on the 'document' element
                    let eid = await dioxus.recv()
                    const handle = (e) => {
                        let target = e.target
                        while (true && target) {
                            if (target.classList.contains(eid)) {
                                // The element is a child of the dropdown
                                dioxus.send(true)
                                return
                            } else {
                                if (target === document.documentElement) {
                                    break
                                }
                            }
                            target = target.parentNode
                        }
                        
                        // The element is outside the dropdown
                        dioxus.send(false)

                        // Remove the event listener
                        // document.removeEventListener('click', handle)
                    }
                    document.addEventListener('click', handle)
                "#,
            );
            eval.send(cn.into()).unwrap();
            if let Value::Bool(res) = eval.recv().await.unwrap() {
                edit.set(res);
            }
        });
    };

    let cn = format!("custom-sub-kind-wapper-{}", props.index);

    click_outside(cn.clone());

    rsx! {
        div {
            class: "{cn}",
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
