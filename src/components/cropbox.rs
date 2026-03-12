use dioxus::prelude::*;
use web_sys::console;
use crate::state::app_state::{ImageState, CropSignal};
use crate::utils::{
    resizeable::{use_resizeable, ResizeType},
    draggable::{use_draggable},
};
use crate::dioxusui::GLOBAL_WINDOW_HANDLE;

#[derive(PartialEq, Clone, Props)]
pub struct CropBoxProps {
    pub target_element: Signal<Option<web_sys::Element>>,
    pub parent: Signal<Option<web_sys::Element>>,
}

#[component]
pub fn CropBox(props: CropBoxProps) -> Element {
    let mut cropbox_element = use_context::<CropSignal>().cropbox_element;
    let mut crop_left = use_context::<CropSignal>().left;
    let mut crop_right = use_context::<CropSignal>().right;
    let mut crop_top = use_context::<CropSignal>().top;
    let mut crop_bottom = use_context::<CropSignal>().bottom;
    let (width, height) = (
        props
            .target_element
            .read()
            .as_ref()
            .expect("No target element found")
            .get_bounding_client_rect()
            .width(),
        props
            .target_element
            .read()
            .as_ref()
            .expect("No target element found")
            .get_bounding_client_rect()
            .height(),
    );

    let scale = use_context::<ImageState>().zoom;
    let scale_value = scale() as f64 / 100.0;
    let mut resize_state = use_resizeable(
        width / scale_value,
        height / scale_value,
        50.0,
        50.0,
        width / scale_value,
        height / scale_value,
        true,
        cropbox_element,
        props.parent.read().clone(),
        scale_value,
    );
    let mut drag_state = use_draggable(true, cropbox_element, props.parent.read().clone(), scale_value);

    use_effect(move || {
        resize_state.scale.set(scale() as f64 / 100.0);
        drag_state.scale.set(scale() as f64 / 100.0);
    });

    let cropbox_style = use_memo(move || {
        format!(
            "transform: translate({}px, {}px); width: {}px; height: {}px;",
            (resize_state.translation.read().0 + drag_state.translation.read().0),
            (resize_state.translation.read().1 + drag_state.translation.read().1),
            *resize_state.width.read(),
            *resize_state.height.read()
        )
    });

    use_effect(move || {
        let _ = cropbox_style();
        cropbox_element.set(Some(GLOBAL_WINDOW_HANDLE()
                    .document()
                    .unwrap()
                    .get_element_by_id("image-crop-box-container")
                    .expect("Couldn't find image-crop-box-container")));
    });

    let mut handle_onmount = move || {
        cropbox_element.set(Some(GLOBAL_WINDOW_HANDLE()
                    .document()
                    .unwrap()
                    .get_element_by_id("image-crop-box-container")
                    .expect("Couldn't find image-crop-box-container")));
    };

    let mut handle_resize = move |evt: Event<MouseData>, resize_direction: Option<ResizeType>| {
        resize_state.last_resize_x.set(evt.client_coordinates().x);
        resize_state.last_resize_y.set(evt.client_coordinates().y);
        resize_state.resize_direction.set(resize_direction);
    };

    console::log_1(&format!("Target image size width: {}, height: {}", width, height).into());
    console::log_1(&format!("Cropbox initial size width: {}, height: {}", *resize_state.width.read(), *resize_state.height.read()).into());

    // Calculate crop percentages
    use_effect(move || {
        if let (Some(target_el), Some(cropbox_el)) = (props.target_element.read().as_ref(), cropbox_element.read().as_ref()) {
            let image_rect = target_el.get_bounding_client_rect();
            let image_left = image_rect.left();
            let image_top = image_rect.top();
            let image_right = image_rect.right();
            let image_bottom = image_rect.bottom();
            let image_width = image_rect.width();
            let image_height = image_rect.height();

            let cropbox_rect = cropbox_el.get_bounding_client_rect();
            let cropbox_left = cropbox_rect.left();
            let cropbox_top = cropbox_rect.top();
            let cropbox_right = cropbox_rect.right();
            let cropbox_bottom = cropbox_rect.bottom();

            let left_crop_percent = (((cropbox_left - image_left) / image_width) * 100.0).max(0.0).min(100.0);
            let top_crop_percent = (((cropbox_top - image_top) / image_height) * 100.0).max(0.0).min(100.0);
            let right_crop_percent = (((image_right - cropbox_right) / image_width) * 100.0).max(0.0).min(100.0);
            let bottom_crop_percent = (((image_bottom - cropbox_bottom) / image_height) * 100.0).max(0.0).min(100.0);

            crop_left.set(left_crop_percent as f32 / 100.0);
            crop_top.set(top_crop_percent as f32 / 100.0);
            crop_right.set(right_crop_percent as f32 / 100.0);
            crop_bottom.set(bottom_crop_percent as f32 / 100.0);

            console::log_1(&format!("Image left: {}, top: {}, right: {}, bottom: {}", image_left, image_top, image_right, image_bottom).into());
            console::log_1(&format!("Cropbox left: {}, top: {}, right: {}, bottom: {}", cropbox_left, cropbox_top, cropbox_right, cropbox_bottom).into());
            console::log_1(&format!("Crop percentages - Left: {:.2}%, Top: {:.2}%, Right: {:.2}%, Bottom: {:.2}%", left_crop_percent, top_crop_percent, right_crop_percent, bottom_crop_percent).into());
        }
    });

    rsx! {
        div { id: "image-crop-box-container",
            style: cropbox_style,
            onmounted: move |_| {
                handle_onmount();
            },
            div {
                id: "crop-box-top-left",
                onmousedown: move |evt| {
                    handle_resize(evt, Some(ResizeType::TopLeft));
                }
            },
            div {
                id: "crop-box-top",
                onmousedown: move |evt| {
                    handle_resize(evt, Some(ResizeType::Top));
                }
            },
            div {
                id: "crop-box-top-right",
                onmousedown: move |evt| {
                    handle_resize(evt, Some(ResizeType::TopRight));
                }
            },
            div {
                id: "crop-box-left",
                onmousedown: move |evt| {
                    handle_resize(evt, Some(ResizeType::Left));
                }
            },
            div {
                id: "crop-box-middle",
                onmousedown: move |evt| {
                    drag_state.is_dragging.set(true);
                    drag_state.start_position.set((evt.client_coordinates().x, evt.client_coordinates().y));
                },
                onmouseup: move |_| {
                    drag_state.is_dragging.set(false);
                }
            },
            div {
                id: "crop-box-right",
                onmousedown: move |evt| {
                    handle_resize(evt, Some(ResizeType::Right));
                }
            },
            div {
                id: "crop-box-bottom-left",
                onmousedown: move |evt| {
                    handle_resize(evt, Some(ResizeType::BottomLeft));
                }
            },
            div {
                id: "crop-box-bottom",
                onmousedown: move |evt| {
                    handle_resize(evt, Some(ResizeType::Bottom));
                }
            },
            div {
                id: "crop-box-bottom-right",
                onmousedown: move |evt| {
                    handle_resize(evt, Some(ResizeType::BottomRight));
                }
            }
        }
    }
}
