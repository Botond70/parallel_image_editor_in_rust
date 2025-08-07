use dioxus::prelude::*;

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
