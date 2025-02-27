use std::time::Duration;

use crate::{
    components::main::{compose::Compose, settings::Settings}, main::sidebar::Sidebar, Account, Messaging, STATE,
};
use dioxus::prelude::*;
use warp::raygun::Conversation;

pub mod compose;
pub mod sidebar;
pub mod friends;
pub mod profile;
pub mod settings;

#[derive(Props, PartialEq)]
pub struct Prop {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Main(cx: Scope<Prop>) -> Element {
    let state = use_atom_ref(&cx, STATE);

    // Read their values from locks
    let rg = cx.props.messaging.clone();

    let st = state.clone();
    cx.spawn(async move {
        loop {
            if let Ok(list) = rg.read().list_conversations().await {
                if !list.is_empty() && st.read().chats != list {
                    st.write().chats = list;
                }
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    });

    let conversation = match state.read().chat.clone() {
        Some(c) => c,
        None => Conversation::default(),
    };

    cx.render(rsx! {
        div {
            class: "main",
            // Settings {
            //     show: true,
            //     on_hide: move |_| {},
            // },
            Sidebar {
                messaging: cx.props.messaging.clone(),
                account: cx.props.account.clone()
            },
            Compose {
                account: cx.props.account.clone(),
                messaging: cx.props.messaging.clone(),
                conversation: conversation
            },
        }
    })
}
