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

    console::log_1(&format!("Current image index: {}", curr_index()).into());

    let mut handle_index_update = move |index: usize| {
        let dimensions = {
            let images = image_vector.read();
            images
                .get(index)
                .map(|img| img.image_data.dimensions())
        };

        if let Some((width, height)) = dimensions {
            image_size.set((width as f64, height as f64));
        }
    };

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