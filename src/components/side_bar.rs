use std::rc::Rc;

use crate::state::app_state::{HSVState, NextImage, SideBarVisibility};
use dioxus::{html::geometry::euclid::Translation2D, prelude::*};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{MouseEvent, console, window};

const HSV_BUTTON_SVG: &str = "<svg fill='#000000' version='1.1' id='Layer_1' xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' viewBox='0 0 472.615 472.615' xml:space='preserve'>
    <g id='SVGRepo_bgCarrier' stroke-width='0'></g><g id='SVGRepo_tracerCarrier' stroke-linecap='round' stroke-linejoin='round'></g><g id='SVGRepo_iconCarrier'> <g> <g>
    <path d='M192.029,226.462v-48.072H76.202v48.072H0v19.692h76.202v48.067h115.827v-48.067h280.587v-19.692H192.029z M172.337,274.529H95.894v-76.447h76.442V274.529z'></path> </g> </g> <g> <g> <path d='M362.49,398.284v-48.067H246.663v48.067H0v19.692h246.663v48.072H362.49v-48.072h110.125v-19.692H362.49z M342.798,446.356h-76.442v-76.447h76.442V446.356z'></path> </g> </g> <g> <g>
    <path d='M265.548,54.635V6.567H149.712v48.067H0v19.692h149.712v48.072h115.837V74.327h207.067V54.635H265.548z M245.856,102.707 h-76.452V26.26h76.452V102.707z'></path> </g> </g> </g></svg>";

const CROP_BUTTON_SVG: &str = "<svg fill='#000000' viewBox='0 0 255.99316 255.99316' id='Flat' xmlns='http://www.w3.org/2000/svg'>
    <g id='SVGRepo_bgCarrier' stroke-width='1.5'></g><g id='SVGRepo_tracerCarrier' stroke-linecap='round' stroke-linejoin='round'></g><g id='SVGRepo_iconCarrier'>
    <path d='M236.00244,192.001a4.0002,4.0002,0,0,1-4,4h-36v36a4,4,0,0,1-8,0v-36h-124a4.0002,4.0002,0,0,1-4-4V68h-36a4,4,0,0,1,0-8h36V24a4,4,0,1,1,8,0V188.001h164A4.00019,4.00019,0,0,1,236.00244,192.001ZM95.99365,68h92.00879v92.001a4,4,0,0,0,8,0V64a4.0002,4.0002,0,0,0-4-4H95.99365a4,4,0,0,0,0,8Z'></path>
    </g></svg>";

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
            button { class: "btn",
                div { class: "button-contents",
                    div { class: "button-svg-container",
                        dangerous_inner_html: CROP_BUTTON_SVG
                    }
                    p { class: "button-text", "Crop" }
                }
            }
            button { class: "btn" , "Click me!"}
            button { class: "btn" , "Click me!"}
            button { class: "btn" , "Click me!"}
        }
    }
}
