use std::rc::Rc;
use dioxus::prelude::*;
use web_sys::MouseEvent;
use crate::dioxusui::GLOBAL_WINDOW_HANDLE;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{Element, console};

#[derive(Clone, Copy)]
pub enum ResizeType {
    Top,
    Right,
    Bottom,
    Left,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

/// Struct for handling the values in [`use_resizeable`].
pub struct ResizeState {
    pub last_resize_x: Signal<f64>,
    pub last_resize_y: Signal<f64>,
    pub resize_direction: Signal<Option<ResizeType>>,
    pub width: Signal<f64>,
    pub height: Signal<f64>,
    pub translation: Signal<(f64, f64)>,
}

fn clamp_mouse_delta(dx: &mut f64, dy: &mut f64, this_element: &Signal<Option<web_sys::Element>>, parent_element: &Option<Element>, resize_dir: ResizeType) -> Result<(), String> {
    let this_el_rect = this_element.read().clone().ok_or("Target element missing for bounded resize")?.get_bounding_client_rect();
    let parent_el_rect = parent_element.clone().ok_or("Parent element missing for bounded resize")?.get_bounding_client_rect();

    let this_left = this_el_rect.left();
    let this_right = this_el_rect.right();
    let this_top = this_el_rect.top();
    let this_bottom = this_el_rect.bottom();

    let parent_left = parent_el_rect.left();
    let parent_right = parent_el_rect.right();
    let parent_top = parent_el_rect.top();
    let parent_bottom = parent_el_rect.bottom();

    match resize_dir {
        ResizeType::Left | ResizeType::BottomLeft | ResizeType::TopLeft => {
            if this_left + *dx < parent_left {
                *dx = parent_left - this_left;
            }
        },
        ResizeType::Right | ResizeType::BottomRight | ResizeType::TopRight => {
            if this_right + *dx > parent_right {
                *dx = parent_right - this_right;
            }
        },
        _ => {}
    }

    match resize_dir {
        ResizeType::Top | ResizeType::TopLeft | ResizeType::TopRight => {
            if this_top + *dy < parent_top {
                *dy = parent_top - this_top;
            }
        },
        ResizeType::Bottom | ResizeType::BottomLeft | ResizeType::BottomRight => {
            if this_bottom + *dy > parent_bottom {
                *dy = parent_bottom - this_bottom;
            }
        },
        _ => {}
    }

    Ok(())
}

/// Custom hook for enabling resizing of a DOM Element, including optional bounding.
/// 
/// The hook adds mouse event listeners to the browser window which will handle the calculation of the elements size and offset.
/// The calculated size and offset can be accessed through the returned [`ResizeState`] struct.
/// 
/// # Arguments
/// 
/// * `width` - initial width of the target DOM element.
/// * `height` - initial height of the target DOM element.
/// * `min_width` - optional minimum width of the target DOM element, defaults to 170.0.
/// * `min_height` - optional minimum height of the target DOM element, defaults to 200.0.
/// * `max_width` - optional maximum width of the target DOM element, defaults to 600.0.
/// * `max_height` - optional maximum height of the target DOM element, defaults to 300.0.
/// * `bound` - specify whether the target DOM element is bound by its parent DOM element.
/// * `this_element` - reactive Signal containing the target DOM element.
/// * `parent_element` - parent DOM element of the target DOM element.
/// 
/// # Returns
/// 
/// A [`ResizeState`] struct.
/// 
/// # Panics
/// 
/// Will panic if `bound` is `true`, but `this_element` or `parent_element` are `None`.
///
pub fn use_resizeable(width: f64, height: f64, min_width: Option<f64>, min_height: Option<f64>, max_width: Option<f64>, max_height: Option<f64>, bound: bool, this_element: Signal<Option<web_sys::Element>>, parent_element: Option<Element>) -> ResizeState {
    let mut resize_type: Signal<Option<ResizeType>> = use_signal(|| None);
    let mut last_resize_x = use_signal(|| 0.0);
    let mut last_resize_y = use_signal(|| 0.0);
    let mut width = use_signal(|| width);
    let mut height = use_signal(|| height);
    let mut translation = use_signal(|| (0.0, 0.0));

    let resize_handle = move |event: MouseEvent| {
        if let Some(resize_dir) = *resize_type.read() {
            let start_x = last_resize_x();
            let start_y = last_resize_y();
            let mut dx = event.client_x() as f64 - start_x;
            let mut dy = event.client_y() as f64 - start_y;
            
            let mut new_width = width();
            let mut new_height = height();

            let (mut tx, mut ty) = translation();

            if bound {
                match clamp_mouse_delta(&mut dx, &mut dy, &this_element, &parent_element, resize_dir) {
                    Ok(()) => {}
                    Err(err) => { console::log_1(&format!("{:?}", err).into()); }
                }
            }

            // calculate the horizontal resize value with translation
            match resize_dir {
                ResizeType::Left | ResizeType::TopLeft | ResizeType::BottomLeft => {
                    new_width -= dx;
                    tx += dx;
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
                    ty += dy;
                }
                ResizeType::Bottom | ResizeType::BottomLeft | ResizeType::BottomRight => {
                    new_height += dy;
                }
                _ => {}
            }

            if new_width >= min_width.unwrap_or(170.0)
                && new_width <= max_width.unwrap_or(600.0) {
                    width.set(new_width);
                    translation.set((tx, translation().1));
                    last_resize_x.set(event.client_x() as f64);
            }

            if new_height >= min_height.unwrap_or(200.0) && new_height <= max_height.unwrap_or(300.0) {
                height.set(new_height);
                translation.set((translation().0, ty));
                last_resize_y.set(event.client_y() as f64);
            }
        }
    };

    use_hook_with_cleanup(
        move || {
            let resize_closure = Rc::new(Closure::wrap(Box::new(resize_handle) as Box<dyn FnMut(_)>));
            let up_closure = Rc::new(Closure::wrap(Box::new(move |_event: MouseEvent| {
                resize_type.set(None);
            }) as Box<dyn FnMut(_)>));

            GLOBAL_WINDOW_HANDLE()
                .add_event_listener_with_callback(
                    "mousemove",
                    resize_closure.as_ref().as_ref().unchecked_ref(),
                )
                .unwrap();

            GLOBAL_WINDOW_HANDLE()
                .add_event_listener_with_callback(
                    "mouseup",
                    up_closure.as_ref().as_ref().unchecked_ref(),
                )
                .unwrap();

            (resize_closure, up_closure)
        },
        move |(resize_closure, up_closure)| {
            GLOBAL_WINDOW_HANDLE()
                .remove_event_listener_with_callback(
                    "mousemove",
                    resize_closure.as_ref().as_ref().unchecked_ref(),
                )
                .unwrap();

            GLOBAL_WINDOW_HANDLE()
                .remove_event_listener_with_callback(
                    "mouseup",
                    up_closure.as_ref().as_ref().unchecked_ref(),
                )
                .unwrap();
        }
    );

    ResizeState {
        last_resize_x,
        last_resize_y,
        resize_direction: resize_type,
        width,
        height,
        translation,
    }
}