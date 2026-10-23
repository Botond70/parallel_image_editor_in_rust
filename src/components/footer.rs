use dioxus::prelude::*;
use crate::state::app_state::{ImageState};

#[component]
pub fn FootBar() -> Element {
    let image_state = use_context::<ImageState>();
    let image_vec = image_state.image_vector.read();
    let currently_selected_image = image_vec.get(*image_state.curr_image_index.read());

    let mut zoom_signal = use_context::<ImageState>().zoom;
    let zoom_limits = use_context::<ImageState>().limits;
    let zoom_value = *zoom_signal.read();

    rsx! {
        div { class: "footer-main",
            div { class: "footer-left flex px-4 justify-between items-center",
                if let Some(image) = currently_selected_image {
                    span {
                        class: "text-sm text-white-900",
                        "Currently editing: {image.file_name}"
                    }
                    span {
                        class: "text-sm text-white-900",
                        {format!("{}x{}", image.image_data.width(), image.image_data.height())}
                    }
                }
            },
            div { class: "footer-mid"   },
            div { class: "footer-right" ,
                div { class: "zoom-slider-container flex gap-2",
                    input {
                        type: "range",
                        min: format!("{}", zoom_limits().0),
                        value:"{zoom_value}" ,
                        max: format!("{}", zoom_limits().1),
                        class: "styled-slider",
                        id:"range1",
                        oninput: move |e| {
                            if let Ok(parsed) = e.value().parse::<i64>() {
                                zoom_signal.set(parsed);
                            }
                        }
                    },
                    p { class: "slider-progress", "{zoom_value}%" }
                }
            }
        }
    }
}
