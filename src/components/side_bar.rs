use std::rc::Rc;

use crate::state::app_state::{HSVState, NextImage, SideBarVisibility};
use dioxus::{prelude::*};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast};
use web_sys::{MouseEvent, console, window};
use crate::dioxusui::GLOBAL_WINDOW_HANDLE;

const HSV_BUTTON_SVG: &str = "<svg fill='#000000' version='1.1' id='Layer_1' xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' viewBox='0 0 472.615 472.615' xml:space='preserve'>
    <g id='SVGRepo_bgCarrier' stroke-width='0'></g><g id='SVGRepo_tracerCarrier' stroke-linecap='round' stroke-linejoin='round'></g><g id='SVGRepo_iconCarrier'> <g> <g>
    <path d='M192.029,226.462v-48.072H76.202v48.072H0v19.692h76.202v48.067h115.827v-48.067h280.587v-19.692H192.029z M172.337,274.529H95.894v-76.447h76.442V274.529z'></path> </g> </g> <g> <g> <path d='M362.49,398.284v-48.067H246.663v48.067H0v19.692h246.663v48.072H362.49v-48.072h110.125v-19.692H362.49z M342.798,446.356h-76.442v-76.447h76.442V446.356z'></path> </g> </g> <g> <g>
    <path d='M265.548,54.635V6.567H149.712v48.067H0v19.692h149.712v48.072h115.837V74.327h207.067V54.635H265.548z M245.856,102.707 h-76.452V26.26h76.452V102.707z'></path> </g> </g> </g></svg>";

const CROP_BUTTON_SVG: &str = "<svg fill='#000000' viewBox='0 0 255.99316 255.99316' id='Flat' xmlns='http://www.w3.org/2000/svg'>
    <g id='SVGRepo_bgCarrier' stroke-width='1.5'></g><g id='SVGRepo_tracerCarrier' stroke-linecap='round' stroke-linejoin='round'></g><g id='SVGRepo_iconCarrier'>
    <path d='M236.00244,192.001a4.0002,4.0002,0,0,1-4,4h-36v36a4,4,0,0,1-8,0v-36h-124a4.0002,4.0002,0,0,1-4-4V68h-36a4,4,0,0,1,0-8h36V24a4,4,0,1,1,8,0V188.001h164A4.00019,4.00019,0,0,1,236.00244,192.001ZM95.99365,68h92.00879v92.001a4,4,0,0,0,8,0V64a4.0002,4.0002,0,0,0-4-4H95.99365a4,4,0,0,0,0,8Z'></path>
    </g></svg>";

#[derive(Clone, Copy)]
enum ResizeType {
    Top,
    Right,
    Bottom,
    Left,
}

// TODO: Make a DraggablePanel component
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
    let mut translation = use_signal(|| (100.0, 100.0));
    let mut resize_type: Signal<Option<ResizeType>> = use_signal(|| None);
    let mut last_resize_x = use_signal(|| 0.0);
    let mut last_resize_y = use_signal(|| 0.0);
    let mut width = use_signal(|| 500.0);
    let mut height = use_signal(|| 200.0);

    let hsv_panel_style = use_memo(move || {
        if hsv_is_visible() {
            format!(
                "display: grid; transform: translate({}px, {}px); width: {}px; height: {}px;",
                translation().0,
                translation().1,
                width(),
                height()
            )
        } else {
            format!("display: none;")
        }
    });

    // mouse move handler for dragging a panel by the title bar
    let drag_handle = move |event: MouseEvent| {
        if is_dragging() {
            let (start_x, start_y) = start_position();
            let dx = event.client_x() as f64 - start_x;
            let dy = event.client_y() as f64 - start_y;
            start_position.set((event.client_x() as f64, event.client_y() as f64));
            let (tx, ty) = translation();
            translation.set((tx + dx, ty + dy));
        }
    };

    // mouse move handler for resizing a panel
    let resize_handle = move |event: MouseEvent| {
        if let Some(resize_dir) = *resize_type.read() {
            match resize_dir {
                ResizeType::Left => {
                    let start_x = last_resize_x();
                    let dx = event.client_x() as f64 - start_x;
                    let new_width = width() - dx;

                    if new_width >= 170.0 && new_width <= 600.0 {
                        last_resize_x.set(event.client_x() as f64);
                        let (tx, ty) = translation();
                        translation.set((tx + dx, ty));
                        width.set(new_width);
                    }
                }
                ResizeType::Right => {
                    let start_x = last_resize_x();
                    let dx = event.client_x() as f64 - start_x;
                    let new_width = width() + dx;

                    if new_width >= 170.0 && new_width <= 600.0 {
                        last_resize_x.set(event.client_x() as f64);
                        width.set(new_width);
                    }
                }
                ResizeType::Top => {
                    let start_y = last_resize_y();
                    let dy = event.client_y() as f64 - start_y;
                    let new_height = height() - dy;

                    if new_height >= 200.0 && new_height <= 300.0 {
                        let (tx, ty) = translation();
                        translation.set((tx, ty + dy));
                        last_resize_y.set(event.client_y() as f64);
                        height.set(new_height);
                    }
                }
                ResizeType::Bottom => {
                    let start_y = last_resize_y();
                    let dy = event.client_y() as f64 - start_y;
                    let new_height = height() + dy;

                    if new_height >= 200.0 && new_height <= 300.0 {
                        last_resize_y.set(event.client_y() as f64);
                        height.set(new_height);
                    }
                }
            }
        }
    };

    if hsv_is_visible() {
        use_hook_with_cleanup(
            move || {
                let move_closure = Rc::new(Closure::wrap(Box::new(drag_handle) as Box<dyn FnMut(_)>));
                let resize_closure = Rc::new(Closure::wrap(Box::new(resize_handle) as Box<dyn FnMut(_)>));

                GLOBAL_WINDOW_HANDLE()
                    .add_event_listener_with_callback(
                        "mousemove",
                        resize_closure.as_ref().as_ref().unchecked_ref()
                    )
                    .unwrap();

                GLOBAL_WINDOW_HANDLE()
                    .add_event_listener_with_callback(
                        "mousemove",
                        move_closure.as_ref().as_ref().unchecked_ref()
                    )
                    .unwrap();

                let mut drag = is_dragging.clone();
                let up_closure = Rc::new(Closure::wrap(Box::new(move |_event: MouseEvent| {
                    drag.set(false);
                    resize_type.set(None);
                }) as Box<dyn FnMut(_)>));

                GLOBAL_WINDOW_HANDLE()
                    .add_event_listener_with_callback(
                        "mouseup",
                        up_closure.as_ref().as_ref().unchecked_ref()
                    )
                    .unwrap();

                (move_closure, up_closure, resize_closure)
            },
            move |(move_closure, up_closure, resize_closure)| {
                if let Some(window) = window() {
                    window
                        .remove_event_listener_with_callback(
                            "mousemove",
                            move_closure.as_ref().as_ref().unchecked_ref()
                        )
                        .unwrap();
                    window
                        .remove_event_listener_with_callback(
                            "mouseup",
                            up_closure.as_ref().as_ref().unchecked_ref()
                        )
                        .unwrap();
                    window
                        .remove_event_listener_with_callback(
                            "mousemove", 
                            resize_closure.as_ref().as_ref().unchecked_ref()
                        ).unwrap();
                }
            },
        );
    }

    rsx! {
        div { id: "hsv-panel-container",
            style: hsv_panel_style(),
            div { id: "left-resize-draggable",
                onmousedown: move |evt| {
                    resize_type.set(Some(ResizeType::Left));
                    last_resize_x.set(evt.client_coordinates().x);
                },
            }
            div { id: "right-resize-draggable",
                onmousedown: move |evt| {
                    resize_type.set(Some(ResizeType::Right));
                    last_resize_x.set(evt.client_coordinates().x);
                }
            }
            div { id: "top-resize-draggable",
                onmousedown: move |evt| {
                    resize_type.set(Some(ResizeType::Top));
                    last_resize_y.set(evt.client_coordinates().y);
                }
            }
            div { id: "bottom-resize-draggable",
                onmousedown: move |evt| {
                    resize_type.set(Some(ResizeType::Bottom));
                    last_resize_y.set(evt.client_coordinates().y);
                }
            }
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
                    p { class: "slider-progress", "{hue_slider_value * 100.0:.2}" }
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
                span { class: "button-svg-container",
                    dangerous_inner_html: HSV_BUTTON_SVG
                }
                span { class: "button-text", "HSV" }
            }
            button { class: "btn",
                span { class: "button-svg-container",
                    dangerous_inner_html: CROP_BUTTON_SVG
                }
                span { class: "button-text", "Crop" }
            }
            button { class: "btn" , "Click me!"}
            button { class: "btn" , "Click me!"}
            button { class: "btn" , "Click me!"}
        }
    }
}
