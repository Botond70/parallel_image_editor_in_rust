use std::rc::Rc;
use dioxus::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use crate::dioxusui::GLOBAL_WINDOW_HANDLE;
use web_sys::MouseEvent;

pub struct DragState {
    pub start_position: Signal<(f64, f64)>,
    pub translation: Signal<(f64, f64)>,
    pub is_dragging: Signal<bool>,
}

pub fn use_draggable() -> DragState {
    let mut translation = use_signal(|| (0.0, 0.0));
    let mut start_position = use_signal(|| (0.0, 0.0));
    let is_dragging = use_signal(|| false);

    // mouse move handler for dragging an element
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
            GLOBAL_WINDOW_HANDLE()
                .remove_event_listener_with_callback(
                    "mousemove",
                    move_closure.as_ref().as_ref().unchecked_ref(),
                )
                .unwrap();

            GLOBAL_WINDOW_HANDLE()
                .remove_event_listener_with_callback(
                    "mouseup",
                    up_closure.as_ref().as_ref().unchecked_ref(),
                )
                .unwrap();
        },
    );

    DragState { 
        start_position,
        translation,
        is_dragging,
    }
}