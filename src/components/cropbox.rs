use dioxus::prelude::*;
use crate::state::app_state::ImageState;
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
    let mut cropbox = use_signal(|| None);
    let mut resize_state = use_resizeable(
        width / scale_value,
        height / scale_value,
        Some(50.0),
        Some(50.0),
        Some(width / scale_value),
        Some(height / scale_value),
        true,
        cropbox,
        props.parent.read().clone(),
        scale_value,
    );
    let mut drag_state = use_draggable(true, cropbox, props.parent.read().clone(), scale_value);

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

    let mut handle_onmount = move || {
        cropbox.set(Some(GLOBAL_WINDOW_HANDLE()
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
