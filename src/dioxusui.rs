use crate::customlib::*;
use crate::dioxus_elements::geometry::WheelDelta;
use crate::renderer::start_wgpu;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as base64_engine;
use dioxus::{
    html::{progress, view, HasFileData},
    prelude::*,
};
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
    zoom: Signal<i64>,
    limits: Signal<(i64, i64)>,
}

#[component]
pub fn App() -> Element {
    let visibility = use_signal(|| true);
    let img_scale = use_signal(|| 100);
    let IMG_SCALE_LIMITS: Signal<(i64, i64)> = use_signal(|| (20, 700));
    use_context_provider(|| SidebarVisibility { state: visibility });
    use_context_provider(|| ImageZoom { zoom: img_scale, limits: IMG_SCALE_LIMITS });
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

fn clamp_translate_value(
    tx: f64,
    ty: f64,
    viewport: (f64, f64),
    image_size: (f64, f64),
) -> (f64, f64) {
    (
        tx.min(image_size.0 + viewport.0)
            .max(-image_size.0 - viewport.0),
        ty.min(image_size.1 + viewport.1)
            .max(-image_size.1 - viewport.1),
    )
}

fn get_scroll_value(delta: WheelDelta) -> f64 {
    match delta {
        WheelDelta::Pixels(pixels) => pixels.y,
        _ => 0.0,
    }
}

#[component]
pub fn ImageBoard() -> Element {
    let mut zoom_signal = use_context::<ImageZoom>().zoom;
    let zoom_limits = use_context::<ImageZoom>().limits;
    let scale_value: f64 = zoom_signal() as f64 / 100.0;
    let mut image_data = use_signal(|| None::<DynamicImage>);
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
        if !image_data().is_none() {
            spawn(async move {
                let mut wgpustate = start_wgpu(image_data().unwrap()).await;
                console::log_1(&"Started WGPU".into());
                wgpustate.draw_this_img().await;
            });
        }
    });

    rsx! {
        div { class: "image-container",
            style: if is_dragging() { "cursor: grabbing;" } else {"cursor: default;"},
            onwheel: move |evt| {
                let scroll_delta = get_scroll_value(evt.delta()) * -0.01;
                zoom_signal.set((zoom_signal() + scroll_delta as i64).max(zoom_limits().0).min(zoom_limits().1));
            },
            onmousedown: move |evt| {
                is_dragging.set(true);
                start_position.set((evt.coordinates().client().x, evt.coordinates().client().y));
                viewport_size.set(get_viewport_size());
            },
            onmouseup: move |_| {
                is_dragging.set(false);
            },
            onmousemove: move |evt| {
                if is_dragging() && !image_data().is_none() {
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
                                image_data.set(Some(img));
                            },
                            Err(err) => {println!("{err:?}");}
                        }
                    }
                });
            },

            match image_data().is_none() {
                false => {
                    rsx!(
                    div { class: "image-inner",
                        canvas {
                            id: "image-board",
                            draggable: false,
                            width: format!("{}px",image_size().0),
                            height: format!("{}px",image_size().1),
                            style: format!("transform: scale({}) translate({}px, {}px);", scale_value, translation().0 / scale_value, translation().1 / scale_value),
                        }
                    }
                )
                },
                true => rsx!(p {class: "text",
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
    let zoom_limits = use_context::<ImageZoom>().limits;
    let zoom_value = *zoom_signal.read();
    let mut progress_style = use_signal(|| String::from(""));

    use_effect(move || {
        let progress_percentage = zoom_signal() * 100 / zoom_limits().1;
        progress_style.set(format!("background: linear-gradient(to right, white {}%, white {}%, gray {}%)", progress_percentage, progress_percentage, progress_percentage));
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
