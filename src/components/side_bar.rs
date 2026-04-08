use crate::components::draggable_resizeable_panel::DraggableResizeablePanel;
use crate::utils::redraw_metrics::mark_click_to_visible_with_start_ns;
use crate::state::app_state::{CropSignal, HSVState, RedrawKind, ResizeState, SideBarState};
use dioxus::prelude::*;
use image::GenericImageView;
use std::io::Cursor;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as base64_engine;
use web_sys::{console, window};

const ADJUST_BUTTON_SVG: Asset = asset!("/assets/adjust_button.svg");
const CROP_BUTTON_SVG: Asset = asset!("/assets/crop_button.svg");
const RESIZE_BUTTON_SVG: Asset = asset!("/assets/resize_button.svg");
const BRUSH_BUTTON_SVG: Asset = asset!("/assets/brush_button.svg");
const DRAG_BUTTON_SVG: Asset = asset!("/assets/drag_button.svg");

#[component]
pub fn HSVPanel(visibility: Signal<bool>) -> Element {
    let mut hue = use_context::<HSVState>().hue;
    let mut sat = use_context::<HSVState>().saturation;
    let mut val = use_context::<HSVState>().value;

    let initial_hsv = use_signal(|| (hue(), sat(), val()));

    let mut hue_slider_value = use_signal(|| hue() / std::f32::consts::PI);
    let mut sat_slider_value = use_signal(|| sat());
    let mut val_slider_value = use_signal(|| val());

    rsx! {
        DraggableResizeablePanel {
            width: 500.0,
            height: 220.0,
            max_height: 220.0,
            min_height: 220.0,
            title: String::from("HSV"),
            PanelContent:
                rsx! {
                    div { class: "panel-slider-container",
                        p { "HUE" },
                        input {
                            class: "styled-slider",
                            type: "range",
                            min: -1.0,
                            value: "{hue_slider_value()}",
                            max: 1.0,
                            step: 0.001,
                            oninput: move |e| {
                                if let Ok(parsed) = e.value().parse::<f32>() {
                                    let start_ns = (window().unwrap().performance().unwrap().now() * 1_000_000.0) as u64;
                                    mark_click_to_visible_with_start_ns(RedrawKind::HSV, start_ns);
                                    hue_slider_value.set(parsed);
                                    hue.set(parsed * std::f32::consts::PI);
                                }
                            },
                        }
                        p { class: "slider-progress", "{hue_slider_value() * 100.0:.2}" }
                    },
                    div { class: "panel-slider-container",
                        p{ "SAT" },
                        input {
                            class: "styled-slider",
                            type: "range",
                            min: -1.0,
                            value: "{sat_slider_value()}",
                            max: 1.0,
                            step: 0.001,
                            oninput: move |e| {
                                if let Ok(parsed) = e.value().parse::<f32>() {
                                    let start_ns = (window().unwrap().performance().unwrap().now() * 1_000_000.0) as u64;
                                    mark_click_to_visible_with_start_ns(RedrawKind::HSV, start_ns);
                                    sat_slider_value.set(parsed);
                                    sat.set(parsed);
                                }
                            },
                        }
                        p { class: "slider-progress", "{sat_slider_value()}" }
                    },
                    div { class: "panel-slider-container",
                        p{ "VAL" },
                        input {
                            class: "styled-slider",
                            type: "range",
                            min: -1.0,
                            value: "{val_slider_value()}",
                            max: 10.0,
                            step: 0.001,
                            oninput: move |e| {
                                if let Ok(parsed) = e.value().parse::<f32>() {
                                    let start_ns = (window().unwrap().performance().unwrap().now() * 1_000_000.0) as u64;
                                    mark_click_to_visible_with_start_ns(RedrawKind::HSV, start_ns);
                                    val_slider_value.set(parsed);
                                    val.set(parsed);
                                }
                            },
                        }
                        p { class: "slider-progress", "{val_slider_value()}" }
                    }
                    div { class: "button-container",
                        button {
                            class: "btn-styled",
                            onclick: move |_evt| {
                                visibility.set(false);
                            },
                            "Save"
                        }
                        button {
                            class: "btn-styled",
                            onclick: move |_evt| {
                                let (initial_hue, initial_sat, initial_val) = initial_hsv();
                                hue.set(initial_hue);
                                sat.set(initial_sat);
                                val.set(initial_val);
                                hue_slider_value.set(initial_hue / std::f32::consts::PI);
                                sat_slider_value.set(initial_sat);
                                val_slider_value.set(initial_val);
                                visibility.set(false);
                            },
                            "Cancel"
                        }
                    }
                }
        }
    }
}

#[component]
fn ResizePanel() -> Element {
    let mut imwidth = use_context::<ResizeState>().width;
    let widthval = imwidth();
    let mut imheight = use_context::<ResizeState>().height;
    let heightval = imheight();

    rsx! {
        DraggableResizeablePanel {
            width: 200.0,
            height: 200.0,
            min_height: 200.0,
            min_width: 200.0,
            max_width: 200.0,
            max_height: 200.0,
            title: String::from("Resize Image"),
            PanelContent:
                rsx! {
                    input {
                        type: "text",
                        value: "{widthval}",
                        placeholder: "Width",
                        oninput: move |e| {
                            if let Ok(parsed) = e.value().parse::<u32>() {
                                let start_ns = (window().unwrap().performance().unwrap().now() * 1_000_000.0) as u64;
                                mark_click_to_visible_with_start_ns(RedrawKind::Resize, start_ns);
                                imwidth.set(parsed);
                            }
                        }
                    }
                    p { "x" }
                    input {
                        type: "text",
                        value: "{heightval}",
                        placeholder: "Height",
                        oninput: move |e| {
                            if let Ok(parsed) = e.value().parse::<u32>() {
                                let start_ns = (window().unwrap().performance().unwrap().now() * 1_000_000.0) as u64;
                                mark_click_to_visible_with_start_ns(RedrawKind::Resize, start_ns);
                                imheight.set(parsed);
                            }
                        }
                    }
                }
        }
    }
}

#[component]
fn CropPanel(visibility: Signal<bool>) -> Element {
    let mut top = use_context::<CropSignal>().top;
    let mut bottom = use_context::<CropSignal>().bottom;
    let mut left = use_context::<CropSignal>().left;
    let mut right = use_context::<CropSignal>().right;
    let mut top_applied = use_context::<CropSignal>().top_applied;
    let mut bottom_applied = use_context::<CropSignal>().bottom_applied;
    let mut left_applied = use_context::<CropSignal>().left_applied;
    let mut right_applied = use_context::<CropSignal>().right_applied;
    let mut image_vector = use_context::<crate::state::app_state::ImageState>().image_vector;
    let mut base64_vector = use_context::<crate::state::app_state::ImageState>().base64_vector;
    let curr_index = use_context::<crate::state::app_state::ImageState>().curr_image_index;
    let mut image_size = use_context::<crate::state::app_state::ImageState>().img_size;
    let mut image_modified = use_context::<crate::state::app_state::ImageState>().image_modified;
    let mut width_signal = use_context::<ResizeState>().width;
    let mut height_signal = use_context::<ResizeState>().height;
    let perf = window().unwrap().performance().unwrap();
    let nanos = (perf.now() * 1_000_000.0) as u64;
    console::log_1(&format!("CropPanel rendered at {} nanoseconds", nanos).into());
    let nanos_now = (perf.now() * 1_000_000.0) as u64;
    console::log_1(&format!("CropPanel render time: {} nanoseconds", nanos_now - nanos).into());

    let top_val = top();
    let bottom_val = bottom();
    let left_val = left();
    let right_val = right();

    let mut handle_crop = move |_evt: Event<MouseData>| {
        let start_ns = (window().unwrap().performance().unwrap().now() * 1_000_000.0) as u64;
        mark_click_to_visible_with_start_ns(RedrawKind::Crop, start_ns);

        let mut img_vec = image_vector.write();
        if let Some(current_image) = img_vec.get_mut(curr_index()) {
            let (img_width, img_height) = current_image.dimensions();
            let left_px = (left_val * img_width as f32) as u32;
            let top_px = (top_val * img_height as f32) as u32;
            let right_px = (right_val * img_width as f32) as u32;
            let bottom_px = (bottom_val * img_height as f32) as u32;

            let crop_width = img_width.saturating_sub(left_px).saturating_sub(right_px);
            let crop_height = img_height.saturating_sub(top_px).saturating_sub(bottom_px);

            if crop_width > 0 && crop_height > 0 {
                left_applied.set(left_val);
                top_applied.set(top_val);
                right_applied.set(right_val);
                bottom_applied.set(bottom_val);

                let cropped_image = current_image.crop_imm(left_px, top_px, crop_width, crop_height);
                *current_image = cropped_image;

                let rgb_img = current_image.to_rgb8();
                let dynamic_rgb = image::DynamicImage::ImageRgb8(rgb_img);
                let mut cursor = Cursor::new(Vec::new());
                if dynamic_rgb.write_to(&mut cursor, image::ImageFormat::Jpeg).is_ok() {
                    let jpg_bytes = cursor.into_inner();
                    let base64_str = base64_engine.encode(&jpg_bytes);
                    let mut base64_vec = base64_vector.write();
                    if let Some(base64_entry) = base64_vec.get_mut(curr_index()) {
                        *base64_entry = format!("data:image/jpeg;base64,{}", base64_str);
                    }
                }

                image_size.set((crop_width as f64, crop_height as f64));
                width_signal.set(crop_width);
                height_signal.set(crop_height);

                left.set(0.0);
                top.set(0.0);
                right.set(0.0);
                bottom.set(0.0);
                left_applied.set(0.0);
                top_applied.set(0.0);
                right_applied.set(0.0);
                bottom_applied.set(0.0);

                image_modified.set(true);

                console::log_1(&format!("Crop applied - Left: {:.2}, Top: {:.2}, Right: {:.2}, Bottom: {:.2}", left_val, top_val, right_val, bottom_val).into());
            }
        }
        visibility.set(false);
    };

    rsx! {
        DraggableResizeablePanel {
            title: String::from("Crop"),
            PanelContent:
                rsx! {
                    div { class: "button-container",
                        button {
                            class: "btn-styled",
                            onclick: move |_evt| {
                                handle_crop(_evt);
                            },
                            "Crop!"
                        }
                        button {
                            class: "btn-styled",
                            onclick: move |_evt| {
                                visibility.set(false);
                            },
                            "Cancel"
                        }
                    }
                }
        }
    }
}

#[component]
pub fn SideBar() -> Element {
    let is_visible = *use_context::<SideBarState>().sidebar_is_visible.read();
    let mut image_is_draggable = use_context::<SideBarState>().is_dragging;
    let sidebar_style = if is_visible {
        "display: flex;"
    } else {
        "display: none;"
    };

    let mut hsv_is_visible = use_context::<HSVState>().panel_visible;
    let mut resize_panel_visibility = use_context::<ResizeState>().panel_visible;
    let mut crop_panel_visibility = use_context::<SideBarState>().is_cropping;

    rsx! {
        div { class: "sidebar-container", style: sidebar_style,
            button { class: if hsv_is_visible() { "btn on" } else { "btn" },
                onclick: move |_| {
                    hsv_is_visible.set(!hsv_is_visible());
                },
                img { class: "button-svg-container",
                    src: ADJUST_BUTTON_SVG,
                }
                span { class: "button-text", "HSV" }
            }
            button { class: if crop_panel_visibility() { "btn on" } else { "btn" },
                onclick: move |_| {
                    crop_panel_visibility.set(!crop_panel_visibility());
                },
                img { class: "button-svg-container",
                    src: CROP_BUTTON_SVG
                }
                span { class: "button-text", "Crop" }
            }
            button { class: "btn",
            onclick: move |_| {
                    resize_panel_visibility.set(!resize_panel_visibility());
                },
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
            button { class: if image_is_draggable() { "btn on" } else { "btn" },
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
            HSVPanel { 
                visibility: hsv_is_visible,
             }
        }
        if crop_panel_visibility() {
            CropPanel { 
                visibility: crop_panel_visibility,
            }
        }
        if resize_panel_visibility() {
            ResizePanel {  }
        }
    }
}
