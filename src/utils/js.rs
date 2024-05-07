use dioxus::prelude::*;

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
