use dioxus::prelude::*;
use crate::utils::{
    resizeable::{use_resizeable, ResizeType, ResizeState},
    draggable::{use_draggable, DragState}
}; 

#[derive(PartialEq, Clone, Props)]
pub struct DraggablePanelProps {
    #[props(default = 150.0)]
    pub width: f64,
    #[props(default = 100.0)]
    pub height: f64,
    #[props(default = 150.0)]
    pub min_width: f64,
    #[props(default = 100.0)]
    pub min_height: f64,
    #[props(default = 600.0)]
    pub max_width: f64,
    #[props(default = 400.0)]
    pub max_height: f64,
    pub title: String,
    pub PanelContent: Element,
}

#[component]
pub fn DraggableResizeablePanel(props: DraggablePanelProps) -> Element {
    let nonesignal = use_signal(|| Option::None);
    let default_offset = (100.0, 100.0);
    let default_scale = 1.0;
    let resize_state = use_resizeable(props.width, props.height, props.min_width, props.min_height, props.max_width, props.max_height, false, nonesignal, None, default_scale);
    let drag_state = use_draggable(false, nonesignal, None, default_scale);

    let panel_style = use_memo(move || {
        format!(
            "display: flex; transform: translate({}px, {}px); width: {}px; height: {}px;",
            resize_state.translation.read().0 + drag_state.translation.read().0 + default_offset.0,
            resize_state.translation.read().1 + drag_state.translation.read().1 + default_offset.1,
            resize_state.width.read(),
            resize_state.height.read()
        )
    });

    rsx! {
        div { id: "hsv-panel-container",
            style: panel_style(),
            ResizeDraggables {
                resize_state,
            }
            Panel {
                drag_state,
                title: props.title,
                PanelContent: props.PanelContent,
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct ResizeDraggablesProps {
    resize_state: ResizeState,
}

#[component]
fn ResizeDraggables(mut props: ResizeDraggablesProps) -> Element {
    let mut handle_resize = move |evt: Event<MouseData>, resize_direction: Option<ResizeType>| {
        props.resize_state.last_resize_x.set(evt.client_coordinates().x);
        props.resize_state.last_resize_y.set(evt.client_coordinates().y);
        props.resize_state.resize_direction.set(resize_direction);
    };

    rsx! {
        div { id: "left-resize-draggable",
            onmousedown: move |evt| {
                handle_resize(evt, Some(ResizeType::Left));
            },
        }
        div { id: "right-resize-draggable",
            onmousedown: move |evt| {
                handle_resize(evt, Some(ResizeType::Right));
            }
        }
        div { id: "top-resize-draggable",
            onmousedown: move |evt| {
                handle_resize(evt, Some(ResizeType::Top));
            }
        }
        div { id: "bottom-resize-draggable",
            onmousedown: move |evt| {
                handle_resize(evt, Some(ResizeType::Bottom));
            }
        }
        div { id: "top-left-resize-draggable",
            onmousedown: move |evt| {
                handle_resize(evt, Some(ResizeType::TopLeft));
            }
        }
        div { id: "top-right-resize-draggable",
            onmousedown: move |evt| {
                handle_resize(evt, Some(ResizeType::TopRight));
            }
        }
        div { id: "bottom-right-resize-draggable",
            onmousedown: move |evt| {
                handle_resize(evt, Some(ResizeType::BottomRight));
            }
        }
        div { id: "bottom-left-resize-draggable",
            onmousedown: move |evt| {
                handle_resize(evt, Some(ResizeType::BottomLeft));
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
struct PanelProps {
    drag_state: DragState,
    title: String,
    PanelContent: Element,
}

#[component]
fn Panel(mut props: PanelProps) -> Element {
    rsx! {
        div { class: "panel-title",
            onmousedown: move |evt| {
                props.drag_state.is_dragging.set(true);
                props.drag_state.start_position.set((evt.client_coordinates().x, evt.client_coordinates().y));
            },
            onmouseup: move |_| {
                props.drag_state.is_dragging.set(false);
            },
            p { "{props.title}" },
        },
        div { class: "panel-content",
            {props.PanelContent}
        }
    }
}
