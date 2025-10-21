use std::collections::VecDeque;
use dioxus::prelude::*;
use image::DynamicImage;

#[derive(Clone, Copy)]
pub struct WGPUSignal {
    pub signal: Signal<bool>,
    pub ready_signal: Signal<bool>,
    pub save_signal: Signal<i64>,
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
pub struct CropSignal {
    pub left: Signal<f32>,
    pub right: Signal<f32>,
    pub top: Signal<f32>,
    pub bottom: Signal<f32>,
}

#[derive(Clone, Copy)]
pub struct SideBarState {
    pub sidebar_is_visible: Signal<bool>,
    pub is_cropping: Signal<bool>,
    pub is_dragging: Signal<bool>,
}

#[derive(Clone, Copy)]
pub struct ImageState {
    pub zoom: Signal<i64>,
    pub limits: Signal<(i64, i64)>,
    pub image_vector: Signal<VecDeque<DynamicImage>>,
    pub base64_vector: Signal<VecDeque<String>>,
    pub curr_image_index: Signal<usize>,
    pub img_size: Signal<(f64, f64)>,
}