use dioxus::prelude::*;
use dioxus_desktop::{Config, WindowBuilder, LogicalSize};

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone, Copy)]
struct SidebarVisibility {
    state: Signal<bool>,
}

fn main() {

    LaunchBuilder::new()
    .with_cfg(
      Config::default()
        .with_menu(None)
        .with_window(
          WindowBuilder::new()
            .with_title("Editor")
            .with_min_inner_size(LogicalSize::new(800.0, 500.0))
        )
    )
    .launch(App);
}

#[component]
fn App() -> Element {
    let visibility = use_signal(|| true);
    use_context_provider(|| SidebarVisibility { state: visibility});

    rsx! {

        document::Stylesheet { rel: "stylesheet", href: MAIN_CSS }

        MenuBar {}
        SideBar {}

    }
}

#[component]
fn SideBar() -> Element {
    let visibility = *use_context::<SidebarVisibility>().state.read();

    rsx! {
        div { class: "sidebar-container", style: if visibility { "display: flex;" } else { "display:none;" },
            button { class: "sidebar-button" , "Click me!"}
            button { class: "sidebar-button" , "Click me!"}
            button { class: "sidebar-button" , "Click me!"}
            button { class: "sidebar-button" , "Click me!"}
            button { class: "sidebar-button" , "Click me!"}
        }
    }
}

#[component]
fn MenuBar() -> Element {
    let curr_state = *use_context::<SidebarVisibility>().state.read();

    let toggle = move |_| {
         use_context::<SidebarVisibility>().state.set(!curr_state);
    };

    rsx! {
        div { class: "menubar-container",
            div { class: "view-dropdown",
                button { class: "view-btn", "View"}
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
