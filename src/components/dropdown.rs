use dioxus::prelude::*;
//use serde_json::Value;

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

    let hash = "com-dropdown".to_string();
    let id = hash;

    // close when click outside
    let root_click_pos = use_context::<Signal<(f64, f64)>>();
    let mut pos: Signal<(f64, f64)> = use_signal(|| root_click_pos());
    use_effect(use_reactive((&pos,), move |(pos,)| {
        // The coordinates of root element
        let root_pos = root_click_pos();

        // The coordinates of current element
        let current_pos = pos();

        // Determine if two coordinates are the same
        if current_pos.0 != root_pos.0 || current_pos.1 != root_pos.1 {
            show.set(false);
        }
    }));

    rsx! {
        div {
            onclick: move |event| {
                // Save the coordinates of the event relative to the screen
                pos.set(event.screen_coordinates().to_tuple());
            },
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
