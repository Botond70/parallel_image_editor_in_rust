use std::collections::VecDeque;

use crate::app_router::Route;
use crate::components::{
    footer::FootBar, image_board::ImageBoard, menu_bar::MenuBar, side_bar::SideBar,
};
use crate::state::app_state::{GalleryState, ImageVec, ImageZoom, NextImage, SideBarVisibility, WGPUSignal};
use dioxus::prelude::*;
use image::DynamicImage;
use web_sys::{console, window};

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    let visibility = use_signal(|| true);
    let img_scale = use_signal(|| 100);
    let IMG_SCALE_LIMITS: Signal<(i64, i64)> = use_signal(|| (20, 700));
    let image_vector = use_signal(|| VecDeque::<DynamicImage>::new());
    let image_vector_base64 = use_signal(|| VecDeque::<String>::new());
    let image_index = use_signal(|| 0 as usize);
    let img_next = use_signal(|| false);
    let img_iter = use_signal(|| 0 as u32);
    let wgpu_signal = use_signal(|| false);
    let grid_size = use_signal(|| String::from("medium"));
    let dropdown_visible = use_signal(|| false);
    use_context_provider(|| GalleryState {
        grid_size,
        visibility: dropdown_visible,
    });
    use_context_provider(|| WGPUSignal {
        signal: wgpu_signal,
    });
    use_context_provider(|| SideBarVisibility { state: visibility });
    use_context_provider(|| ImageZoom {
        zoom: img_scale,
        limits: IMG_SCALE_LIMITS,
    });
    use_context_provider(|| NextImage {
        pressed: img_next,
        count: img_iter,
    });
    use_context_provider(|| ImageVec {
        vector: image_vector,
        curr_image_index: image_index,
        base64_vector: image_vector_base64,
    });
    rsx! {

        document::Stylesheet { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}

    }
}

#[component]
pub fn WorkSpace() -> Element {
    rsx! {
        MenuBar {}
        FootBar {}
        div { class: "work-space",
            SideBar {}
            ImageBoard {}
        }
    }
}
