use dioxus::prelude::*;
use crate::state::app_state::{ImageState};

#[component]
pub fn FootBar() -> Element {
    let mut zoom_signal = use_context::<ImageState>().zoom;
    let zoom_limits = use_context::<ImageState>().limits;
    let zoom_value = *zoom_signal.read();

    rsx! {
        div { class: "footer-main",
            div { class: "footer-left"  },
            div { class: "footer-mid"   },
            div { class: "footer-right" ,
                div { class: "zoom-slider-container",
                    input {
                        type: "range",
                        min: format!("{}", zoom_limits().0),
                        value:"{zoom_value}" ,
                        max: format!("{}", zoom_limits().1),
                        class: "zoom-slider",
                        id:"range1",
                        oninput: move |e| {
                            if let Ok(parsed) = e.value().parse::<i64>() {
                                zoom_signal.set(parsed);
                            }
                        }
                    },
                    label{"{zoom_value}%"}
                }
            }
        }
    }
}
