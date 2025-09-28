use crate::dioxusui::GLOBAL_WINDOW_HANDLE;
use crate::state::app_state::HSVState;
use dioxus::prelude::{rsx, *};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{MouseEvent, console, window};
use crate::utils::resizeable::{use_resizeable, ResizeType}; 

#[derive(PartialEq, Clone, Props)]
pub struct DraggablePanelProps {
    pub min_width: Option<f64>,
    pub min_height: Option<f64>,
    pub max_width: Option<f64>,
    pub max_height: Option<f64>,
    pub title: String,
    pub header_visible: Option<bool>,
    pub PanelContent: Element,
}

#[component]
pub fn DraggablePanel(props: DraggablePanelProps) -> Element {
    let mut is_dragging = use_signal(|| false);
    let mut start_position = use_signal(|| (0.0, 0.0));
    let mut translation = use_signal(|| (100.0, 100.0));
    let nonesignal = use_signal(|| Option::None);

    let mut state = use_resizeable(500.0, 200.0, props.min_width, props.min_height, props.max_width, props.max_height, false, nonesignal, None);

    let panel_style = use_memo(move || {
        format!(
            "display: grid; transform: translate({}px, {}px); width: {}px; height: {}px;",
            state.translation.read().0 + translation().0,
            state.translation.read().1 + translation().1,
            state.width.read(),
            state.height.read()
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

    use_hook_with_cleanup(
        move || {
            let move_closure = Rc::new(Closure::wrap(Box::new(drag_handle) as Box<dyn FnMut(_)>));

            GLOBAL_WINDOW_HANDLE()
                .add_event_listener_with_callback(
                    "mousemove",
                    move_closure.as_ref().as_ref().unchecked_ref(),
                )
                .unwrap();

            let mut drag = is_dragging.clone();
            let up_closure = Rc::new(Closure::wrap(Box::new(move |_event: MouseEvent| {
                drag.set(false);
            }) as Box<dyn FnMut(_)>));

            GLOBAL_WINDOW_HANDLE()
                .add_event_listener_with_callback(
                    "mouseup",
                    up_closure.as_ref().as_ref().unchecked_ref(),
                )
                .unwrap();

            (move_closure, up_closure)
        },
        move |(move_closure, up_closure)| {
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
            }
        },
    );

    rsx! {
        div { id: "hsv-panel-container",
            style: panel_style(),
            div { id: "left-resize-draggable",
                onmousedown: move |evt| {
                    state.resize_direction.set(Some(ResizeType::Left));
                    state.last_resize_x.set(evt.client_coordinates().x);
                },
            }
            div { id: "right-resize-draggable",
                onmousedown: move |evt| {
                    state.resize_direction.set(Some(ResizeType::Right));
                    state.last_resize_x.set(evt.client_coordinates().x);
                }
            }
            div { id: "top-resize-draggable",
                onmousedown: move |evt| {
                    state.resize_direction.set(Some(ResizeType::Top));
                    state.last_resize_y.set(evt.client_coordinates().y);
                }
            }
            div { id: "bottom-resize-draggable",
                onmousedown: move |evt| {
                    state.resize_direction.set(Some(ResizeType::Bottom));
                    state.last_resize_y.set(evt.client_coordinates().y);
                }
            }
            div { id: "top-left-resize-draggable",
                onmousedown: move |evt| {
                    state.resize_direction.set(Some(ResizeType::TopLeft));
                    state.last_resize_x.set(evt.client_coordinates().x);
                    state.last_resize_y.set(evt.client_coordinates().y);
                }
            }
            div { id: "top-right-resize-draggable",
                onmousedown: move |evt| {
                    state.resize_direction.set(Some(ResizeType::TopRight));
                    state.last_resize_x.set(evt.client_coordinates().x);
                    state.last_resize_y.set(evt.client_coordinates().y);
                }
            }
            div { id: "bottom-right-resize-draggable",
                onmousedown: move |evt| {
                    state.resize_direction.set(Some(ResizeType::BottomRight));
                    state.last_resize_x.set(evt.client_coordinates().x);
                    state.last_resize_y.set(evt.client_coordinates().y);
                }
            }
            div { id: "bottom-left-resize-draggable",
                onmousedown: move |evt| {
                    state.resize_direction.set(Some(ResizeType::BottomLeft));
                    state.last_resize_x.set(evt.client_coordinates().x);
                    state.last_resize_y.set(evt.client_coordinates().y);
                }
            }
            if props.header_visible.unwrap_or(true) {
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
            else {
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
