use dioxus::prelude::*;
use crate::dioxusui::TEST_IMG2;
use crate::app_router::Route;

const BACK_BUTTON: Asset = asset!("/assets/back-button.svg");

#[component]
pub fn Gallery() -> Element {
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
                for i in 1..200 {
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
