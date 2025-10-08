use crate::components::cropbox::CropBox;
use crate::dioxusui::GLOBAL_WINDOW_HANDLE;
use crate::state::app_state::{
    DragSignal, HSVState, ImageVec, ImageZoom, NextImage, ResizeState, WGPUSignal,
};
use crate::state::customlib::{Filesave_config, State};
use crate::utils::renderer::start_wgpu;
use crate::utils::upload_img::upload_img;
use crate::utils::utils::{clamp_translate_value, get_scroll_value};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as base64_engine;
use dioxus::html::canvas::width;
use dioxus::html::g::{scale, transform_origin};
use dioxus::{html::HasFileData, prelude::*};
use image::{DynamicImage, GenericImageView, load_from_memory};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::Cursor;
use std::rc::Rc;
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
    let mut can_drag = use_context::<DragSignal>().can_drag;
    let mut start_position = use_signal(|| (0.0, 0.0));
    let get_viewport_size = || {
        let window = window().expect("No global window found.");
        let win_width = window.inner_width().unwrap();
        let win_height = window.inner_height().unwrap();
        (win_width.as_f64().unwrap(), win_height.as_f64().unwrap())
    };
    let mut viewport_size = use_signal(|| get_viewport_size());
    let mut image_size = use_context::<ImageZoom>().img_size;
    let mut wgpu_on = use_context::<WGPUSignal>().signal;
    let mut next_img_signal = use_context::<NextImage>().count;
    let mut ready_signal = use_context::<WGPUSignal>().ready_signal;
    let mut hue = use_context::<HSVState>().hue;
    let mut sat = use_context::<HSVState>().saturation;
    let mut val = use_context::<HSVState>().value;
    let zoom_speed = 1.15;
    let mut wgpu_state_signal = use_signal::<Option<Rc<RefCell<State>>>>(|| None);
    let mut save_signal = use_context::<WGPUSignal>().save_signal;

    let mut width_signal = use_context::<ResizeState>().width;
    let mut height_signal = use_context::<ResizeState>().height;

    let mut canvas_el = use_signal(|| None::<web_sys::Element>);
    let mut is_cropping = use_signal(|| false);
    let mut image_inner_el = use_signal(|| None::<web_sys::Element>);

    #[allow(unused)]
    use_effect(move || {
        if wgpu_on() {
            spawn(async move {
                hue.set(0.0);
                sat.set(0.0);
                val.set(0.0);
                let mut image_datas: VecDeque<DynamicImage> = image_data_q.cloned();
                console::log_1(&format!("Images : {}", image_datas.clone().len()).into());
                console::log_1(&format!("Current index: {}", curr_index() as u32).into());
                let first_img = image_datas.get(curr_index()).unwrap();
                let state = Rc::new(RefCell::new(start_wgpu(first_img).await));

                image_size.set((
                    first_img.dimensions().0 as f64,
                    first_img.dimensions().1 as f64,
                ));
                width_signal.set(first_img.dimensions().0);
                height_signal.set(first_img.dimensions().1);
                console::log_1(&"Started WGPU".into());
                console::log_1(&format!("Images: {}", image_datas.len()).into());
                let mut wgpusender = state.borrow().sender();
                for (i, img) in image_datas.iter().enumerate() {
                    if i > 0 {
                        wgpusender.send(img.clone());
                    }
                }
                state.borrow_mut().receive().await;
                state.borrow_mut().set_index(curr_index() as u32);
                state.borrow_mut().draw(true, None);
                wgpu_state_signal.set(Some(state.clone()));
                ready_signal.set(true);
                console::log_1(&"Drew first image".into());
            });
        };
    });

    use_effect(move || {
        // track hue
        let _hue = hue();
        let _saturation = sat();
        let _value = val();

        if wgpu_on() && ready_signal() {
            if let Some(wgpu_state_rc) = &*wgpu_state_signal.read() {
                let mut wgpu_state = wgpu_state_rc.borrow_mut();
                wgpu_state.draw(false, None);
                console::log_1(&"Triggered re-render from HSV change".into());
            }
        }
    });

    use_effect(move || {
        if wgpu_on() && save_signal() > 0 {
            if let Some(wgpu_state_rc) = &*wgpu_state_signal.read() {
                let mut wgpu_state = wgpu_state_rc.borrow_mut();
                wgpu_state.draw_to_texture(Filesave_config {
                    path: String::from("image.png"),
                });
                console::log_1(&"Triggered save from signal".into());
                save_signal.set(0);
            }
        } else if save_signal() > 0 {
            save_signal.set(0);
        }
    });

    use_effect(move || {
        if wgpu_on() && ready_signal() && width_signal() > 0 && height_signal() > 0 {
            if let Some(wgpu_state_rc) = &*wgpu_state_signal.read() {
                let mut wgpu_state = wgpu_state_rc.borrow_mut();

                wgpu_state.resize(width_signal(), height_signal());
                console::log_1(&"Triggered from resize signal".into());
                image_size.set((width_signal() as f64, height_signal() as f64));
                console::log_1(
                    &format!("image_size: {} x {}", image_size().0, image_size().1).into(),
                );
                wgpu_state.draw(false, None);
            }
        } else if width_signal() > 0 && height_signal() > 0 {
            width_signal.set(0);
            height_signal.set(0);
        }
    });

    rsx! {
        div { class: "image-container",
            style: if is_dragging() { "cursor: grabbing;" } else {"cursor: default;"},
            onwheel: move |evt| {
                if wgpu_on() {
                    evt.prevent_default();

                    let delta = get_scroll_value(evt.delta());

                    let old_scale = zoom_signal() as f64 / 100.0;
                    let new_scale = if delta > 0.0 {
                        (old_scale / zoom_speed).max(zoom_limits().0 as f64 / 100.0)
                    } else {
                        (old_scale * zoom_speed).min(zoom_limits().1 as f64 / 100.0)
                    };

                    // check if zoom is at limit
                    let new_zoom = (new_scale * 100.0).round() as i64;
                    if new_zoom == zoom_signal() {
                        return;
                    }

                    let canvas_el = GLOBAL_WINDOW_HANDLE().document().unwrap().get_element_by_id("image-board").expect("Cannot find canvas element.");
                    let rect = canvas_el.get_bounding_client_rect();
                    let rect_left = rect.left();
                    let rect_top = rect.top();

                    let client_x = evt.coordinates().client().x;
                    let client_y = evt.coordinates().client().y;

                    let (tx, ty) = translation();

                    // calculate the position of the mouse relative to our canvas
                    let local_trans_x = client_x - rect_left;
                    let local_trans_y = client_y - rect_top;

                    // calculate the new translation, taking scale into account
                    let ratio = new_scale / old_scale;
                    let new_tx = tx + (1.0 - ratio) * local_trans_x;
                    let new_ty = ty + (1.0 - ratio) * local_trans_y;

                    // clamp to viewport using new scale
                    let (clamped_tx, clamped_ty) = clamp_translate_value(
                        new_tx,
                        new_ty,
                        viewport_size(),
                        (image_size().0 * new_scale, image_size().1 * new_scale),
                    );

                    translation.set((clamped_tx, clamped_ty));
                    zoom_signal.set(new_zoom);
                }
            },
            onmousedown: move |evt| {
                if can_drag() {
                    is_dragging.set(true);
                    start_position.set((evt.coordinates().client().x, evt.coordinates().client().y));
                    viewport_size.set(get_viewport_size());
                }
            },
            onmouseleave: move |_| {
                is_dragging.set(false);
            },
            onmouseup: move |_| {
                is_dragging.set(false);
                is_cropping.set(true);
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
                let files = evt.files().unwrap();
                upload_img(
                    files,
                    image_size,
                    wgpu_on,
                    next_img_signal,
                    ready_signal,
                    zoom_signal,
                    image_vector_base64,
                    image_data_q,
                );
            }
            ,
            match *wgpu_on.read() {
                true => {
                    rsx!(
                        div { id: "image-inner",
                            style: format!(
                            "transform: translate({}px, {}px) scale({}); transform-origin: 0px 0px;",
                                translation().0,
                                translation().1,
                                zoom_signal() as f64 / 100.0
                            ),
                            onmounted: move |_| {
                                let image_inner = GLOBAL_WINDOW_HANDLE()
                                    .document()
                                    .unwrap()
                                    .get_element_by_id("image-inner")
                                    .expect("No image-inner element found");

                                image_inner_el.set(Some(image_inner));
                            },
                            canvas {
                                id: "image-board",
                                draggable: false,
                                width: format!("{}px",image_size().0),
                                height: format!("{}px",image_size().1),
                                onmounted: move |_| {
                                    canvas_el.set(Some(GLOBAL_WINDOW_HANDLE()
                                        .document()
                                        .unwrap()
                                        .get_element_by_id("image-board")
                                        .expect("No canvas found")));
                                },
                            },
                            if is_cropping() {
                                CropBox {
                                    target_element: canvas_el,
                                    parent: image_inner_el,
                                    scale: zoom_signal,
                                }
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
