use std::rc::Rc;

use crate::state::app_state::{HSVState, NextImage, SideBarVisibility};
use dioxus::{html::geometry::euclid::Translation2D, prelude::*};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{MouseEvent, console, window};

const HSV_BUTTON_SVG: &str = "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='size-6'>
  <path stroke-linecap='round' stroke-linejoin='round' d='M10.5 6h9.75M10.5 6a1.5 1.5 0 1 1-3 0m3 0a1.5 1.5 0 1 0-3 0M3.75 6H7.5m3 12h9.75m-9.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-3.75 0H7.5m9-6h3.75m-3.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-9.75 0h9.75' />
</svg>";

#[component]
pub fn HSVPanel() -> Element {
    let mut hsv_is_visible = use_context::<HSVState>().panel_visible;
    let mut hue = use_context::<HSVState>().hue;
    let mut hue_slider_value = use_signal(|| 0.0);
    let mut sat = use_context::<HSVState>().saturation;
    let mut sat_slider_value = use_signal(|| 0.0);
    let mut val = use_context::<HSVState>().value;
    let mut val_slider_value = use_signal(|| 0.0);
    let mut is_dragging = use_signal(|| false);
    let mut start_position = use_signal(|| (0.0, 0.0));
    let mut translation = use_signal(|| (0.0, 0.0));

    let mut hsv_panel_style = use_memo(move || {
        if hsv_is_visible() {
            format!(
                "display: grid; transform: translate({}px, {}px);",
                translation().0,
                translation().1
            )
        } else {
            format!("display: none;")
        }
    });

    if hsv_is_visible() {
        use_hook_with_cleanup(
            move || {
                let window = window().unwrap();

                let drag = is_dragging.clone();
                let move_closure = Rc::new(Closure::wrap(Box::new(move |event: MouseEvent| {
                    if drag() {
                        let (start_x, start_y) = start_position();
                        let dx = event.client_x() as f64 - start_x;
                        let dy = event.client_y() as f64 - start_y;
                        start_position.set((event.client_x() as f64, event.client_y() as f64));
                        let (tx, ty) = translation();
                        translation.set((tx + dx, ty + dy));
                    }
                }) as Box<dyn FnMut(_)>));

                window
                    .add_event_listener_with_callback(
                        "mousemove",
                        move_closure.as_ref().as_ref().unchecked_ref(),
                    )
                    .unwrap();

                let mut drag = is_dragging.clone();
                let up_closure = Rc::new(Closure::wrap(Box::new(move |_event: MouseEvent| {
                    drag.set(false);
                }) as Box<dyn FnMut(_)>));

                window
                    .add_event_listener_with_callback(
                        "mouseup",
                        up_closure.as_ref().as_ref().unchecked_ref(),
                    )
                    .unwrap();

                (move_closure, up_closure)
            },
            move |(move_closure, up_closure)| {
                if let Some(window) = window() {
                    window
                        .remove_event_listener_with_callback(
                            "mousemove",
                            move_closure.as_ref().as_ref().unchecked_ref(),
                        )
                        .unwrap();
                    window
                        .remove_event_listener_with_callback(
                            "mouseup",
                            up_closure.as_ref().as_ref().unchecked_ref(),
                        )
                        .unwrap();
                }
            },
        );
    }

    rsx! {
        div { class: "hsv-panel-container",
            style: hsv_panel_style(),
            div { class: "panel-title",
                onmousedown: move |evt| {
                    is_dragging.set(true);
                    start_position.set((evt.client_coordinates().x, evt.client_coordinates().y));
                },
                onmouseup: move |_| {
                    is_dragging.set(false);
                },
                p { "HSV" },
            },
            div { class: "panel-content",
                div { class: "panel-slider-container",
                    p{ "HUE" },
                    input {
                        class: "panel-slider",
                        type: "range",
                        min: -1.0,
                        value:"{hue_slider_value}" ,
                        max: 1.0,
                        step: 0.001,
                        oninput: move |e|{
                            if let Ok(parsed) = e.value().parse::<f32>() {
                                hue_slider_value.set(parsed);
                                hue.set(parsed * std::f32::consts::PI);
                            }
                        },
                    }
                    p { class: "slider-progress", "{hue_slider_value}" }
                },
                div { class: "panel-slider-container",
                    p{ "SAT" },
                    input {
                        class: "panel-slider",
                        type: "range",
                        min: -1.0,
                        value:"{sat_slider_value}",
                        max: 1.0,
                        step: 0.001,
                        oninput: move |e| {
                            if let Ok(parsed) = e.value().parse::<f32>() {
                                sat_slider_value.set(parsed);
                                sat.set(parsed);
                            }
                        },
                    }
                    p { class: "slider-progress", "{sat_slider_value}" }
                },
                div { class: "panel-slider-container",
                    p{ "VAL" },
                    input {
                        class: "panel-slider",
                        type: "range",
                        min: -1.0,
                        value:"{val_slider_value}" ,
                        max: 10.0,
                        step: 0.001,
                        oninput: move |e| {
                            if let Ok(parsed) = e.value().parse::<f32>() {
                                val_slider_value.set(parsed);
                                val.set(parsed);
                            }
                        },
                    }
                    p { class: "slider-progress", "{val_slider_value}" }
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
