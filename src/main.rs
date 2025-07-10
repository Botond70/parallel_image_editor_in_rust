use dioxus::{html::{image, img}, prelude::*};
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TEST_IMG: Asset = asset!("/assets/wgpu_jumpscare.png");

#[derive(Clone, Copy)]
struct SidebarVisibility {
    state: Signal<bool>,
}

fn main() {
    LaunchBuilder::new()
        .with_cfg(
            Config::default().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_title("Editor")
                    .with_min_inner_size(LogicalSize::new(800.0, 500.0)),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    let visibility = use_signal(|| true);
    use_context_provider(|| SidebarVisibility { state: visibility });

    rsx! {

        document::Stylesheet { rel: "stylesheet", href: MAIN_CSS }
        MenuBar {}
        WorkSpace {}


    }
}

#[component]
fn SideBar() -> Element {
    let visibility = *use_context::<SidebarVisibility>().state.read();

    rsx! {
        div { class: "sidebar-container", style: if visibility { "display: flex;" } else { "display:none;" },
            button { class: "btn" , "Click me!"}
            button { class: "btn" , "Click me!"}
            button { class: "btn" , "Click me!"}
            button { class: "btn" , "Click me!"}
            button { class: "btn" , "Click me!"}
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

#[component]
fn ImageBoard() -> Element {
    rsx! {
        div { class: "image-container",
            div { class: "image-inner",
                img {
                    src: TEST_IMG,
                    class: "image-board",
                }
            }
        }
    }
}

#[component]
fn WorkSpace() -> Element {
    rsx! {
        div { class: "work-space",
            SideBar {}
            ImageBoard {}
        }
    }
}
