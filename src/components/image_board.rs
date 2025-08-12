use crate::state::app_state::{ImageVec, ImageZoom, NextImage, WGPUSignal};
use crate::utils::renderer::start_wgpu;
use crate::utils::utils::{clamp_translate_value, get_scroll_value};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as base64_engine;
use dioxus::{html::HasFileData, prelude::*};
use image::{DynamicImage, GenericImageView, load_from_memory};
use std::collections::VecDeque;
use std::io::Cursor;
use web_sys::{console, window};

#[component]
pub fn ImageBoard() -> Element {
    let mut zoom_signal = use_context::<ImageZoom>().zoom;
    let zoom_limits = use_context::<ImageZoom>().limits;
    let scale_value: f64 = zoom_signal() as f64 / 100.0;
    let mut image_data_q = use_context::<ImageVec>().vector;
    let mut image_vector_base64 = use_context::<ImageVec>().base64_vector;
    let mut curr_index = use_context::<ImageVec>().curr_image_index;
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
    let mut wgpu_on = use_context::<WGPUSignal>().signal;
    let mut next_img_signal = use_context::<NextImage>().count;
    let mut draw_signal = use_signal(|| false);
    let mut ready_signal = use_signal(|| false);

    #[allow(unused)]
    use_effect(move || {
        if wgpu_on() {
            spawn(async move {
                let mut image_datas: VecDeque<DynamicImage> = image_data_q.cloned();
                console::log_1(&format!("Images : {}", image_datas.clone().len()).into());
                console::log_1(&format!("Current index: {}", curr_index() as u32).into());
                let first_img = image_datas.get(curr_index()).unwrap();
                let mut wgpustate = start_wgpu(first_img).await;
                wgpustate.set_index(curr_index() as u32);
                image_size.set((
                    first_img.dimensions().0 as f64,
                    first_img.dimensions().1 as f64,
                ));
                console::log_1(&"Started WGPU".into());
                console::log_1(&format!("Images: {}", image_datas.len()).into());
                let mut wgpusender = wgpustate.sender();
                for (i, img) in image_datas.iter().enumerate() {
                    if i > 0 {
                        wgpusender.send(img.clone());
                    }
                }
                wgpustate.receive().await;
                ready_signal.set(true);
                console::log_1(&"Drew first image".into());

                use_effect(move || {
                    if (!ready_signal()) {
                        draw_signal.set(false);
                    } else {
                        draw_signal.set(true);
                    }
                });

                use_effect(move || {
                    if *draw_signal.read() {
                        wgpustate.load_and_draw();
                        ready_signal.set(false);
                    } else if curr_index() != wgpustate.img_index as usize {
                        wgpustate.set_index(curr_index() as u32);
                        ready_signal.set(true);
                    }
                });
            });
        };
    });

    rsx! {
        div { class: "image-container",
            style: if is_dragging() { "cursor: grabbing;" } else {"cursor: default;"},
            onwheel: move |evt| {
                let mut scroll_delta = get_scroll_value(evt.delta());
                if scroll_delta > 0.0 {
                    scroll_delta = -5.0;
                } else {
                    scroll_delta = 5.0;
                }
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
                if is_dragging() && wgpu_on() {
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

                zoom_signal.set(100);

                spawn(async move {
                    wgpu_on.set(false);
                    draw_signal.set(false);
                    ready_signal.set(false);
                    next_img_signal.set(0);
                    let mut image_datas = VecDeque::<DynamicImage>::new();
                    let mut image_datas_base64 = VecDeque::<String>::new();
                    for file_name in file_names{if let Some(bytes) = file_engine.read_file(&file_name).await {
                        match load_from_memory(&bytes) {
                            Ok(img) => {
                                let mut png_bytes = Vec::new();
                                if let Err(err) = img.write_to(&mut Cursor::new(&mut png_bytes), image::ImageFormat::Png) {
                                    println!("Error during formatting: {err:?}");
                                }

                                let base64_str = base64_engine.encode(&png_bytes);

                                image_datas_base64.push_back(format!("data:image/png;base64, {}", base64_str));
                                image_datas.push_back(img);
                            },
                            Err(err) => {println!("UNSUPPORTED IMAGE FORMAT: {err:?}");}
                        }
                    }}
                    image_size.set((image_datas.front().unwrap().dimensions().0 as f64, image_datas.front().unwrap().dimensions().1 as f64));
                    let mut img_vec = image_data_q();
                    img_vec.append(&mut image_datas);
                    image_data_q.set(img_vec);
                    let mut img_vec_base64 = image_vector_base64();
                    img_vec_base64.append(&mut image_datas_base64);
                    image_vector_base64.set(img_vec_base64);
                    console::log_1(&format!("New vec count: {}", image_data_q().len()).into());
                    wgpu_on.set(true);
                });
            },

            match *wgpu_on.read() {
                true => {

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
                false => rsx!(p {class: "text",
                    "Drag and drop images here!"})
            }
        }
    }
}
