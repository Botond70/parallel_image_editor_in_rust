use crate::app_router::Route;
use crate::components::{
    footer::FootBar, image_board::ImageBoard, menu_bar::MenuBar,
    side_bar::SideBar,
};
use crate::state::providers::{provide_crop_state, provide_hsv_state, provide_image_state, provide_sidebar_state, provide_wgpu_state};
use dioxus::prelude::*;
use web_sys::{Window, window};

const MAIN_CSS: Asset = asset!("/assets/main.css");
pub static GLOBAL_WINDOW_HANDLE: GlobalSignal<Window> =
    Signal::global(|| window().expect("No global window found"));

#[component]
pub fn App() -> Element {
    provide_wgpu_state();
    provide_hsv_state();
    provide_sidebar_state();
    provide_crop_state();
    provide_image_state();

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
