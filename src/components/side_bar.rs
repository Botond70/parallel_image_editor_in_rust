use crate::components::draggable_resizeable_panel::DraggableResizeablePanel;
use crate::utils::redraw_metrics::mark_click_to_visible_with_start_ns;
use crate::state::app_state::{BlurDirection, BlurMode, BlurState, CropSignal, FilterMenuState, HSVState, RedrawKind, ResizeState, SideBarState};
use dioxus::prelude::*;
use image::GenericImageView;
use std::io::Cursor;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as base64_engine;
use web_sys::{console, window};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

const ADJUST_BUTTON_SVG: Asset = asset!("/assets/adjust_button.svg");
const CROP_BUTTON_SVG: Asset = asset!("/assets/crop_button.svg");
const RESIZE_BUTTON_SVG: Asset = asset!("/assets/resize_button.svg");
const FILTER_BUTTON_SVG: Asset = asset!("/assets/filter_button.svg");
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

    let mut blur_state = use_context::<BlurState>();
    let mut original_blur_size = use_signal(|| (blur_state.window_size)());
    let mut blur_restore_timeout = use_signal(|| None::<i32>);
    let mut disable_blur_temporarily = move || {
        if original_blur_size() == 0 {
            original_blur_size.set((blur_state.window_size)());
        }
        blur_state.window_size.set(1); // 1 == no blur in implementation

        if let Some(timeout_id) = blur_restore_timeout() {
            let window = web_sys::window().unwrap();
            window.clear_timeout_with_handle(timeout_id);
        }
        // Schedule restoration of original blur after 250ms
        let closure = Closure::wrap(Box::new(move || {
            blur_state.window_size.set(original_blur_size());
            original_blur_size.set(0);
            blur_restore_timeout.set(None);
        }) as Box<dyn FnMut()>);
        let window = web_sys::window().unwrap();
        let timeout_id = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                250,
            )
            .unwrap();
        blur_restore_timeout.set(Some(timeout_id));
        closure.forget();
    };

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
                                    disable_blur_temporarily();
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
                                    disable_blur_temporarily();
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
                                    disable_blur_temporarily();
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
fn ResizePanel(visibility: Signal<bool>) -> Element {
    let mut imwidth = use_context::<ResizeState>().width;
    let mut imheight = use_context::<ResizeState>().height;
    let initial_size = use_signal(|| (imwidth(), imheight()));
    let mut draft_width = use_signal(|| imwidth());
    let mut draft_height = use_signal(|| imheight());

    rsx! {
        DraggableResizeablePanel {
            width: 230.0,
            height: 180.0,
            min_height: 180.0,
            min_width: 230.0,
            max_width: 230.0,
            max_height: 180.0,
            title: String::from("Resize Image"),
            PanelContent:
                rsx! {
                    div { class: "resize-input-row",
                        input {
                            class: "styled-input",
                            r#type: "text",
                            inputmode: "numeric",
                            value: "{draft_width()}",
                            placeholder: "Width",
                            oninput: move |e| {
                                if let Ok(parsed) = e.value().parse::<u32>() {
                                    draft_width.set(parsed);
                                }
                            }
                        }
                        p { "x" }
                        input {
                            class: "styled-input",
                            r#type: "text",
                            inputmode: "numeric",
                            value: "{draft_height()}",
                            placeholder: "Height",
                            oninput: move |e| {
                                if let Ok(parsed) = e.value().parse::<u32>() {
                                    draft_height.set(parsed);
                                }
                            }
                        }
                    }
                    div { class: "button-container",
                        button {
                            class: "btn-styled",
                            onclick: move |_evt| {
                                let start_ns = (window().unwrap().performance().unwrap().now() * 1_000_000.0) as u64;
                                mark_click_to_visible_with_start_ns(RedrawKind::Resize, start_ns);
                                imwidth.set(draft_width());
                                imheight.set(draft_height());
                                visibility.set(false);
                            },
                            "Save"
                        }
                        button {
                            class: "btn-styled",
                            onclick: move |_evt| {
                                let (w, h) = initial_size();
                                draft_width.set(w);
                                draft_height.set(h);
                                imwidth.set(w);
                                imheight.set(h);
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
fn BlurPanel() -> Element {
    let mut blur_state = use_context::<BlurState>();
    let mut blur_mode = blur_state.mode;
    let mut blur_window_size = blur_state.window_size;
    let mut blur_direction = blur_state.direction;

    let mut initial_mode = use_signal(|| blur_mode());
    let mut initial_size = use_signal(|| blur_window_size());
    let mut initial_direction = use_signal(|| blur_direction());

    let mut draft_mode = use_signal(|| initial_mode());
    let mut draft_size = use_signal(|| initial_size());
    let mut draft_direction = use_signal(|| initial_direction());

    let mut mode_value = draft_mode();
    let mut size_value = draft_size();
    let mut direction_value = draft_direction();

    rsx! {
        DraggableResizeablePanel {
            width: 320.0,
            height: 260.0,
            min_height: 260.0,
            min_width: 280.0,
            max_height: 380.0,
            title: String::from("Blur"),
            PanelContent:
                rsx! {
                    div { class: "panel-button-row",
                        button { class: if matches!(mode_value, BlurMode::Gaussian) { "btn on" } else { "btn" },
                            onclick: move |_| {
                                draft_mode.set(BlurMode::Gaussian);
                            },
                            "Gaussian"
                        }
                        button { class: if matches!(mode_value, BlurMode::Box) { "btn on" } else { "btn" },
                            onclick: move |_| {
                                draft_mode.set(BlurMode::Box);
                            },
                            "Box"
                        }
                    }
                    div { class: "panel-slider-container",
                        p { "Window Size" }
                        input {
                            class: "styled-input",
                            type: "number",
                            min: "1",
                            value: "{size_value}",
                            oninput: move |e| {
                                if let Ok(parsed) = e.value().parse::<u32>() {
                                    draft_size.set(parsed.max(1));
                                }
                            }
                        }
                        p { class: "slider-progress", "{size_value}" }
                    }
                    div { class: "panel-button-row",
                        button { class: if matches!(direction_value, BlurDirection::Omnidirectional) { "btn on" } else { "btn" },
                            onclick: move |_| { draft_direction.set(BlurDirection::Omnidirectional); },
                            "Omnidirectional"
                        }
                        button { class: if matches!(direction_value, BlurDirection::Horizontal) { "btn on" } else { "btn" },
                            onclick: move |_| { draft_direction.set(BlurDirection::Horizontal); },
                            "Horizontal"
                        }
                        button { class: if matches!(direction_value, BlurDirection::Vertical) { "btn on" } else { "btn" },
                            onclick: move |_| { draft_direction.set(BlurDirection::Vertical); },
                            "Vertical"
                        }
                    }
                    div { class: "button-container",
                        button {
                            class: "btn-styled",
                            onclick: move |_| {
                                blur_mode.set(draft_mode());
                                blur_window_size.set(draft_size());
                                blur_direction.set(draft_direction());
                                blur_state.panel_visible.set(false);
                            },
                            "Save"
                        }
                        button {
                            class: "btn-styled",
                            onclick: move |_| {
                                draft_mode.set(initial_mode());
                                draft_size.set(initial_size());
                                draft_direction.set(initial_direction());
                                blur_state.panel_visible.set(false);
                            },
                            "Cancel"
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
fn FilterMenu() -> Element {
    let mut filter_menu_state = use_context::<FilterMenuState>();
    let mut menu_visible = filter_menu_state.menu_visible;
    let mut blur_state = use_context::<BlurState>();
    let mut blur_panel_visible = blur_state.panel_visible;

    rsx! {
        div { class: "filter-menu",
            button {
                class: "filter-menu-item",
                onclick: move |_| {
                    blur_panel_visible.set(true);
                    menu_visible.set(false);
                },
                "Blur"
            }
            button {
                class: "filter-menu-item",
                onclick: move |_| {
                    // TODO: Implement sharpen filter
                    menu_visible.set(false);
                },
                "Sharpen"
            }
            button {
                class: "filter-menu-item",
                onclick: move |_| {
                    // TODO: Implement edge detection filter
                    menu_visible.set(false);
                },
                "Edge Detection"
            }
            button {
                class: "filter-menu-item",
                onclick: move |_| {
                    // TODO: Implement color adjustment filter
                    menu_visible.set(false);
                },
                "Color Adjustment"
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
    let mut blur_panel_visibility = use_context::<BlurState>().panel_visible;
    let mut crop_panel_visibility = use_context::<SideBarState>().is_cropping;
    let filter_menu_state = use_context::<FilterMenuState>();
    let mut filter_menu_visible = filter_menu_state.menu_visible;

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
            button { class: if resize_panel_visibility() { "btn on" } else { "btn" },
            onclick: move |_| {
                    resize_panel_visibility.set(!resize_panel_visibility());
                },
                img { class: "button-svg-container",
                    src: RESIZE_BUTTON_SVG,
                }
                span { class: "button-text", "Resize" }
            }
            div { class: "filter-button-container",
                button { class: if blur_panel_visibility() { "btn on" } else { "btn" },
                    onclick: move |_| {
                        filter_menu_visible.set(!filter_menu_visible());
                    },
                    img { class: "button-svg-container",
                        src: FILTER_BUTTON_SVG,
                    }
                    span { class: "button-text", "Filter" }
                }
                if filter_menu_visible() {
                    FilterMenu { }
                }
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
            ResizePanel { visibility: resize_panel_visibility }
        }
        if blur_panel_visibility() {
            BlurPanel { }
        }
    }
}
