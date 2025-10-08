use crate::{
    app_router::Route,
    state::app_state::{DragSignal, HSVState, ImageVec, ImageZoom, NextImage, ResizeState},
    state::app_state::{SideBarVisibility, WGPUSignal},
    utils::upload_img::upload_img,
};
use dioxus::prelude::*;

#[component]
pub fn MenuBar() -> Element {
    let curr_state = *use_context::<SideBarVisibility>().state.read();
    let mut toggle_signal = use_context::<SideBarVisibility>().state;
    let toggle = move |_| toggle_signal.set(!curr_state);

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
                    input { class: "btn", r#type: "file", accept:"image/*", multiple: "true",
                        onchange: move |evt| {
                            let files = evt.files().unwrap();
                            upload_img(
                                files,
                                use_context::<ImageZoom>().img_size,
                                use_context::<WGPUSignal>().signal,
                                use_context::<NextImage>().count,
                                use_context::<WGPUSignal>().ready_signal,
                                use_context::<ImageZoom>().zoom,
                                use_context::<ImageVec>().base64_vector,
                                use_context::<ImageVec>().vector,
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
                    button { class: "btn", "Click me!" }
                    button { class: "btn", "Click me!" }
                }
            }
            Link { to: Route::Gallery, button { class: "btn", "Gallery" } }
        }
    }
}
