use dioxus::prelude::*;
use crate::state::app_state::{ImageZoom};

#[component]
pub fn FootBar() -> Element {
    let mut zoom_signal = use_context::<ImageZoom>().zoom;
    let zoom_limits = use_context::<ImageZoom>().limits;
    let zoom_value = *zoom_signal.read();
    let mut progress_style = use_signal(|| String::from(""));

    use_effect(move || {
        let progress_percentage = zoom_signal() * 100 / zoom_limits().1;
        progress_style.set(format!(
            "background: linear-gradient(to right, white {}%, white {}%, gray {}%)",
            progress_percentage, progress_percentage, progress_percentage
        ));
    });

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
                        style: progress_style(),
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
