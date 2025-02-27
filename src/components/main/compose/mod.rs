use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use sir::global_css;
use warp::raygun::Conversation;

use crate::{
    components::{
        main::compose::{messages::Messages, topbar::TopBar, write::Write},
        ui_kit::button::Button,
    },
    Account, Messaging, STATE, LANGUAGE,
};

#[derive(PartialEq, Props)]
pub struct Props {
    account: Account,
    messaging: Messaging,
    conversation: Conversation,
}

pub mod messages;
pub mod msg;
pub mod topbar;
pub mod write;

#[allow(non_snake_case)]
pub fn Compose(cx: Scope<Props>) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let conversation_id = cx.props.conversation.id();
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let warningMessage = l.prerelease_warning.to_string();
    // Load Multipass & Raygun's Atom Ref
    let raygun = cx.props.messaging.clone();

    // Read their values from locks
    let rg = cx.props.messaging.clone();

    let blur = state.read().chat.is_none();
    let text = use_state(&cx, || String::from(""));
    let show_warning = use_state(&cx, || true);

    cx.render(rsx! {
        div {
            class: "compose",
            if blur {
                rsx!(
                    div {
                        class: "blurmask"
                    }
                )
            } else {
                rsx!(
                    TopBar {
                        account: cx.props.account.clone(),
                        conversation: cx.props.conversation.clone(),
                        on_call: move |_| {},
                    }
                )
            },
            (**show_warning).then(|| rsx!(
                div {
                    class: "alpha-warning animate__animated animate__slideInDown",
                    "{warningMessage}",
                    Button {
                        on_pressed: move |_| {
                            show_warning.set(false);
                        },
                        icon: Shape::Check,
                        text: l.user_agrees.to_string(),
                    }
                },
            ))
            div {
                class: "messages-container",
                Messages {
                    account: cx.props.account.clone(),
                    messaging: cx.props.messaging.clone(),
                    conversation: cx.props.conversation.clone(),
                }
            },
            div {
                class: "writer-container",
                Write {
                    on_submit: move |message: String| {
                        text.set(String::from(""));
                        let rg = rg.clone();

                        let text_as_vec = message
                            .split('\n')
                            .filter(|&s| !s.is_empty())
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>();

                        // TODO: We need to wire this message up to display differently
                        // until we confim whether it was successfully sent or failed
                        let _send_message = warp::async_block_in_place_uncheck(rg
                                .write()
                                .send(conversation_id, None, text_as_vec));
                    },
                    on_upload: move |_| {}
                }
            }
        }
    })
}
