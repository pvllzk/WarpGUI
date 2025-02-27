use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::components::ui_kit::icon_button::IconButton;

#[derive(Props)]
pub struct Props<'a> {
    children: Element<'a>,
    on_dismiss: EventHandler<'a, ()>,
    hidden: bool,
}

#[allow(non_snake_case)]
pub fn Popup<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let first_render = use_state(&cx, || true);
    let full = use_state(&cx, || false);
    let modal = use_state(&cx, || false);
    let show_children = use_state(&cx, || true);

    let full_class = match full.get() {
        true => "popup full",
        false => "popup",
    };

    let hidden_class = match cx.props.hidden {
        true => "hidden",
        false => "show",
    };

    let as_modal = match *modal.clone() {
        true => "as-modal",
        false => "",
    };

    // TODO: find out how to make things not animate when the page first loads.
    // we basically need to skip the first render only
    let class = match cx.props.hidden {
        true => match first_render.get() {
            true => "",
            false => "animate__animated animate__slideOutDown",
        },
        false => match first_render.get() {
            true => "",
            false => "animate__animated animate__slideInUp",
        },
    };

    let render = cx.render(rsx!(
        div {
            class: "popup-mask {hidden_class} {as_modal}",
            onclick: move |_| cx.props.on_dismiss.call(()),
            div {
                class: "{full_class} {hidden_class} {class}",
                button {
                    class: "handle",
                    // TODO:
                    // ID:
                    // Title: Allow draging of popup handle to resize
                    // Reporter: Matt Wisniewski
                    // Desc:
                    // We should be able to click and drag the popup and snap the popup to different sizes.
                    onclick: move |evt| {
                        evt.cancel_bubble();
                        full.set(!full.get());
                    }
                }
                div {
                    onclick: move |evt| {
                        evt.cancel_bubble();
                        full.set(true);
                    },
                    div {
                        class: "controls",
                        IconButton {
                            on_pressed: move |_| {
                                modal.set(!modal.clone());
                            },
                            icon: match *modal.clone() {
                                true => Shape::Minus,
                                false => Shape::ArrowsExpand
                            }
                        },
                    },
                    // TODO:
                    // ID:
                    // Title: Popup renders content forever
                    // Reporter: Matt Wisniewski
                    // Desc:
                    // We currently render the children even when the popup is hidden off screen.
                    // We are animating this popup so we need to make sure it's off screen before
                    // we "de-render" thse children, realistically this probably involves a 0.2ms delay
                    // followed by changing the children to render conditionally.
                    // Maybe something like...
                    // cx.spawn({
                    //     async move {
                    //         loop {
                    //             wait_ms(200).await;
                    //             show_children.set(false);
                    //         }
                    //     }
                    // })
                    show_children.then(|| rsx!(cx.props.children.as_ref()))
                }
            }
        }
    ));

    if !cx.props.hidden {
        first_render.set(false);
    }

    render
}
