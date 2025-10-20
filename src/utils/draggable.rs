use std::rc::Rc;
use dioxus::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use crate::dioxusui::GLOBAL_WINDOW_HANDLE;
use web_sys::{Element, MouseEvent, console};

#[derive(Clone, Copy, PartialEq)]
pub struct DragState {
    pub start_position: Signal<(f64, f64)>,
    pub translation: Signal<(f64, f64)>,
    pub is_dragging: Signal<bool>,
    pub scale: Signal<f64>,
}

fn clamp_drag_delta(dx: &mut f64, dy: &mut f64, this_element: &Signal<Option<web_sys::Element>>, parent_element: &Option<Element>) -> Result<(), String> {
    let this_el_rect = this_element.read().clone().ok_or("Target element missing for bounded drag")?.get_bounding_client_rect();
    let parent_el_rect = parent_element.clone().ok_or("Parent element missing for bounded drag")?.get_bounding_client_rect();

    let this_left = this_el_rect.left();
    let this_right = this_el_rect.right();
    let this_top = this_el_rect.top();
    let this_bottom = this_el_rect.bottom();

    let parent_left = parent_el_rect.left();
    let parent_right = parent_el_rect.right();
    let parent_top = parent_el_rect.top();
    let parent_bottom = parent_el_rect.bottom();

    if this_left + *dx < parent_left {
        *dx = parent_left - this_left;
    }
    if this_right + *dx > parent_right {
        *dx = parent_right - this_right;
    }
    if this_top + *dy < parent_top {
        *dy = parent_top - this_top;
    }
    if this_bottom + *dy > parent_bottom {
        *dy = parent_bottom - this_bottom;
    }

    Ok(())
}

pub fn use_draggable(bound: bool, this_element: Signal<Option<web_sys::Element>>, parent_element: Option<Element>, scale: f64) -> DragState {
    let mut translation = use_signal(|| (0.0, 0.0));
    let mut start_position = use_signal(|| (0.0, 0.0));
    let is_dragging = use_signal(|| false);
    let mut scale_signal = use_signal(|| scale);

    // mouse move handler for dragging an element
    let drag_handle = move |event: MouseEvent| {
        if is_dragging() {
            let (start_x, start_y) = start_position();
            let mut dx = (event.client_x() as f64 - start_x) / scale_signal();
            let mut dy = (event.client_y() as f64 - start_y) / scale_signal();
            
            if bound {
                if let Err(err) = clamp_drag_delta(&mut dx, &mut dy, &this_element, &parent_element) {
                    console::log_1(&format!("Error during bounded dragging: {:?}", err).into());
                }
            }
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
        scale: scale_signal,
    }
}