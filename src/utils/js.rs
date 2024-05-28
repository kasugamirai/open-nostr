use dioxus::prelude::*;
use serde_json::Value;

pub fn export_to_clipboard(text: String) -> UseEval {
    let eval: UseEval = eval(
        r#"
            let c = navigator.clipboard;
            if (!c) {
                console.error('Clipboard not supported');
                return false;
            }
            let msg = await dioxus.recv();
            console.log(msg);
            await c.writeText(msg);
            alert("Copied to clipboard");
            return true;
        "#,
    );
    eval.send(text.into()).unwrap();
    eval
}

pub async fn import_from_clipboard() -> String {
    let mut eval: UseEval = eval(
        r#"
            let c = navigator.clipboard;
            if (!c) {
                console.error('Clipboard not supported');
                return false;
            }
            let msg = await c.readText();
            console.log(msg);
            await dioxus.send(msg);
            return true;
        "#,
    );
    let res = eval.recv().await.unwrap();
    if let Value::String(res) = res {
        res
    } else {
        "".into()
    }
}

pub async fn alert(msg: String) {
    let eval: UseEval = eval(
        r#"
        let msg = await dioxus.recv();
        alert(msg);
        "#,
    );
    eval.send(msg.into()).unwrap();
}



pub async fn node_add_class(node_id: &str,cls: &str) {
    let eval: UseEval = eval(
        r#"
        let data = await dioxus.recv();
        let node = document.querySelector(`#${data.nodeId}`);
        if (!node) {
            console.error('Node not found');
            return false;
        }
        node.classList.add(data.cls);
        node.scrollIntoView({ behavior: 'smooth', block: 'start'});
        "#,
    );
    eval.send({
        let mut map = serde_json::Map::new();
        map.insert("cls".into(), cls.into());
        map.insert("nodeId".into(), node_id.into());
        Value::Object(map)
    }).unwrap();
}

