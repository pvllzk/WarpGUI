use dioxus::{prelude::*, events::KeyCode};
use dioxus_heroicons::outline::Shape;
use sir::{css, global_css};
use warp::tesseract::Tesseract;

use crate::{components::ui_kit::{pin::Pin, icon_button::{IconButton, self}}, TESSERACT, LANGUAGE};

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct UnlockProps {
    pin: String,
}

#[allow(non_snake_case)]
pub fn Unlock(cx: Scope<UnlockProps>) -> Element {
    let tess = use_atom_ref(&cx, TESSERACT);
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let l2 = l.clone();

    global_css! {"
        .unlock {
            display: flex;
            justify-content: center;
            align-items: center;
            text-align: center;
            height: 80%;

            .container {
                max-width: 350px;position: relative;
                .invis-input {
                    position: fixed;
                    top: 0;
                    left: 0;
                    right: 0;
                    bottom: 0;
                    z-index: 2;
                    cursor: default;
                    opacity: 0;
                    font-size: 0;
                }
                .confirm-button {
                    position: absolute;
                    right: -80px;
                    bottom: -12px;
                    z-index: 3;
                }
            }

            .login-actions {
                position: fixed;
                bottom: 2rem;
                right: 2rem;
                left: 2rem;
                max-height: 40px;
                display: inline-block;
                text-align: right;
                z-index: 3;
                div {
                    padding: 0.2rem 0.6rem;
                    &:last-of-type {
                        padding-right: 0;
                    }
                }
            }

        }
    "}

    let pin = use_state(&cx, || String::from(""));
    let error = use_state(&cx, || String::from(""));
    let error_class = if error.is_empty() {
        css!("opacity: 0")
    } else {
        "error_text"
    };
    let valid_pin = pin.len() >= 4;
    // Used later to try to unlock as we type a valid pin automatically much like modern phones and operating systems.
    let tesseract_exists = Tesseract::from_file(".warp_datastore").is_ok();

    cx.render(rsx!{
        div {
            class: "unlock",
            div {
                class: "container",
                h2 {
                    "{l.create_pin}",
                },
                label {
                    tesseract_exists.then(|| l.enter_your_pin.clone()),
                    (!tesseract_exists).then(|| l.choose_a_pin.clone()),
                },
                div {
                    class: "m-bottom-xl",
                },
                div {
                    style: "position: relative;",
                    Pin {
                        pin: pin.as_bytes().to_vec(),
                        error: !error.is_empty()
                    },
                    valid_pin.then(||
                        rsx! {
                            span {
                                class: "confirm-button",
                                IconButton {
                                    icon: if error.is_empty() {
                                        Shape::Check
                                    } else {
                                        Shape::X
                                    }
                                    onclick: move |_| {
                                        match tess.write().unlock(pin.as_bytes()) {
                                            Ok(_) => use_router(&cx).push_route("/auth", None, None),
                                            Err(_) => error.set(l2.invalid_pin.clone()),
                                        }
                                    },
                                },
                            },
                        }
                    ),
                }
                div {
                    class: "m-bottom-xl",
                },
                p {
                    class: "{error_class}",
                    "{error}　"
                },
                input {
                    class: "invis-input",
                    value: "{pin}",
                    autofocus: "true",
                    onkeypress: move |evt| {
                        error.set(String::from(""));
                        if evt.key_code == KeyCode::Enter {
                            if pin.len() < 4 {
                                error.set(String::from(l.short_pin.clone()));
                            } else {
                                match tess.write().unlock(pin.as_bytes()) {
                                    Ok(_) => use_router(&cx).push_route("/auth", None, None),
                                    Err(_) => error.set(String::from(l.invalid_pin.clone())),
                                }
                            }
                        }
                    },
                    oninput: move |evt| {
                        pin.set(evt.value.clone());
                        // If tesseract exists, we can try to unlock as we type to save time
                        // We can ignore the error though since we're doing this without the users command
                        if pin.len() >= 4 && tesseract_exists {
                            match tess.write().unlock(pin.as_bytes()) {
                                Ok(_) => use_router(&cx).push_route("/auth", None, None),
                                Err(_) => {},
                            }
                        }
                        // If the pin entered is longer than the allowed limit, we'll just set it back to the max.
                        if pin.len() > 6 {
                            pin.set(evt.value[..6].to_string());
                        }
                    },
                }
            },
            div {
                class: "login-actions",
                IconButton {
                    icon: Shape::User,
                    disabled: true,
                    state: icon_button::State::Secondary,
                    onclick: move |_| {},
                },
                IconButton {
                    icon: Shape::GlobeAlt,
                    disabled: true,
                    state: icon_button::State::Secondary,
                    onclick: move |_| {},
                },
            }
        },
    })
}