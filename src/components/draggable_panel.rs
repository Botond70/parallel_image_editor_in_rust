use crate::dioxusui::GLOBAL_WINDOW_HANDLE;
use crate::state::app_state::HSVState;
use dioxus::prelude::{rsx, *};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{MouseEvent, console, window};
use crate::utils::{
    resizeable::{use_resizeable, ResizeType},
    draggable::{use_draggable, DragState}
}; 

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
    let nonesignal = use_signal(|| Option::None);
    let default_offset = (100.0, 100.0);
    let mut state = use_resizeable(500.0, 200.0, props.min_width, props.min_height, props.max_width, props.max_height, false, nonesignal, None);
    let mut drag_state = use_draggable();

    let panel_style = use_memo(move || {
        format!(
            "display: grid; transform: translate({}px, {}px); width: {}px; height: {}px;",
            state.translation.read().0 + drag_state.translation.read().0 + default_offset.0,
            state.translation.read().1 + drag_state.translation.read().1 + default_offset.1,
            state.width.read(),
            state.height.read()
        )
    });

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
                        drag_state.is_dragging.set(true);
                        drag_state.start_position.set((evt.client_coordinates().x, evt.client_coordinates().y));
                    },
                    onmouseup: move |_| {
                        drag_state.is_dragging.set(false);
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
                        drag_state.is_dragging.set(true);
                        drag_state.start_position.set((evt.client_coordinates().x, evt.client_coordinates().y));
                    },
                    onmouseup: move |_| {
                        drag_state.is_dragging.set(false);
                    },
                    {props.PanelContent},
                },
            }

        }
    }
}
