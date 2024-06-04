use dioxus::prelude::*;

use crate::views::{note_list::custom_sub::CustomSubscription, NoteList};

#[component]
pub fn Subscription(name: String) -> Element {

    let handle_save = move |value: CustomSub| {
        spawn(async move {
            let old_name = {
                let sub_current_lock = sub_current();
                sub_current_lock.name.clone()
            };
            let edit_value = value.clone();
            tracing::info!("Update: {:?}", edit_value);

            match cb_database_db()
                .update_custom_sub(old_name.clone(), edit_value.clone())
                .await
            {
                Ok(_) => {
                    let edit_name = edit_value.name.clone();

                    {
                        sub_current.set(value.clone());
                    }
                    let index: usize = *sub_index.read();
                    {
                        let mut subs: Write<_, UnsyncStorage> = all_sub.write();
                        subs[index] = sub_current().clone();
                    }

                    if old_name != edit_name {
                        navigator().replace(Route::Subscription { name: edit_name });
                    }
                    tracing::info!("Update success: wait for reload");
                }
                Err(e) => {
                    tracing::error!("Update error: {:?}", e);
                }
            }
        });
    };

    let handle_reload = move |_: CustomSub| {
        //todo
        // index += 1;
        // sub_current.set(value);
    };
    rsx! {
        section {
            class: "subscription",
            NoteList{
                name: name,
            }
            CustomSubscription {
                on_save: handle_save,
                on_reload: handle_reload,
                subscription: sub_current.read().clone(),
            }
        }
    }
}
