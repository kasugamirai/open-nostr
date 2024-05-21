
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
