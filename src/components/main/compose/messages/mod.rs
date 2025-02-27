use crate::{components::main::compose::msg::Msg, Account, Messaging};
use dioxus::prelude::*;
use warp::{
    raygun::{Conversation, MessageOptions},
};

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messaging: Messaging,
    conversation: Conversation,
}

#[allow(non_snake_case)]
pub fn Messages(cx: Scope<Props>) -> Element {
    let conversation_id = cx.props.conversation.id();

    // let _show_skeleton = conversation_id == Uuid::default();

    // Read their values from locks
    let rg = cx.props.messaging.clone();
    let mp = cx.props.account.clone();

    let messages = use_future(&cx, (), |_| async move {
        rg.read()
            .get_messages(conversation_id, MessageOptions::default())
            .await
    });

    //Note: We will just unwrap for now though we need to
    //      handle the error properly if there is ever one when
    //      getting own identity
    let ident = mp.read().get_own_identity().unwrap();

    let element = cx.render(match messages.value() {
        Some(Ok(list)) => {
            let mut prev_sender = "".to_string();
            rsx! {
                div {
                    class: "messages",
                    list.iter().rev().map(|message|{
                        let msg_sender = message.sender().to_string();
                        let i = ident.did_key().to_string();
                        let remote = i != msg_sender;
                        let last = prev_sender != msg_sender;
                        let middle = prev_sender == msg_sender;
                        let first = false;

                        prev_sender = message.sender().to_string();

                        rsx!(
                            Msg {
                                message: message.clone(),
                                remote: remote,
                                last: last,
                                first: first,
                                middle: middle,
                            }
                        )
                    })
                }
            }
        }
        Some(Err(_e)) => {
            rsx!(div {})
        }
        None => rsx!(div {}),
    });

    messages.restart();

    element
}
