//! Stack-based undo/redo: full editor snapshots (images + related UI/GPU state).

use crate::state::app_state::{CropSignal, HSVState, ImageState, ResizeState, BlurState, BlurDirection, BlurMode, UndoRedoState};
use dioxus::{html::u::bleed, prelude::*};
use image::DynamicImage;
use std::collections::VecDeque;
use web_sys::console::log_1;

/// Max checkpoints kept in the undo stack (oldest dropped first).
pub const MAX_UNDO_STACK: usize = 40;

#[derive(Clone)]
pub struct EditorSnapshot {
    pub images: VecDeque<DynamicImage>,
    pub base64: VecDeque<String>,
    pub curr_index: usize,
    pub img_size: (f64, f64),
    pub resize_w: u32,
    pub resize_h: u32,
    pub left_applied: f32,
    pub right_applied: f32,
    pub top_applied: f32,
    pub bottom_applied: f32,
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
    pub blur_mode: BlurMode,
    pub blur_direction: BlurDirection,
    pub blur_window_size: u32,
}


pub fn capture_editor_snapshot(
    image: &ImageState,
    resize: &ResizeState,
    crop: &CropSignal,
    hsv: &HSVState,
    blur: &BlurState,
) -> EditorSnapshot {
    let images = (image.image_vector)();
    let curr_index = (image.curr_image_index)();
    let clamped_index = if images.is_empty() {
        0
    } else {
        curr_index.min(images.len() - 1)
    };
    EditorSnapshot {
        images,
        base64: (image.base64_vector)(),
        curr_index: clamped_index,
        img_size: (image.img_size)(),
        resize_w: (resize.width)(),
        resize_h: (resize.height)(),
        left_applied: (crop.left_applied)(),
        right_applied: (crop.right_applied)(),
        top_applied: (crop.top_applied)(),
        bottom_applied: (crop.bottom_applied)(),
        hue: (hsv.hue)(),
        saturation: (hsv.saturation)(),
        value: (hsv.value)(),
        blur_mode: (blur.mode)(),
        blur_direction: (blur.direction)(),
        blur_window_size: (blur.window_size)(),

    }
}

pub fn apply_editor_snapshot(
    snap: &EditorSnapshot,
    mut image: ImageState,
    mut resize: ResizeState,
    mut crop: CropSignal,
    mut hsv: HSVState,
    mut blur: BlurState,
    mut image_modified: Signal<bool>,
) {
    if !snap.images.is_empty() {
        image.image_vector.set(snap.images.clone());
        image.base64_vector.set(snap.base64.clone());
    }
    let clamped_index = if snap.images.is_empty() {
        0
    } else {
        snap.curr_index.min(snap.images.len() - 1)
    };
    image.curr_image_index.set(clamped_index);
    image.img_size.set(snap.img_size);
    resize.width.set(snap.resize_w);
    resize.height.set(snap.resize_h);
    crop.left_applied.set(snap.left_applied);
    crop.right_applied.set(snap.right_applied);
    crop.top_applied.set(snap.top_applied);
    crop.bottom_applied.set(snap.bottom_applied);
    hsv.hue.set(snap.hue);
    hsv.saturation.set(snap.saturation);
    hsv.value.set(snap.value);
    blur.mode.set(snap.blur_mode);
    blur.direction.set(snap.blur_direction);
    blur.window_size.set(snap.blur_window_size);
    image_modified.set(true);
}

/// Call immediately before a mutating action. Saves current state to undo and clears redo.
pub fn record_undo_checkpoint(
    mut undo_redo: UndoRedoState,
    image: &ImageState,
    resize: &ResizeState,
    crop: &CropSignal,
    hsv: &HSVState,
    blur: &BlurState,
) {
    let images = (image.image_vector)();
    if images.is_empty() {
        return;
    }
    let snap = capture_editor_snapshot(image, resize, crop, hsv, blur);
    let mut u = (undo_redo.undo_stack)();
    if u.len() >= MAX_UNDO_STACK {
        u.remove(0);
    }
    u.push(snap);
    (undo_redo.undo_stack).set(u);
    (undo_redo.redo_stack).set(Vec::new());
}

pub fn perform_undo(
    mut undo_redo: UndoRedoState,
    image: &ImageState,
    resize: &ResizeState,
    crop: &CropSignal,
    hsv: &HSVState,
    blur: &BlurState
) -> bool {
    
    log_1(&"Performing undo".into());
    let mut u = (undo_redo.undo_stack)();
    if u.is_empty() {
        return false;
    }
    let previous = u.pop().unwrap();
    (undo_redo.undo_stack).set(u);

    let current = capture_editor_snapshot(image, resize, crop, hsv, blur);
    let mut r = (undo_redo.redo_stack)();
    r.push(current);
    (undo_redo.redo_stack).set(r);

    apply_editor_snapshot(
        &previous,
        *image,
        *resize,
        *crop,
        *hsv,
        *blur,
        image.image_modified,
    );
    true
}

pub fn perform_redo(
    mut undo_redo: UndoRedoState,
    image: &ImageState,
    resize: &ResizeState,
    crop: &CropSignal,
    hsv: &HSVState,
    blur: &BlurState,
) -> bool {
    log_1(&"Performing redo".into());
    let mut r = (undo_redo.redo_stack)();
    if r.is_empty() {
        return false;
    }
    let next = r.pop().unwrap();
    (undo_redo.redo_stack).set(r);

    let current = capture_editor_snapshot(image, resize, crop, hsv, blur);
    let mut u = (undo_redo.undo_stack)();
    u.push(current);
    (undo_redo.undo_stack).set(u);

    apply_editor_snapshot(&next, *image, *resize, *crop, *hsv, *blur, image.image_modified);
    true
}
