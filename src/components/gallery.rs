use crate::app_router::Route;
use crate::state::app_state::ImageVec;
use web_sys::console;
use dioxus::prelude::*;
use image::GenericImageView;
const BACK_BUTTON: Asset = asset!("/assets/back-button.svg");

#[component]
pub fn Gallery() -> Element {
    let img_vec_base64 = use_context::<ImageVec>().base64_vector;
    let img_vec = use_context::<ImageVec>().vector.clone();
    let mut curr_index = use_context::<ImageVec>().curr_image_index;
    let img_vector = img_vec_base64().clone();

    console::log_1(&format!("Current index: {}", curr_index()).into());

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
                                        Link { to: Route::WorkSpace,
                                            img {
                                                key: index,
                                                onclick: move |_| {
                                                    curr_index.set(index);
                                                    console::log_1(&format!("Clicked image index: {}", index).into());
                                                },
                                                src: "{img_url}"
                                            }
                                        }
                                        p { "{img_vec().get(index).unwrap().dimensions().0} x {img_vec().get(index).unwrap().dimensions().1}" }
                                    }
                                }
                        )
                    })
                }
            }
        }
    }
}
