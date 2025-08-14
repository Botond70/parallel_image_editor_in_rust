use crate::app_router::Route;
use crate::state::app_state::{GalleryState, ImageVec};
use dioxus::html::col;
use dioxus::html::g::dangerous_inner_html;
use web_sys::console;
use dioxus::prelude::*;
use image::GenericImageView;
const BACK_BUTTON: Asset = asset!("/assets/back-button.svg");

const GRID_SIZE_BUTTON_SVG: &str = "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='size-6'>
  <path stroke-linecap='round' stroke-linejoin='round' d='M9 4.5v15m6-15v15m-10.875 0h15.75c.621 0 1.125-.504 1.125-1.125V5.625c0-.621-.504-1.125-1.125-1.125H4.125C3.504 4.5 3 5.004 3 5.625v12.75c0 .621.504 1.125 1.125 1.125Z' />
</svg>";

const CHEVRON_DOWN_SVG: &str = "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='size-6'>
  <path stroke-linecap='round' stroke-linejoin='round' d='m19.5 8.25-7.5 7.5-7.5-7.5' />
</svg>
";

#[component]
fn GalleryHeader() -> Element {
    let mut grid_size = use_context::<GalleryState>().grid_size;
    let mut dropdown_visible = use_context::<GalleryState>().visibility;
    let visibility = if dropdown_visible() {
        "display: column;"
    } else {
        "display: none;"
    };

    rsx! {
        div { class: "gallery-page-header",
            div { class: "back-button-wrapper",
                Link { to: Route::WorkSpace,
                    img { class: "back-button",
                        src: BACK_BUTTON
                    }
                }
            }
            p { "GALLERY" }
            div { class: "grid-size-select-container",
                onclick: move |_| {
                    dropdown_visible.set(!dropdown_visible());
                },
                div { class: "grid-size-select-button",
                    dangerous_inner_html: GRID_SIZE_BUTTON_SVG
                }
                div { class: "chevron-button",
                    dangerous_inner_html: CHEVRON_DOWN_SVG
                }
                div { class: "dropdown-content",
                    style: visibility,
                    button { class: "btn", onclick: move |_| { grid_size.set(String::from("large")) }, "Large" }
                    button { class: "btn", onclick: move |_| { grid_size.set(String::from("medium")) }, "Medium" }
                    button { class: "btn", onclick: move |_| { grid_size.set(String::from("small")) }, "Small" }
                }
            }
        }
    }
}


#[component]
pub fn Gallery() -> Element {
    let img_vec_base64 = use_context::<ImageVec>().base64_vector;
    let img_vec = use_context::<ImageVec>().vector;
    let mut curr_index = use_context::<ImageVec>().curr_image_index;
    let grid_size = use_context::<GalleryState>().grid_size;
    let img_vector = img_vec_base64();

    let (column_width, image_width, image_height) = match &*grid_size() {
        "small" => (220, 180, 90),
        "medium" => (400, 360, 180),
        "large" => (520, 480, 270),
        _ => (400, 360, 180)
    };

    console::log_1(&format!("Current index: {}", curr_index()).into());

    rsx! {
        div { class: "gallery-page",
            GalleryHeader { }
            div { class: "image-display-container",
                style: format!("grid-template-columns: repeat(auto-fit, minmax({}px, 1fr));", column_width),
                {
                    img_vector.iter().enumerate().map(|(index, img_url)| {
                        rsx! (
                                div { class: "image-display",
                                    div { class: "is-selected-wrapper",
                                        style: if index == curr_index() { "background-color: rgba(200, 200, 200, 0.5); scale: 1.1;"},
                                        Link { to: Route::WorkSpace,
                                            img {
                                                key: index,
                                                style: format!("width: {}px; height: {}px;", image_width, image_height),
                                                onclick: move |_| {
                                                    curr_index.set(index);
                                                    console::log_1(&format!("Clicked image index: {}", index).into());
                                                },
                                                src: "{img_url}"
                                            }
                                        }
                                    }
                                }
                        )
                    })
                }
            }
        }
    }
}
