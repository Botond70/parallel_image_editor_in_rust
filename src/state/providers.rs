use std::collections::VecDeque;

use crate::state::app_state::{CropSignal, HSVState, ImageState, SideBarState, WGPUSignal};
use dioxus::prelude::*;
use image::DynamicImage;

pub fn provide_hsv_state() {
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

pub fn provide_sidebar_state() {
    let sidebar_is_visible = use_signal(|| true);
    let is_cropping = use_signal(|| false);
    let is_dragging = use_signal(|| false);

    use_context_provider(|| SideBarState {
        sidebar_is_visible,
        is_cropping,
        is_dragging,
    });
}

pub fn provide_crop_state() {
    let left = use_signal(|| 0.0 as f32);
    let right = use_signal(|| 0.0 as f32);
    let top = use_signal(|| 0.0 as f32);
    let bottom = use_signal(|| 0.0 as f32);

    use_context_provider(|| CropSignal {
        left,
        right,
        top,
        bottom,
    });
}

pub fn provide_image_state() {
    let img_scale = use_signal(|| 100);
    let image_scale_limits: Signal<(i64, i64)> = use_signal(|| (20, 3000));
    let image_vector = use_signal(|| VecDeque::<DynamicImage>::new());
    let image_vector_base64 = use_signal(|| VecDeque::<String>::new());
    let image_index = use_signal(|| 0 as usize);

    use_context_provider(|| ImageState {
        zoom: img_scale,
        limits: image_scale_limits,
        image_vector,
        base64_vector: image_vector_base64,
        curr_image_index: image_index,
    });
}

pub fn provide_wgpu_state() {
    let wgpu_signal = use_signal(|| false);
    let save_signal = use_signal(|| 0 as i64);

    use_context_provider(|| WGPUSignal {
        signal: wgpu_signal,
        save_signal: save_signal,
    });
}