use crate::app_router::Route;
use crate::dioxusui::TEST_IMG2;
use crate::state::app_state::ImageVec;
use dioxus::prelude::*;
const BACK_BUTTON: Asset = asset!("/assets/back-button.svg");

#[component]
pub fn Gallery() -> Element {
    let mut img_vec = use_context::<ImageVec>().vector;
    let mut curr_index = use_context::<ImageVec>().curr_image_index;

    rsx! {
        div { class: "gallery-page",
            div { class: "gallery-page-header",
                div { class: "back-button-wrapper",
                    Link { to: Route::WorkSpace,
                        img { class: "back-button",
                            src: BACK_BUTTON
                        }
                    }
                }
                p { "GALLERY" }
            }
            div { class: "image-display-container",
                for i in 0..img_vec().len(){
                    div { class: "image-display",
                        img {
                            src: TEST_IMG2
                        }
                        p { "360 x 120" }
                    }
                },
            }
        }
    }
}
