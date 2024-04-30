use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    let mut text = use_signal(|| "Hello".to_string());
    let mut content = use_signal(|| "Hello".to_string());
    rsx! {
        button {
            onclick: move |_| {
                text.set("World".to_string());
                content.set("World".to_string());
            },
            "Hello World"
        },
        TestCom {
            text: text,
            content: "{content}"
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct TestComProps {
    text: Signal<String>,
    content: String,
}

#[component]
fn TestCom(props: TestComProps) -> Element {
    use_effect(move || {
        tracing::info!("Starting Dioxus {}", props.text.read());
    });

    rsx! {
        h1 {
            "==> {props.content}"
        }
    }
}
