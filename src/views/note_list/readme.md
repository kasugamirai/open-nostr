
# show 1 normal
note_wrap -> 
    avatar + more 
    note_content
        format_content(xxxxxxxx)
    actions

# show 2 Reply
note_wrap -> 
    avatar + more 
    Rely
    note_content
        format_content(xxxxxxxx)
    actions

# show 3 Repost 
note_wrap -> 
    RepostAvatar + RoostAvatar(RootNickname + RootAvatar) + more 
    note_content
        format_content(xxxxxxxx)
    actions (Repost actions)

# show 4 Quote
note_wrap -> 
    avatar + more
    note_content
        format_content(
            xxxx
            Quote
            xxxx
            Quote
            xxxx
        )
    actions



            // detail modal
            // if *show_detail.read() { 
            //     div {
            //         style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background-color: rgba(0, 0, 0, 0.5); z-index: 99999999;",
            //         div {
            //             style: "width: 50%; height: 60%; max-width: 900px; background-color: #fff; position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); padding: 20px; border-radius: 10px;",
            //             button {
            //                 class: "btn-icon remove",
            //                 style: "position: absolute; top: -12px; left: -12px;",
            //                 onclick: move |_| {
            //                     show_detail.set(false);
            //                 },
            //                 dangerous_inner_html: "{FALSE}",
            //             }
            //             pre {
            //                 style: "height: 100%; overflow-y: auto; font-size: 16px;",
            //                 "{detail}"
            //             }
            //         }
            //     }
            // }