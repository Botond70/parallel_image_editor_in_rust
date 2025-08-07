use dioxus::prelude::*;
use crate::state::app_state::{SideBarVisibility, NextImage};
use web_sys::console;

#[component]
pub fn SideBar() -> Element {
    let is_visible = *use_context::<SideBarVisibility>().state.read();
    let nowcount = *use_context::<NextImage>().count.read();
    let sidebar_style = if is_visible {
        "display: flex;"
    } else {
        "display: none;"
    };

    let nextimg = move |_| {
        console::log_1(&"Trying to load next image".into());
        use_context::<NextImage>().pressed.set(true);
        use_context::<NextImage>().count.set(nowcount + 1);
    };

    rsx! {
        div { class: "sidebar-container", style: sidebar_style,
            button { onclick: nextimg, class: "btn" , "Load next image"}
            button { class: "btn" , "Click me!"}
            button { class: "btn" , "Click me!"}
            button { class: "btn" , "Click me!"}
            button { class: "btn" , "Click me!"}
        }
    }
}
