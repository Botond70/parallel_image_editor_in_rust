use std::collections::VecDeque;

use dioxus::prelude::*;
use image::DynamicImage;

#[derive(Clone, Copy)]
pub struct SideBarVisibility {
    pub state: Signal<bool>,
}

#[derive(Clone, Copy)]
pub struct ImageZoom {
    pub zoom: Signal<i64>,
    pub limits: Signal<(i64, i64)>,
}

#[derive(Clone, Copy)]
pub struct NextImage {
    pub pressed: Signal<bool>,
    pub count: Signal<u32>,
}
#[derive(Clone, Copy)]
pub struct ImageVec {
    pub vector: Signal<VecDeque<DynamicImage>>,
    pub base64_vector: Signal<VecDeque<String>>,
    pub curr_image_index: Signal<usize>,
}

#[derive(Clone, Copy)]
pub struct WGPUSignal {
    pub signal: Signal<bool>,
    pub save_signal: Signal<i64>,
}

#[derive(Clone, Copy)]
pub struct GalleryState {
    pub grid_size: Signal<String>,
    pub visibility: Signal<bool>,
}

#[derive(Clone, Copy)]
pub struct HSVState {
    pub panel_visible: Signal<bool>,
    pub hue: Signal<f32>,
    pub saturation: Signal<f32>,
    pub value: Signal<f32>,
}

#[derive(Clone, Copy)]
pub struct ResizeState {
    pub panel_visible: Signal<bool>,
    pub width: Signal<u32>,
    pub height: Signal<u32>,
}

#[derive(Clone, Copy)]
pub struct TestPanelVisibility {
    pub visibility: Signal<bool>,
}

#[derive(Clone, Copy)]
pub struct DragSignal {
    pub can_drag: Signal<bool>,
}

#[derive(Clone, Copy)]
pub struct CropSignal {
    pub visibility: Signal<bool>,
    pub left: Signal<f32>,
    pub right: Signal<f32>,
    pub top: Signal<f32>,
    pub bottom: Signal<f32>,
}
