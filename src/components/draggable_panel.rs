use crate::dioxusui::GLOBAL_WINDOW_HANDLE;
use crate::state::app_state::HSVState;
use dioxus::prelude::{rsx, *};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{MouseEvent, console, window};

#[derive(Clone, Copy)]
enum ResizeType {
    Top,
    Right,
    Bottom,
    Left,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(PartialEq, Clone, Props)]
pub struct DraggablePanelProps {
    pub min_width: Option<f64>,
    pub min_height: Option<f64>,
    pub max_width: Option<f64>,
    pub max_height: Option<f64>,
    pub title: String,
    pub header_visible: bool,
    pub PanelContent: Element,
}

#[component]
pub fn DraggablePanel(props: DraggablePanelProps) -> Element {
    let mut is_dragging = use_signal(|| false);
    let mut start_position = use_signal(|| (0.0, 0.0));
    let mut translation = use_signal(|| (100.0, 100.0));
    let mut resize_type: Signal<Option<ResizeType>> = use_signal(|| None);
    let mut last_resize_x = use_signal(|| 0.0);
    let mut last_resize_y = use_signal(|| 0.0);
    let mut width = use_signal(|| 500.0);
    let mut height = use_signal(|| 200.0);

    let panel_style = use_memo(move || {
        format!(
            "display: grid; transform: translate({}px, {}px); width: {}px; height: {}px;",
            translation().0,
            translation().1,
            width(),
            height()
        )
    });

    // mouse move handler for dragging a panel by the title bar
    let drag_handle = move |event: MouseEvent| {
        if is_dragging() {
            let (start_x, start_y) = start_position();
            let dx = event.client_x() as f64 - start_x;
            let dy = event.client_y() as f64 - start_y;
            start_position.set((event.client_x() as f64, event.client_y() as f64));
            let (tx, ty) = translation();
            translation.set((tx + dx, ty + dy));
        }
    };

    let resize_handle = move |event: MouseEvent| {
        if let Some(resize_dir) = *resize_type.read() {
            let start_x = last_resize_x();
            let start_y = last_resize_y();
            let dx = event.client_x() as f64 - start_x;
            let dy = event.client_y() as f64 - start_y;

            let mut new_width = width();
            let mut new_height = height();

            let (mut tx, mut ty) = translation();

            // calculate the horizontal resize value with translation
            match resize_dir {
                ResizeType::Left | ResizeType::TopLeft | ResizeType::BottomLeft => {
                    new_width -= dx;
                    if new_width >= props.min_width.unwrap_or(170.0)
                        && new_width <= props.max_width.unwrap_or(600.0)
                    {
                        tx += dx;
                    }
                }
                ResizeType::Right | ResizeType::TopRight | ResizeType::BottomRight => {
                    new_width += dx;
                }
                _ => {}
            }

            // calculate the vertical resize value with translation
            match resize_dir {
                ResizeType::Top | ResizeType::TopLeft | ResizeType::TopRight => {
                    new_height -= dy;
                    if new_height >= props.min_height.unwrap_or(200.0)
                        && new_height <= props.max_height.unwrap_or(300.0)
                    {
                        ty += dy;
                    }
                }
                ResizeType::Bottom | ResizeType::BottomLeft | ResizeType::BottomRight => {
                    new_height += dy;
                }
                _ => {}
            }

            if new_width >= props.min_width.unwrap_or(170.0)
                && new_width <= props.max_width.unwrap_or(600.0)
            {
                width.set(new_width);
                translation.set((tx, ty));
                last_resize_x.set(event.client_x() as f64);
            }

            if new_height >= props.min_height.unwrap_or(200.0)
                && new_height <= props.max_height.unwrap_or(300.0)
            {
                height.set(new_height);
                translation.set((tx, ty));
                last_resize_y.set(event.client_y() as f64);
            }
        }
    };

    use_hook_with_cleanup(
        move || {
            let move_closure = Rc::new(Closure::wrap(Box::new(drag_handle) as Box<dyn FnMut(_)>));
            let resize_closure =
                Rc::new(Closure::wrap(Box::new(resize_handle) as Box<dyn FnMut(_)>));

            GLOBAL_WINDOW_HANDLE()
                .add_event_listener_with_callback(
                    "mousemove",
                    resize_closure.as_ref().as_ref().unchecked_ref(),
                )
                .unwrap();

            GLOBAL_WINDOW_HANDLE()
                .add_event_listener_with_callback(
                    "mousemove",
                    move_closure.as_ref().as_ref().unchecked_ref(),
                )
                .unwrap();

            let mut drag = is_dragging.clone();
            let up_closure = Rc::new(Closure::wrap(Box::new(move |_event: MouseEvent| {
                drag.set(false);
                resize_type.set(None);
            }) as Box<dyn FnMut(_)>));

            GLOBAL_WINDOW_HANDLE()
                .add_event_listener_with_callback(
                    "mouseup",
                    up_closure.as_ref().as_ref().unchecked_ref(),
                )
                .unwrap();

            (move_closure, up_closure, resize_closure)
        },
        move |(move_closure, up_closure, resize_closure)| {
            if let Some(window) = window() {
                window
                    .remove_event_listener_with_callback(
                        "mousemove",
                        move_closure.as_ref().as_ref().unchecked_ref(),
                    )
                    .unwrap();
                window
                    .remove_event_listener_with_callback(
                        "mouseup",
                        up_closure.as_ref().as_ref().unchecked_ref(),
                    )
                    .unwrap();
                window
                    .remove_event_listener_with_callback(
                        "mousemove",
                        resize_closure.as_ref().as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        },
    );

    rsx! {
        div { id: "hsv-panel-container",
            style: panel_style(),
            div { id: "left-resize-draggable",
                onmousedown: move |evt| {
                    resize_type.set(Some(ResizeType::Left));
                    last_resize_x.set(evt.client_coordinates().x);
                },
            }
            div { id: "right-resize-draggable",
                onmousedown: move |evt| {
                    resize_type.set(Some(ResizeType::Right));
                    last_resize_x.set(evt.client_coordinates().x);
                }
            }
            div { id: "top-resize-draggable",
                onmousedown: move |evt| {
                    resize_type.set(Some(ResizeType::Top));
                    last_resize_y.set(evt.client_coordinates().y);
                }
            }
            div { id: "bottom-resize-draggable",
                onmousedown: move |evt| {
                    resize_type.set(Some(ResizeType::Bottom));
                    last_resize_y.set(evt.client_coordinates().y);
                }
            }
            div { id: "top-left-resize-draggable",
                onmousedown: move |evt| {
                    resize_type.set(Some(ResizeType::TopLeft));
                    last_resize_x.set(evt.client_coordinates().x);
                    last_resize_y.set(evt.client_coordinates().y);
                }
            }
            div { id: "top-right-resize-draggable",
                onmousedown: move |evt| {
                    resize_type.set(Some(ResizeType::TopRight));
                    last_resize_x.set(evt.client_coordinates().x);
                    last_resize_y.set(evt.client_coordinates().y);
                }
            }
            div { id: "bottom-right-resize-draggable",
                onmousedown: move |evt| {
                    resize_type.set(Some(ResizeType::BottomRight));
                    last_resize_x.set(evt.client_coordinates().x);
                    last_resize_y.set(evt.client_coordinates().y);
                }
            }
            div { id: "bottom-left-resize-draggable",
                onmousedown: move |evt| {
                    resize_type.set(Some(ResizeType::BottomLeft));
                    last_resize_x.set(evt.client_coordinates().x);
                    last_resize_y.set(evt.client_coordinates().y);
                }
            }
            if(props.header_visible) {
                div { class: "panel-title",
                    onmousedown: move |evt| {
                        is_dragging.set(true);
                        start_position.set((evt.client_coordinates().x, evt.client_coordinates().y));
                    },
                    onmouseup: move |_| {
                        is_dragging.set(false);
                    },
                    p { "{props.title}" },
                },
                div { class: "panel-content",
                    {props.PanelContent}
                }
            }
            else{
                div { class: "panel-content",
                    onmousedown: move |evt| {
                        is_dragging.set(true);
                        start_position.set((evt.client_coordinates().x, evt.client_coordinates().y));
                    },
                    onmouseup: move |_| {
                        is_dragging.set(false);
                    },
                    {props.PanelContent},
                },
            }

        }
    }
}
