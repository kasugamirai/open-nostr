/// This is the note content component
/// 
/// Props 
/// 
/// 
/// 



#[derive(Properties, Clone, Debug, Props)]
pub struct NoteContent {
    #[props(default = false)]
    is_repost: bool,
    #[props(default = false)]
    is_reply: bool,
    event: Event,
    children: Element,
}

#[component]
pub fn NoteContent(props: NoteContent) -> Element {
    

    rsx! {
        
        div {
            class: "note-content font-size-16 word-wrap lh-26",
            onclick: move |_| {
                handle_nav(Route::NoteDetail { 
                    sub: urlencoding::encode(&props.sub_name.clone()).to_string(), 
                    id: event.read().id().to_string(), });
            },
            if is_reply() && !props.is_tree {
                Reply {
                    event: event.read().clone(),
                    sub_name: props.sub_name.clone(),
                    relay_name: props.relay_name.clone().unwrap_or("default".to_string()),
                }
            }
            {element}
        }
    }
}