use std::collections::VecDeque;

use crate::app_router::Route;
use crate::components::{
    footer::FootBar, image_board::ImageBoard, menu_bar::MenuBar, side_bar::HSVPanel,
    side_bar::SideBar,
};
use crate::state::app_state::{

    CropSignal, DragSignal, GalleryState, HSVState, ImageVec, ImageZoom, NextImage, ResizeState,
    SideBarVisibility, TestPanelVisibility, WGPUSignal,
};
use dioxus::html::canvas::width;
use dioxus::prelude::*;
use image::DynamicImage;
use web_sys::{Window, console, window};

const MAIN_CSS: Asset = asset!("/assets/main.css");
pub static GLOBAL_WINDOW_HANDLE: GlobalSignal<Window> =
    Signal::global(|| window().expect("No global window found"));

#[component]
pub fn App() -> Element {
    let visibility = use_signal(|| true);

    let img_scale = use_signal(|| 100);
    let IMG_SCALE_LIMITS: Signal<(i64, i64)> = use_signal(|| (20, 3000));
    let image_vector = use_signal(|| VecDeque::<DynamicImage>::new());
    let image_vector_base64 = use_signal(|| VecDeque::<String>::new());
    let image_index = use_signal(|| 0 as usize);
    let img_next = use_signal(|| false);
    let img_iter = use_signal(|| 0 as u32);

    let wgpu_signal = use_signal(|| false);

    let grid_size = use_signal(|| String::from("medium"));

    let dropdown_visible = use_signal(|| false);

    let hsv_visible = use_signal(|| false);
    let hue = use_signal(|| 0 as f32);
    let saturation = use_signal(|| 0 as f32);
    let value = use_signal(|| 0 as f32);

    let panel_visibility = use_signal(|| false);

    let save_signal = use_signal(|| 0 as i64);

    let can_drag = use_signal(|| false);


    let rs_width = use_signal(|| 800 as u32);
    let rs_height = use_signal(|| 600 as u32);
    let resize_panel_visible = use_signal(|| false);

    let crop_panel_visibility = use_signal(|| false);
    let left = use_signal(|| 0.0 as f32);
    let right = use_signal(|| 0.0 as f32);
    let top = use_signal(|| 0.0 as f32);
    let bottom = use_signal(|| 0.0 as f32);


    use_context_provider(|| DragSignal { can_drag });
    use_context_provider(|| TestPanelVisibility {
        visibility: panel_visibility,
    });
    use_context_provider(|| GalleryState {
        grid_size,
        visibility: dropdown_visible,
    });
    use_context_provider(|| WGPUSignal {
        signal: wgpu_signal,
        save_signal: save_signal,
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
    use_context_provider(|| HSVState {
        panel_visible: hsv_visible,
        hue,
        saturation,
        value,
    });

    use_context_provider(|| ResizeState {
        panel_visible: resize_panel_visible,
        width: rs_width,
        height: rs_height,

    use_context_provider(|| CropSignal {
        visibility: crop_panel_visibility,
        left,
        right,
        top,
        bottom,
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
