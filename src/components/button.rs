use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
    on_click: EventHandler<MouseEvent>,
    children: Element,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: "com-button",
            onclick: move |evt| props.on_click.call(evt),
            { props.children }
        }
    }
}
// use dioxus::prelude::*;
 
// // 父组件
// fn parent_component(cx: Scope) -> Element {
//     let parent_value = use_state(&cx, || String::from("initial value"));
 
//     cx.render(rsx!(
//         child_component {
//             on_change: move |new_value| parent_value.set(new_value),
//             value: parent_value.clone(),
//         }
//     ))
// }
 
// // 子组件的属性
// struct ChildProps {
//     on_change: Callback<String>,
//     value: String,
// }
 
// // 子组件
// fn child_component(cx: Scope, props: ChildProps) -> Element {
//     let on_change = props.on_change;
//     let value = props.value;
 
//     cx.render(rsx!(
//         input {
//             value: value,
//             onchange: move |evt| {
//                 if let InputEvent::Change(input_evt) = evt {
//                     if let Some(value) = input_evt.value.as_ref() {
//                         on_change.call(value.clone());
//                     }
//                 }
//             },
//         }
//     ))
// }
 
// fn main() {
//     dioxus::desktop::launch(app);
// }
 
// fn app(cx: Scope) -> Element {
//     cx.render(rsx!(
//         div {
//             parent_component()
//         }
//     ))
// }