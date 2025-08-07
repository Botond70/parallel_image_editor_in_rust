use dioxus::prelude::*;
use web_sys::{console, window};
use crate::state::app_state::{SideBarVisibility, ImageZoom, NextImage};
use crate::components::{
    side_bar::SideBar,
    menu_bar::MenuBar,
    image_board::ImageBoard,
    footer::FootBar
};

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TEST_IMG: Asset = asset!("/assets/wgpu_jumpscare.png");

#[component]
pub fn App() -> Element {
    let visibility = use_signal(|| true);
    let img_scale = use_signal(|| 100);
    let IMG_SCALE_LIMITS: Signal<(i64, i64)> = use_signal(|| (20, 700));
    let img_next = use_signal(|| false);
    let img_iter = use_signal(|| 0 as u32);
    use_context_provider(|| SideBarVisibility { state: visibility });
    use_context_provider(|| ImageZoom {
        zoom: img_scale,
        limits: IMG_SCALE_LIMITS,
    });
    use_context_provider(|| NextImage {
        pressed: img_next,
        count: img_iter,
    });
    rsx! {

        document::Stylesheet { rel: "stylesheet", href: MAIN_CSS }
        MenuBar {}
        WorkSpace {}
        FootBar {}

    }
}

#[component]
fn WorkSpace() -> Element {
    rsx! {
        div { class: "work-space",
            SideBar {}
            ImageBoard {}
        }
    }
}
