use crate::components::draggable_panel::DraggablePanel;
use crate::state::app_state::{HSVState, TestPanelVisibility, SideBarVisibility, DragSignal};
use dioxus::prelude::*;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{MouseEvent, console, window};

const ADJUST_BUTTON_SVG: Asset = asset!("/assets/adjust_button.svg");
const CROP_BUTTON_SVG: Asset = asset!("/assets/crop_button.svg");
const RESIZE_BUTTON_SVG: Asset = asset!("/assets/resize_button.svg");
const BRUSH_BUTTON_SVG: Asset = asset!("/assets/brush_button.svg");
const DRAG_BUTTON_SVG: Asset = asset!("/assets/drag_button.svg");

#[component]
pub fn HSVPanel() -> Element {
    let mut hue = use_context::<HSVState>().hue;
    let mut hue_slider_value = use_signal(|| 0.0);
    let mut sat = use_context::<HSVState>().saturation;
    let mut sat_slider_value = use_signal(|| 0.0);
    let mut val = use_context::<HSVState>().value;
    let mut val_slider_value = use_signal(|| 0.0);

    rsx! {
        DraggablePanel {
            title: String::from("HSV"),
            PanelContent:
                rsx! {
                    div { class: "panel-slider-container",
                        p { "HUE" },
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
                    }
                }
        }
    }
}

#[component]
fn TestPanel() -> Element {
    rsx! {
        DraggablePanel {
            title: String::from("Crop"),
            PanelContent:
                rsx! {
                    div { "PLACEHOLDER" }
                }
        }
    }
}

#[component]
pub fn SideBar() -> Element {
    let is_visible = *use_context::<SideBarVisibility>().state.read();
    let mut image_is_draggable = use_context::<DragSignal>().can_drag;
    let sidebar_style = if is_visible {
        "display: flex;"
    } else {
        "display: none;"
    };

    let mut hsv_is_visible = use_context::<HSVState>().panel_visible;
    let mut test_panel_visibility = use_context::<TestPanelVisibility>().visibility;

    rsx! {
        div { class: "sidebar-container", style: sidebar_style,
            button { class: "btn",
                onclick: move |_| {
                    hsv_is_visible.set(!hsv_is_visible());
                },
                img { class: "button-svg-container",
                    src: ADJUST_BUTTON_SVG,
                }
                span { class: "button-text", "HSV" }
            }
            button { class: "btn",
                onclick: move |_| {
                    test_panel_visibility.set(!test_panel_visibility());
                },
                img { class: "button-svg-container",
                    src: CROP_BUTTON_SVG
                }
                span { class: "button-text", "Crop" }
            }
            button { class: "btn",
                img { class: "button-svg-container",
                    src: RESIZE_BUTTON_SVG,
                }
                span { class: "button-text", "Resize" }
            }
            button { class: "btn",
                img { class: "button-svg-container",
                    src: BRUSH_BUTTON_SVG,
                }
                span { class: "button-text", "Brush" }
            }
            button { class: "btn",
                onclick: move |_| {
                    image_is_draggable.set(!image_is_draggable());
                },
                img { class: "button-svg-container", 
                    src: DRAG_BUTTON_SVG,
                }
                span { class: "button-text", "Drag" }
            }
        }
        if hsv_is_visible() {
            HSVPanel {  }
        }
        if test_panel_visibility() {
            TestPanel {  }
        }
    }
}
