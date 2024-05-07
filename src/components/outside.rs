use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ClickOutsideProps {
    on_click: EventHandler<bool>,
    #[props(default = String::new())]
    class: String,
    #[props(default = String::new())]
    style: String,
    children: Element,
}

#[component]
pub fn ClickOutside(props: ClickOutsideProps) -> Element {
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
            props.on_click.call(true);
        }
    }));

    rsx! {
        div {
            class: "{props.class}",
            style: "{props.style}",
            onclick: move |event| {
                pos.set(event.screen_coordinates().to_tuple());
            },
            { props.children }
        }
    }
}
