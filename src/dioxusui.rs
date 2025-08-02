use crate::renderer::start_wgpu;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as base64_engine;
use dioxus::{html::{view, HasFileData}, prelude::*};
use image::{DynamicImage, GenericImageView, load_from_memory};
use std::{io::Cursor, path::absolute};
use web_sys::{console, window};

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

#[component]
pub fn App() -> Element {
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
    let is_visible = *use_context::<SidebarVisibility>().state.read();
    let sidebar_style = if is_visible {
        "display: flex;"
    } else {
        "display: none;"
    };

    rsx! {
        div { class: "sidebar-container", style: sidebar_style,
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

fn clamp_translate_value(tx: f64, ty: f64, viewport: (f64, f64), image_size: (f64, f64)) -> (f64, f64) {
    (
        tx.min(image_size.0 + viewport.0).max(-image_size.0 - viewport.0),
        ty.min(image_size.1 + viewport.1).max(-image_size.1 - viewport.1)
    )
}

#[component]
pub fn ImageBoard() -> Element {
    let curr_zoom = *use_context::<ImageZoom>().zoom.read();
    let scale_value: f64 = curr_zoom as f64 / 100.0;
    let mut image_data_url = use_signal(|| None::<String>);
    let mut translation = use_signal(|| (0.0, 0.0));
    let mut is_dragging = use_signal(|| false);
    let mut start_position = use_signal(|| (0.0, 0.0));
    let get_viewport_size = || {
        let window = window().expect("No global window found.");
        let width = window.inner_width().unwrap();
        let height = window.inner_height().unwrap();
        (width.as_f64().unwrap(), height.as_f64().unwrap())
    };
    let mut viewport_size = use_signal(|| get_viewport_size());
    let mut image_size = use_signal(|| (0.0, 0.0));
    
    use_effect(move || {
        if image_data_url().is_some() {
            spawn(start_wgpu());
        }
    });

    rsx! {
        div { class: "image-container",
            onmousedown: move |evt| {
                is_dragging.set(true);
                start_position.set((evt.coordinates().client().x, evt.coordinates().client().y));
                viewport_size.set(get_viewport_size());
            },
            onmouseup: move |_| {
                is_dragging.set(false);
            },
            onmousemove: move |evt| {
                if is_dragging() && image_data_url().is_some() {
                    let (start_x, start_y) = (start_position().0, start_position().1);
                    let dx = evt.coordinates().client().x - start_x;
                    let dy = evt.coordinates().client().y - start_y;
                    start_position.set((evt.coordinates().client().x, evt.coordinates().client().y));
                    let (tx, ty) = translation();
                    let clamped_translation = clamp_translate_value(tx + dx, ty + dy, viewport_size(), (image_size().0 * scale_value, image_size().1 * scale_value));
                    translation.set((clamped_translation.0, clamped_translation.1));
                }
            },
            ondragover: move |evt| {
                evt.prevent_default();
            },
            ondrop: move |evt| {
                evt.prevent_default();

                let file_engine = evt.files().unwrap();
                let file_names = file_engine.files();
                let first_file_name = file_names.iter().next().cloned().unwrap();

                spawn(async move {
                    if let Some(bytes) = file_engine.read_file(&first_file_name).await {
                        match load_from_memory(&bytes) {
                            Ok(img) => {
                                println!("Loaded image: {:?}", img.dimensions());
                                
                                image_size.set((img.dimensions().0 as f64, img.dimensions().1 as f64));
                                
                                let mut png_bytes = Vec::new();
                                if let Err(err) = img.write_to(&mut Cursor::new(&mut png_bytes), image::ImageFormat::Png) {
                                    println!("Error during formatting: {err:?}");
                                }

                                let base64_str = base64_engine.encode(&png_bytes);
                                image_data_url.set(format!("data:image/png;base64,{}", base64_str).into());
                            },
                            Err(err) => {println!("{err:?}");}
                        }
                    }
                });
            },

            match image_data_url.as_ref() {
                Some(url) => {
                    rsx!(
                    div { class: "image-inner",
                        style: format!("transform: translate({}px, {}px); scale: {};", translation().0 / scale_value, translation().1 / scale_value, scale_value),
                        canvas {
                            id: "image-board",
                            draggable: false,
                        }
                    }
                )
                },
                None => rsx!(p {class: "text",
                    "Drag and drop images here!"})
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
    let mut zoom_signal = use_context::<ImageZoom>().zoom;
    let zoom_value = *zoom_signal.read();

    rsx! {
        div { class: "footer-main",
            div { class: "footer-left"  },
            div { class: "footer-mid"   },
            div { class: "footer-right" ,
                div { class: "zoom",
                    input {
                        type: "range",
                        min: "20",
                        value:"{zoom_value}" ,
                        max: "400",
                        class: "slider",
                        id:"range1",
                        oninput: move |e| {
                            if let Ok(parsed) = e.value().parse::<u64>() {
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
