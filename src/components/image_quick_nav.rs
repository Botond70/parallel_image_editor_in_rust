use dioxus::prelude::*;
use crate::state::app_state::{ImageState, UndoRedoState};
use web_sys::console;
use crate::utils::undo_redo::clear_undo_redo_history;
use image::GenericImageView;

const UP_ARROW_SVG: Asset = asset!("/assets/up-arrow.svg");
const DOWN_ARROW_SVG: Asset = asset!("/assets/down-arrow.svg");

#[component]
pub fn ImageQuickNav() -> Element {
    let mut image_state = use_context::<ImageState>();
    let mut curr_index = use_context::<ImageState>().curr_image_index;
    let mut image_size = use_context::<ImageState>().img_size;
    let image_vector = image_state.image_vector;
    let image_count = image_vector.read().len();
    let image_index = curr_index();

    let start_index = image_index.saturating_sub(1).min(image_count.saturating_sub(3));

    let end_index = (start_index + 3).min(image_count);

    rsx! {
        div {
            id: "image-quick-nav-container",
            div {
                id: "image-quick-nav-button-container",
                class: "h-12 min-h-12 w-full flex items-center justify-center shrink-0 border-b-1 border-solid border-gray-600",
                button {
                    id: "image-quick-nav-button-previous",
                    class: "flex items-center justify-center w-full h-full cursor-pointer hover:bg-cyan-400",
                    onclick: move |_| {
                        if image_count == 0 {
                            return;
                        }

                        let index = *curr_index.read();
                        let new_index = if index == 0 {
                            image_count - 1
                        } else {
                            index - 1
                        };

                        clear_undo_redo_history(use_context::<UndoRedoState>());
                        curr_index.set(new_index);
                    },
                    img {
                        class: "button-svg-container",
                        src: UP_ARROW_SVG,
                    }
                }
            }
            div {
                id: "image-quick-nav-image-container",
                class: "w-full h-full flex flex-col items-center justify-evenly gap-2 overflow-y-hidden",
                if image_count > 0 {
                    for index in start_index..end_index {
                        div {
                            id: "image-quick-nav-image-wrapper-{index}",
                            class: "w-55 h-50 flex flex-col",
                            div {
                                key: "{index}",
                                class: if index == image_index {
                                    "w-55 h-40 rounded-sm outline-2 outline-solid outline-blue-500 cursor-pointer"
                                } else {
                                    "w-55 h-40 rounded-sm outline-1 outline-solid outline-gray-500 cursor-pointer"
                                },
                                img {
                                    class: "w-55 h-40 object-fill rounded-sm",
                                    src: image_vector.read()[index].base64_data.clone(),
                                }
                            }
                            div {
                                id: "image-quick-nav-image-info-{index}",
                                class: "w-55 h-10 flex flex items-center justify-between",
                                span {
                                    class: "text-sm text-gray-400 pt-1",
                                    "{image_vector.read()[index].file_name.clone()}"
                                }
                                span {
                                    class: "text-sm text-gray-400 pt-1",
                                    "{image_vector.read()[index].image_data.dimensions().0} x {image_vector.read()[index].image_data.dimensions().1}"
                                }
                            }
                        }
                    }
                }
                span {
                    class: "text-sm text-white-900 bg-gray-600 px-2 py-1 rounded-lg bg-opacity-50",
                    if image_count == 0 {
                        "No images loaded"
                    } else {
                        {format!("Image {}/{}", image_index + 1, image_count)}
                    }
                }
            }
            div {
                id: "image-quick-nav-button-container",
                class: "h-12 min-h-12 w-full flex items-center justify-center shrink-0 border-t-1 border-solid border-gray-600",
                button {
                    id: "image-quick-nav-button-next",
                    class: "flex items-center justify-center w-full h-full cursor-pointer hover:bg-cyan-400",
                    onclick: move |_| {
                        if image_count == 0 {
                            return;
                        }

                        let index = *curr_index.read();
                        let new_index = (index + 1) % image_count;

                        clear_undo_redo_history(use_context::<UndoRedoState>());
                        curr_index.set(new_index);
                    },
                    img {
                        class: "button-svg-container",
                        src: DOWN_ARROW_SVG,
                    }
                }
            }
        }
    }
}