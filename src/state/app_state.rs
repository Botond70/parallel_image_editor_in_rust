use std::collections::VecDeque;
use dioxus::prelude::*;
use image::DynamicImage;

#[derive(Clone, Copy, Debug)]
pub enum RedrawKind {
    HSV,
    Crop,
    Resize,
    Blur,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlurMode {
    Gaussian,
    Box,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlurDirection {
    Omnidirectional,
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy)]
pub struct BlurState {
    pub panel_visible: Signal<bool>,
    pub mode: Signal<BlurMode>,
    pub window_size: Signal<u32>,
    pub direction: Signal<BlurDirection>,
    pub redraw_request_count: Signal<u64>,
}

#[derive(Clone, Copy)]
pub struct FilterMenuState {
    pub menu_visible: Signal<bool>,
}

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
    pub left_applied: Signal<f32>,
    pub right_applied: Signal<f32>,
    pub top_applied: Signal<f32>,
    pub bottom_applied: Signal<f32>,
    pub cropbox_element: Signal<Option<web_sys::Element>>,
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
    pub image_modified: Signal<bool>,
    pub ui_redraw_start_ns: Signal<Option<u64>>,
    pub ui_redraw_kind: Signal<Option<RedrawKind>>,
}

#[derive(Clone, Copy)]
pub struct UndoRedoState {
    pub undo_stack: Signal<Vec<crate::utils::undo_redo::EditorSnapshot>>,
    pub redo_stack: Signal<Vec<crate::utils::undo_redo::EditorSnapshot>>,
}