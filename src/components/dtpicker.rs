use chrono::{DateTime, NaiveDateTime};
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

/// DateTimePicker
///
/// # Example
///
/// ```
/// DateTimePicker {
///     value: 0,  // start time
///     end: 1000,  // end time
///     range: true,  // select time range
///     on_change: move |(start, end): (u64, u64)| {
///         
///     },
/// }
/// ```
#[component]
pub fn DateTimePicker(props: DateTimePickerProps) -> Element {
    let mut value = use_signal(|| props.value);
    let mut end = use_signal(|| props.end);
    let start_value = DateTime::from_timestamp(value() as i64, 0)
        .unwrap()
        .format("%Y-%m-%dT%H:%M")
        .to_string();
    let end_value = DateTime::from_timestamp(end() as i64, 0)
        .unwrap()
        .format("%Y-%m-%dT%H:%M")
        .to_string();
    rsx! {
        div {
            class: "com-dtpicker2",
            input {
                r#type: "datetime-local",
                value: "{start_value}",
                oninput: move |event| {
                    let parsed_datetime = NaiveDateTime::parse_from_str(&event.value(), "%Y-%m-%dT%H:%M").unwrap();
                    let timestamp = parsed_datetime.and_utc().timestamp() as u64;
                    value.set(timestamp);
                    props.on_change.call((value(), end()));
                }
            }
            input {
                r#type: "datetime-local",
                value: "{end_value}",
                oninput: move |event| {
                    let parsed_datetime = NaiveDateTime::parse_from_str(&event.value(), "%Y-%m-%dT%H:%M").unwrap();
                    let timestamp = parsed_datetime.and_utc().timestamp() as u64;
                    end.set(timestamp);
                    props.on_change.call((value(), end()));
                }
            }
        }
    }
}
