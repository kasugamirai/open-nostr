use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct DateTimePickerProps {
    on_change: EventHandler<(u64, u64)>,
    value: u64,
    #[props(default = 0)]
    end: u64,
    #[props(default = false)]
    range: bool,
}

#[component]
pub fn DateTimePicker(props: DateTimePickerProps) -> Element {
    let mut value = use_signal(|| props.value);
    let mut end = use_signal(|| props.end);
    rsx! {
        div {
            class: "com-dtpicker",
            input {
                r#type: "datetime-local",
                value: "{value()}",
                oninput: move |event| {
                    value.set(event.value().parse::<u64>().unwrap_or(0));
                }
            }
            input {
                r#type: "datetime-local",
                value: "{end()}",
                oninput: move |event| {
                    end.set(event.value().parse::<u64>().unwrap_or(0));
                    props.on_change.call((value(), end()));
                }
            }
        }
    }
}
