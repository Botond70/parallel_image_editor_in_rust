use crate::state::app_state::{NextImage, SideBarVisibility};
use dioxus::prelude::*;
use web_sys::console;

const HSV_BUTTON_SVG: &str = "<svg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 24 24' stroke-width='1.5' stroke='currentColor' class='size-6'>
  <path stroke-linecap='round' stroke-linejoin='round' d='M10.5 6h9.75M10.5 6a1.5 1.5 0 1 1-3 0m3 0a1.5 1.5 0 1 0-3 0M3.75 6H7.5m3 12h9.75m-9.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-3.75 0H7.5m9-6h3.75m-3.75 0a1.5 1.5 0 0 1-3 0m3 0a1.5 1.5 0 0 0-3 0m-9.75 0h9.75' />
</svg>";


#[component]
pub fn SideBar() -> Element {
    let is_visible = *use_context::<SideBarVisibility>().state.read();
    let nowcount = *use_context::<NextImage>().count.read();
    let sidebar_style = if is_visible {
        "display: flex;"
    } else {
        "display: none;"
    };

    rsx! {
        div { class: "sidebar-container", style: sidebar_style,
            button { class: "btn",
                div { class: "button-contents",
                    div { class: "button-svg-container",
                        dangerous_inner_html: HSV_BUTTON_SVG
                    }
                    p { class: "button-text", "HSV" }
                }
            }
            button { class: "btn" , "Click me!"}
            button { class: "btn" , "Click me!"}
            button { class: "btn" , "Click me!"}
            button { class: "btn" , "Click me!"}
        }
    }
}
