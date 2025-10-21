use crate::app_router::Route;
use crate::components::{
    footer::FootBar, image_board::ImageBoard, menu_bar::MenuBar,
    side_bar::SideBar,
};
use crate::state::app_state::{ResizeState};
use dioxus::html::canvas::width;
use crate::state::providers::{provide_crop_state, provide_hsv_state, provide_image_state, provide_sidebar_state, provide_wgpu_state};
use dioxus::prelude::*;
use web_sys::{Window, window};

const MAIN_CSS: Asset = asset!("/assets/main.css");
pub static GLOBAL_WINDOW_HANDLE: GlobalSignal<Window> =
    Signal::global(|| window().expect("No global window found"));

#[component]
pub fn App() -> Element {
    let rs_width = use_signal(|| 800 as u32);
    let rs_height = use_signal(|| 600 as u32);
    let resize_panel_visible = use_signal(|| false);

    use_context_provider(|| ResizeState {
        panel_visible: resize_panel_visible,
        width: rs_width,
        height: rs_height,
    });
    
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
