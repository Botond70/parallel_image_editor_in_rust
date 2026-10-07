use crate::app_router::Route;
use crate::state::app_state::{ImageState, UndoRedoState};
use crate::utils::undo_redo::clear_undo_redo_history;
use image::GenericImageView;
use web_sys::console;
use dioxus::prelude::*;
const BACK_BUTTON: Asset = asset!("/assets/back-button.svg");

const GRID_SIZE_BUTTON_SVG: &str = "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='size-6'>
  <path stroke-linecap='round' stroke-linejoin='round' d='M9 4.5v15m6-15v15m-10.875 0h15.75c.621 0 1.125-.504 1.125-1.125V5.625c0-.621-.504-1.125-1.125-1.125H4.125C3.504 4.5 3 5.004 3 5.625v12.75c0 .621.504 1.125 1.125 1.125Z' />
</svg>";

const CHEVRON_DOWN_SVG: &str = "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='size-6'>
  <path stroke-linecap='round' stroke-linejoin='round' d='m19.5 8.25-7.5 7.5-7.5-7.5' />
</svg>
";

const DEFAULT_GRID_SIZE: &str = "medium";

#[component]
fn GalleryHeader(grid_size: Signal<String>) -> Element {
    let mut dropdown_visible = use_signal(|| false);
    let visibility = if dropdown_visible() {
        "display: flex;"
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
    let image_data_q = use_context::<ImageState>().image_vector;
    let mut image_size = use_context::<ImageState>().img_size;
    let mut curr_index = use_context::<ImageState>().curr_image_index;
    let grid_size = use_signal(|| String::from(DEFAULT_GRID_SIZE));
    let img_vector = image_data_q();

    let (image_width, image_height) = match &*grid_size() {
        "small" => (180, 90),
        "medium" => (360, 180),
        "large" => (480, 270),
        _ => (360, 180)
    };

    let mut handle_onclick = move |index: usize| {
        if curr_index() != index {
            clear_undo_redo_history(use_context::<UndoRedoState>());
        }
        curr_index.set(index);
        let image_q = image_data_q();
        let currently_selected_image = image_q.get(index).expect("Error during ondrop");
        let (width, height) = currently_selected_image.image_data.dimensions();
        image_size.set((width as f64, height as f64));
        console::log_1(&format!("Clicked image index: {}", index).into());
    };

    rsx! {
        div { class: "gallery-page",
            GalleryHeader { 
                grid_size,
            }
            div {
                class: "grid pt-4 pl-2 pr-2 gap-x-4 gap-y-10 overflow-y-auto",
                style: format!(
                    "grid-template-columns: repeat(auto-fill, minmax({}px, 1fr));",
                    image_width
                ),
                {
                    img_vector.iter().enumerate().map(|(index, loaded_img)| {
                        let img_url = loaded_img.base64_data.clone();
                        rsx! (
                            div { class: "image-display flex items-center justify-center w-full",
                                div { class: "is-selected-wrapper",
                                    style: if index == curr_index() { "border: 2px solid #007bff; scale: 1.1;"},
                                    Link { to: Route::WorkSpace,
                                        img {
                                            key: "{index}",
                                            style: format!("width: {}px; height: {}px;", image_width, image_height),
                                            onclick: move |_| {
                                                handle_onclick(index);
                                            },
                                            src: "{img_url}"
                                        }

                                        p {
                                            class: "pt-2 w-fit text-sm text-gray-300",
                                            {loaded_img.file_name.clone()}
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
