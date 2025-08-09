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
    pub curr_image_index: Signal<usize>,
}

#[derive(Clone, Copy)]
pub struct WGPUSignal {
    pub signal: Signal<bool>,
}
