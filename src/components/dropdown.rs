use dioxus::prelude::*;
use serde_json::Value;

#[derive(PartialEq, Clone, Props)]
pub struct DropdownProps {
    #[props(default = "left".to_string())]
    pos: String,
    #[props(default = "hover".to_string())]
    show: String,
    trigger: Element,
    children: Element,
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let style = if props.pos == "left" {
        "left: 0;"
    } else {
        "right: 0;"
    };

    let mut show = use_signal(|| false);

    let hash = format!("com-dropdown");
    let id = hash.clone();

    let get_events = move |id: String| {
        spawn(async move {
            let mut eval = eval(
                r#"
                    // Listens for clicks on the 'document' element
                    let eid = await dioxus.recv()
                    const handle = (e) => {
                        let target = e.target
                        while (target != null) {
                            if (target.classList.contains(eid)) {
                                // The element is a child of the dropdown
                                dioxus.send(true)
                                break
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
            eval.send(id.into()).unwrap();
            if let Value::Bool(res) = eval.recv().await.unwrap() {
                show.set(res);
            }
        });
    };
    get_events(hash.clone());
    rsx! {
        div {
            class: "com-dropdown {props.show}",
            div {
                class: "com-dropdown-trigger",
                onclick: move |_| {
                    show.set(!show());
                },
                { props.trigger }
            }
            div {
                id: "{id}",
                class: "com-dropdown-content {props.show} {show}",
                style: "{style}",
                { props.children }
            }
        }
    }
}
