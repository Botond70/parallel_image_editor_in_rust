use std::collections::VecDeque;

use crate::state::app_state::{UndoRedoState, BlurDirection, BlurMode, BlurState, CropSignal, FilterMenuState, HSVState, ImageState, RedrawKind, ResizeState, SideBarState, WGPUSignal};

use dioxus::prelude::*;
use image::DynamicImage;

pub fn use_hsv_state() {
    let hsv_visible = use_signal(|| false);
    let hue = use_signal(|| 0 as f32);
    let saturation = use_signal(|| 0 as f32);
    let value = use_signal(|| 0 as f32);

    use_context_provider(|| HSVState {
        panel_visible: hsv_visible,
        hue,
        saturation,
        value,
    });
}

pub fn use_sidebar_state() {
    let sidebar_is_visible = use_signal(|| true);
    let is_cropping = use_signal(|| false);
    let is_dragging = use_signal(|| false);

    use_context_provider(|| SideBarState {
        sidebar_is_visible,
        is_cropping,
        is_dragging,
    });
}

pub fn use_crop_state() {
    let left = use_signal(|| 0.0 as f32);
    let right = use_signal(|| 0.0 as f32);
    let top = use_signal(|| 0.0 as f32);
    let bottom = use_signal(|| 0.0 as f32);
    let left_applied = use_signal(|| 0.0 as f32);
    let right_applied = use_signal(|| 0.0 as f32);
    let top_applied = use_signal(|| 0.0 as f32);
    let bottom_applied = use_signal(|| 0.0 as f32);
    let cropbox_element = use_signal(|| Option::None);

    use_context_provider(|| CropSignal {
        left,
        right,
        top,
        bottom,
        left_applied,
        right_applied,
        top_applied,
        bottom_applied,
        cropbox_element,
    });
}

pub fn use_image_state() {
    let img_scale = use_signal(|| 100);
    let image_scale_limits: Signal<(i64, i64)> = use_signal(|| (20, 3000));
    let image_vector = use_signal(|| VecDeque::<DynamicImage>::new());
    let image_vector_base64 = use_signal(|| VecDeque::<String>::new());
    let image_index = use_signal(|| 0 as usize);
    let img_size = use_signal(|| (0.0, 0.0));
    let image_modified = use_signal(|| false);
    let ui_redraw_start_ns = use_signal(|| Option::<u64>::None);
    let ui_redraw_kind = use_signal(|| Option::<RedrawKind>::None);

    use_context_provider(|| ImageState {
        zoom: img_scale,
        limits: image_scale_limits,
        image_vector,
        base64_vector: image_vector_base64,
        curr_image_index: image_index,
        img_size,
        image_modified,
        ui_redraw_start_ns,
        ui_redraw_kind,
    });
}

pub fn use_blur_state() {
    let blur_panel_visible = use_signal(|| false);
    let blur_mode = use_signal(|| BlurMode::Gaussian);
    let blur_window_size = use_signal(|| 3 as u32);
    let blur_direction = use_signal(|| BlurDirection::Omnidirectional);
    let blur_redraw_request = use_signal(|| 0 as u64);

    use_context_provider(|| BlurState {
        panel_visible: blur_panel_visible,
        mode: blur_mode,
        window_size: blur_window_size,
        direction: blur_direction,
        redraw_request_count: blur_redraw_request,
    });
}

pub fn use_filter_menu_state() {
    let filter_menu_visible = use_signal(|| false);

    use_context_provider(|| FilterMenuState {
        menu_visible: filter_menu_visible,
    });
}

pub fn use_wgpu_state() {
    let wgpu_signal = use_signal(|| false);
    let save_signal = use_signal(|| 0 as i64);
    let ready_signal = use_signal(|| false);

    use_context_provider(|| WGPUSignal {
        signal: wgpu_signal,
        save_signal: save_signal,
        ready_signal,
    });
}

pub fn use_undo_redo_state() {
    let undo_stack = use_signal(|| Vec::<crate::utils::undo_redo::UndoEntry>::new());
    let redo_stack = use_signal(|| Vec::<crate::utils::undo_redo::UndoEntry>::new());
    let undo_len = use_signal(|| 0usize);
    let redo_len = use_signal(|| 0usize);

    use_context_provider(|| UndoRedoState {
        undo_stack,
        redo_stack,
        undo_len,
        redo_len,
    });
}

pub fn use_resize_state() {
    let rs_width = use_signal(|| 800 as u32);
    let rs_height = use_signal(|| 600 as u32);
    let resize_panel_visible = use_signal(|| false);

    use_context_provider(|| ResizeState {
        panel_visible: resize_panel_visible,
        width: rs_width,
        height: rs_height,
    });
}