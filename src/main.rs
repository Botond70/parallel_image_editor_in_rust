use dioxus::{
    html::{image, img},
    prelude::*,
};
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

const MAIN_CSS: Asset = asset!("/assets/main.css");
const TEST_IMG: Asset = asset!("/assets/wgpu_jumpscare.png");

#[derive(Clone, Copy)]
struct SidebarVisibility {
    state: Signal<bool>,
}
#[derive(Clone, Copy)]
struct ImageZoom {
    zoom: Signal<u64>,
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
    let img_scale = use_signal(|| 100);
    use_context_provider(|| SidebarVisibility { state: visibility });
    use_context_provider(|| ImageZoom { zoom: img_scale });
    rsx! {

        document::Stylesheet { rel: "stylesheet", href: MAIN_CSS }
        MenuBar {}
        WorkSpace {}
        FootBar {}

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
#[component]
fn FootBar() -> Element {
    let mut curr_zoom = *use_context::<ImageZoom>().zoom.read();

    rsx! {
        div { class: "footer-main",
            div { class: "footer-left"  },
            div { class: "footer-mid"   },
            div { class: "footer-right" ,
                div { class: "zoom",
                    input {
                        type: "range",
                        min: "20",
                        value:"{curr_zoom}" ,
                        max: "400",
                        class: "slider",
                        id:"range1",
                        oninput: move |e| {
                            curr_zoom = e.value().parse::<u64>().unwrap();
                            use_context::<ImageZoom>().zoom.set(curr_zoom);
                        }


                    },
                    label{"{curr_zoom}"}
                }
            }
        }
    }
}
