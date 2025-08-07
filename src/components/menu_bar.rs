use dioxus::prelude::*;
use crate::state::app_state::{SideBarVisibility};

#[component]
pub fn MenuBar() -> Element {
    let curr_state = *use_context::<SideBarVisibility>().state.read();

    let toggle = move |_| {
        use_context::<SideBarVisibility>().state.set(!curr_state);
    };

    rsx! {
        div { class: "menubar-container",
            div { class: "view-dropdown",
                button { class: "btn", "View"}
                div { class: "dropdown-content",
                    button { onclick: toggle,
                        class: "btn", "Toggle Sidebar" }
                    button { class: "btn", "Click me!" }
                    button { class: "btn", "Click me!" }
                }
            }
        }
    }
}
