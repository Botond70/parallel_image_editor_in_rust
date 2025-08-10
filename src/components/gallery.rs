use crate::app_router::Route;
use crate::state::app_state::ImageVec;
use web_sys::console;
use dioxus::prelude::*;
const BACK_BUTTON: Asset = asset!("/assets/back-button.svg");

#[component]
pub fn Gallery() -> Element {
    let mut img_vec = use_context::<ImageVec>().base64_vector;
    let mut curr_index = use_context::<ImageVec>().curr_image_index;
    let img_vector = img_vec().clone();

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
                {
                    img_vector.iter().enumerate().map(|(index, img_url)| {
                        rsx! (
                                div { class: "image-display",
                                    div { class: "is-selected-wrapper",
                                        style: if index == curr_index() { "background-color: rgba(200, 200, 200, 0.5); scale: 1.1;"},
                                        img {
                                            key: index,
                                            onclick: move |_| {
                                                curr_index.set(index);
                                                console::log_1(&format!("Clicked image index: {}", index).into());
                                            },
                                            src: "{img_url}"
                                        }
                                        p { "PLACEHOLDER" }
                                    }
                                }
                        )
                    })
                }
            }
        }
    }
}
