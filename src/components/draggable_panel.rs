use dioxus::prelude::{rsx, *};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, console, window};
use crate::state::app_state::HSVState;
use crate::dioxusui::GLOBAL_WINDOW_HANDLE;
use std::rc::Rc;

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
    pub title: String,
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

            match resize_dir {
                ResizeType::Left => {
                    let new_width = width() - dx;

                    if new_width >= 170.0 && new_width <= 600.0 {
                        last_resize_x.set(event.client_x() as f64);
                        let (tx, ty) = translation();
                        translation.set((tx + dx, ty));
                        width.set(new_width);
                    } 
                }
                ResizeType::Right => {
                    let new_width = width() + dx;

                    if new_width >= 170.0 && new_width <= 600.0 {
                        last_resize_x.set(event.client_x() as f64);
                        width.set(new_width);
                    }
                }
                ResizeType::Top => {
                    let new_height = height() - dy;

                    if new_height >= 200.0 && new_height <= 300.0 {
                        let (tx, ty) = translation();
                        translation.set((tx, ty + dy));
                        last_resize_y.set(event.client_y() as f64);
                        height.set(new_height);
                    }
                }
                ResizeType::Bottom => {
                    let new_height = height() + dy;

                    if new_height >= 200.0 && new_height <= 300.0 {
                        last_resize_y.set(event.client_y() as f64);
                        height.set(new_height);
                    }
                }
                _ => {}
            }
        }
    };

    use_hook_with_cleanup(
        move || {
            let move_closure = Rc::new(Closure::wrap(Box::new(drag_handle) as Box<dyn FnMut(_)>));
            let resize_closure = Rc::new(Closure::wrap(Box::new(resize_handle) as Box<dyn FnMut(_)>));

            GLOBAL_WINDOW_HANDLE()
                .add_event_listener_with_callback(
                    "mousemove",
                    resize_closure.as_ref().as_ref().unchecked_ref()
                )
                .unwrap();

            GLOBAL_WINDOW_HANDLE()
                .add_event_listener_with_callback(
                    "mousemove",
                    move_closure.as_ref().as_ref().unchecked_ref()
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
                    up_closure.as_ref().as_ref().unchecked_ref()
                )
                .unwrap();

            (move_closure, up_closure, resize_closure)
        },
        move |(move_closure, up_closure, resize_closure)| {
            if let Some(window) = window() {
                window
                    .remove_event_listener_with_callback(
                        "mousemove",
                        move_closure.as_ref().as_ref().unchecked_ref()
                    )
                    .unwrap();
                window
                    .remove_event_listener_with_callback(
                        "mouseup",
                        up_closure.as_ref().as_ref().unchecked_ref()
                    )
                    .unwrap();
                window
                    .remove_event_listener_with_callback(
                        "mousemove", 
                        resize_closure.as_ref().as_ref().unchecked_ref()
                    ).unwrap();
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
    }

}