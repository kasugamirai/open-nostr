use chrono::{DateTime, NaiveDateTime};
use dioxus::prelude::*;
use crate::components::icons::*;

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
    let mut start_signal = use_signal(|| props.value);
    let mut end_signal = use_signal(|| props.end);
    let mut start_value = use_signal(|| String::new());
    let mut end_value = use_signal(|| String::new());

    use_effect(use_reactive((&props.value, &props.end), move |(start, end)| {
        let start_time = DateTime::from_timestamp(start as i64, 0).unwrap();
        let end_time = DateTime::from_timestamp(end as i64, 0).unwrap();
        start_value.set(start_time.format("%Y-%m-%dT%H:%M").to_string());
        end_value.set(end_time.format("%Y-%m-%dT%H:%M").to_string());
    }));

    // tracing::info!("start_value: {:?}   {}", start_value, props.value);
    // tracing::info!("end_value: {:?}", end_value);
    // tracing::info!("value: {:?}", start_signal());
    // tracing::info!("end: {:?}", end_signal());

    rsx! {
        div {
            class: "com-dtpicker",
            input {
                r#type: "datetime-local",
                value: "{start_value}",
                oninput: move |event| {
                    let v = event.value();
                    if v.len() == 0 {
                        start_signal.set(0);
                        props.on_change.call((start_signal(), end_signal()));
                    } else {
                        let parsed_datetime = NaiveDateTime::parse_from_str(&v, "%Y-%m-%dT%H:%M").unwrap();
                        let timestamp = parsed_datetime.and_utc().timestamp() as u64;
                        start_signal.set(timestamp);
                        tracing::info!("value: {:?}", parsed_datetime);
                        props.on_change.call((start_signal(), end_signal()));
                    }
                }
            }
            span{
              class:"data-start-icon",
              dangerous_inner_html: "{LEFTICON}",
            }
            span{
              class:"data-end-icon",
              dangerous_inner_html: "{RIGHTICON}",
            }
            input {
                class:"end_data",
                r#type: "datetime-local",
                value: "{end_value}",
                oninput: move |event| {
                    let v = event.value();
                    if v.len() == 0 {
                        end_signal.set(0);
                        props.on_change.call((start_signal(), end_signal()));
                    } else {
                        let parsed_datetime = NaiveDateTime::parse_from_str(&v, "%Y-%m-%dT%H:%M").unwrap();
                        let timestamp = parsed_datetime.and_utc().timestamp() as u64;
                        end_signal.set(timestamp);
                        props.on_change.call((start_signal(), end_signal()));
                    }
                }
            }
        }
    }
}
