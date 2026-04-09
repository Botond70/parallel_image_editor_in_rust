use crate::app_router::Route;
use crate::components::{
    footer::FootBar, image_board::ImageBoard, menu_bar::MenuBar,
    side_bar::SideBar,
};
use crate::state::providers::{use_blur_state, use_crop_state, use_filter_menu_state, use_hsv_state, use_image_state, use_resize_state, use_sidebar_state, use_undo_redo_state, use_wgpu_state};
use dioxus::prelude::*;
use web_sys::{Window, window};

const MAIN_CSS: Asset = asset!("/assets/main.css");
pub static GLOBAL_WINDOW_HANDLE: GlobalSignal<Window> =
    Signal::global(|| window().expect("No global window found"));

#[component]
pub fn App() -> Element {
    use_resize_state();
    use_wgpu_state();
    use_hsv_state();
    use_sidebar_state();
    use_crop_state();
    use_blur_state();
    use_filter_menu_state();
    use_undo_redo_state();
    use_image_state();

    rsx! {
        document::Stylesheet { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {
            config: || {
                RouterConfig::default()
                    .on_update(|state| {
                        (state.current() == Route::NotFound { segments: vec![]})
                            .then_some(NavigationTarget::Internal(Route::WorkSpace))
                    })
            }
        }
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
