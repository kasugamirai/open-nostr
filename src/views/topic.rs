use dioxus::prelude::*;

#[component]
pub fn Topic(topic_id: String) -> Element {
    let eval = eval(
        r#"
            let msg = await dioxus.recv();
            console.log(id);
        "#,
    );
    eval.send(topic_id.clone().into()).unwrap();

    rsx! {
        div {
            "{topic_id}"
        }
    }
}
