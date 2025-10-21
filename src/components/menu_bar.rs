use crate::{
    app_router::Route,
    state::app_state::{SideBarState, WGPUSignal},
};
use dioxus::prelude::*;

#[component]
pub fn MenuBar() -> Element {
    let curr_state = *use_context::<SideBarState>().sidebar_is_visible.read();
    let toggle = move |_| {
        use_context::<SideBarState>().sidebar_is_visible.set(!curr_state);
    };

    let curr_save = *use_context::<WGPUSignal>().save_signal.read();
    let saver = move |_| {
        use_context::<WGPUSignal>().save_signal.set(curr_save + 1);
    };

    rsx! {
        div { class: "menubar-container",
            div { class: "dropdown-button-container",
                button {class: "btn", "File" }
                div { class: "dropdown-content",
                    button { class: "btn", "Load" }
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
