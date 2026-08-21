use crate::{
    app_router::Route,
    state::app_state::{ImageState, SideBarState, WGPUSignal, UndoRedoState, CropSignal, HSVState, ResizeState, BlurState},
    utils::upload_img::upload_img,
    utils::undo_redo::{perform_redo, perform_undo},
};
use dioxus::prelude::*;

#[component]
pub fn MenuBar() -> Element {
    let curr_state = *use_context::<SideBarState>().sidebar_is_visible.read();
    let mut sidebar_signal = use_context::<SideBarState>().sidebar_is_visible;
    let toggle = move |_| {
        sidebar_signal.set(!curr_state);
    };

    let curr_save = *use_context::<WGPUSignal>().save_signal.read();
    let mut saver_signal = use_context::<WGPUSignal>().save_signal;
    let saver = move |_| {
        saver_signal.set(curr_save + 1);
    };

    rsx! {
        div { class: "menubar-container",
            div { class: "dropdown-button-container",
                button {class: "btn", "File" }
                div { class: "dropdown-content",
                    label { class: "btn", "Load",
                    input { r#type: "file", accept:"image/*", multiple: "true",
                        onchange: move |evt| {
                            let files = evt.files().unwrap();
                            upload_img(
                                files,
                                use_context::<ImageState>().img_size,
                                use_context::<WGPUSignal>().signal,
                                use_context::<WGPUSignal>().ready_signal,
                                use_context::<ImageState>().zoom,
                                use_context::<ImageState>().base64_vector,
                                use_context::<ImageState>().image_vector,
                            );
                        },
                    }},
                    button { onclick: saver, class: "btn", "Save as" }
                }
            }
            div { class: "dropdown-button-container",
                button { class: "btn", "View" }
                div { class: "dropdown-content",
                    button { onclick: toggle,
                        class: "btn", "Toggle Sidebar" }
                }
            }
            Link { to: Route::Gallery, button { class: "btn", "Gallery" } }
            UndoRedoPanel {}
        }
    }
}


#[component]
fn UndoRedoPanel() -> Element {
    let undo_redo_state = use_context::<UndoRedoState>();
    let image_state = use_context::<ImageState>();
    let resize_state = use_context::<ResizeState>();
    let crop_state = use_context::<CropSignal>();
    let hsv_state = use_context::<HSVState>();
    let blur_state = use_context::<BlurState>();

    let undo_count = (undo_redo_state.undo_len)();
    let redo_count = (undo_redo_state.redo_len)();

    rsx! {
        div { class: "undo-redo-panel",
            div { class: "history-actions",
                button {
                    class: if undo_count > 0 { "btn" } else { "btn disabled" },
                    onclick: move |_| {
                        if undo_count > 0 {
                            perform_undo(
                                undo_redo_state,
                                &image_state,
                                &resize_state,
                                &crop_state,
                                &hsv_state,
                                &blur_state,
                            );
                        }
                    },
                    "Undo"
                }
                button {
                    class: if redo_count > 0 { "btn" } else { "btn disabled" },
                    onclick: move |_| {
                        if redo_count > 0 {
                            perform_redo(
                                undo_redo_state,
                                &image_state,
                                &resize_state,
                                &crop_state,
                                &hsv_state,
                                &blur_state,
                            );
                        }
                    },
                    "Redo"
                }
            }

        }
    }
}
