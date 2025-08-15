use crate::state::app_state::{HSVState, NextImage, SideBarVisibility};
use dioxus::{html::geometry::euclid::Translation2D, prelude::*};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{console, window, MouseEvent};

const HSV_BUTTON_SVG: &str = "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='size-6'>
  <path stroke-linecap='round' stroke-linejoin='round' d='M10.5 6h9.75M10.5 6a1.5 1.5 0 1 1-3 0m3 0a1.5 1.5 0 1 0-3 0M3.75 6H7.5m3 12h9.75m-9.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-3.75 0H7.5m9-6h3.75m-3.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-9.75 0h9.75' />
</svg>";

#[component]
pub fn HSVPanel() -> Element {
    let mut hsv_is_visible = use_context::<HSVState>().panel_visible;
    let mut hue = use_context::<HSVState>().hue;
    let mut hvalue = hue();
    let mut sat = use_context::<HSVState>().saturation;
    let mut val = use_context::<HSVState>().value;
    let mut is_dragging = use_signal(|| false);
    let mut start_position = use_signal(|| (0.0, 0.0));
    let mut translation = use_signal(|| (0.0, 0.0));

    let mut hsv_panel_style = use_memo( move ||
        if hsv_is_visible() {
            format!("display: grid; transform: translate({}px, {}px);", translation().0, translation().1)
        } else {
            format!("display: none;")
        }
    );

    rsx! {
        div { class: "hsv-panel-container",
            style: hsv_panel_style(),
            div { class: "panel-title",
                onmousedown: move |evt| {
                    is_dragging.set(true);
                    start_position.set((evt.client_coordinates().x, evt.client_coordinates().y));
                    let window = window().unwrap();

                    let mouse_move_cb = Closure::wrap(Box::new(move |event: MouseEvent| {
                        if is_dragging() {
                            let dx = event.client_x() as f64 - start_position().0;
                            let dy = event.client_y() as f64 - start_position().1;
                            start_position.set((event.client_x() as f64, event.client_y() as f64));
                            let (tx, ty) = translation();
                            translation.set((tx + dx, ty + dy));
                        }
                    }) as Box<dyn FnMut(_)>);
                    window
                        .add_event_listener_with_callback("mousemove", mouse_move_cb.as_ref().unchecked_ref())
                        .unwrap();
                    
                    mouse_move_cb.forget();

                    let mouse_up_cb = Closure::wrap(Box::new(move |_event: MouseEvent| {
                        is_dragging.set(false);
                    }) as Box<dyn FnMut(_)>);
                    window
                        .add_event_listener_with_callback("mouseup", mouse_up_cb.as_ref().unchecked_ref())
                        .unwrap();

                    mouse_up_cb.forget();
                },
                onmouseup: move |_| {
                    is_dragging.set(false);
                },
                p { "HSV" },
            },
            div { class: "panel-content",
                div { class: "hsv-slider",
                    p{ "HUE" },
                    input {
                        id: "hsv-h",
                        type: "range",
                        min: 0.0,
                        value:"{hue}" ,
                        max: 360.0,
                        oninput: move |e|{
                            if let Ok(parsed) = e.value().parse::<f64>() {
                                hue.set(parsed);
                            }
                        },
                    }
                },
                div { class: "hsv-slider",
                    p{ "SAT" },
                    input {
                        id: "hsv-s",
                        type: "range",
                        min: 0.0,
                        value:"{sat}",
                        max: 1.0,
                        oninput: move |e| {
                            if let Ok(parsed) = e.value().parse::<f64>() {
                                sat.set(parsed);
                            }
                        },
                    }
                },
                div { class: "hsv-slider",
                    p{ "VAL" },
                    input {
                        id: "hsv-v",
                        type: "range",
                        min: 0.0,
                        value:"{val}" ,
                        max: 1.0,
                        oninput: move |e| {
                            if let Ok(parsed) = e.value().parse::<f64>() {
                                val.set(parsed);
                            }
                        },
                    }
                },
            }
        }
    }
}

#[component]
pub fn SideBar() -> Element {
    let is_visible = *use_context::<SideBarVisibility>().state.read();
    let nowcount = *use_context::<NextImage>().count.read();
    let sidebar_style = if is_visible {
        "display: flex;"
    } else {
        "display: none;"
    };
    let mut hsv_is_visible = use_context::<HSVState>().panel_visible;

    rsx! {
        div { class: "sidebar-container", style: sidebar_style,
            button { class: "btn",
                onclick: move |_| {
                    hsv_is_visible.set(!hsv_is_visible());
                },
                div { class: "button-contents",
                    div { class: "button-svg-container",
                        dangerous_inner_html: HSV_BUTTON_SVG
                    }
                    p { class: "button-text", "HSV" }
                }
            }
            button { class: "btn" , "Click me!"}
            button { class: "btn" , "Click me!"}
            button { class: "btn" , "Click me!"}
            button { class: "btn" , "Click me!"}
        }
    }
}
