use crate::state::app_state::{
    BlurDirection, BlurMode, BlurState, CropSignal, HSVState, ImageState, ResizeState, UndoRedoState,
};
use dioxus::prelude::*;
use image::DynamicImage;
use web_sys::console::log_1;

pub const MAX_UNDO_STACK: usize = 40;

#[derive(Clone, Copy)]
pub struct ParamSnapshot {
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

#[derive(Clone)]
pub struct ImageEditSnapshot {
    pub index: usize,
    pub image: DynamicImage,
    pub base64: String,
    pub img_size: (f64, f64),
    pub resize_w: u32,
    pub resize_h: u32,
    pub left_applied: f32,
    pub right_applied: f32,
    pub top_applied: f32,
    pub bottom_applied: f32,
}

#[derive(Clone)]
pub enum UndoEntry {
    Params(ParamSnapshot),
    ImageEdit(ImageEditSnapshot),
}

pub fn capture_params(
    resize: &ResizeState,
    crop: &CropSignal,
    hsv: &HSVState,
    blur: &BlurState,
) -> ParamSnapshot {
    ParamSnapshot {
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

fn capture_image_edit_at(
    image: &ImageState,
    resize: &ResizeState,
    crop: &CropSignal,
    index: usize,
) -> Option<ImageEditSnapshot> {
    let images = image.image_vector.read();
    if images.is_empty() {
        return None;
    }
    let index = index.min(images.len() - 1);
    let img = images.get(index)?.clone();
    let base64 = image.base64_vector.read().get(index)?.clone();
    Some(ImageEditSnapshot {
        index,
        image: img,
        base64,
        img_size: (image.img_size)(),
        resize_w: (resize.width)(),
        resize_h: (resize.height)(),
        left_applied: (crop.left_applied)(),
        right_applied: (crop.right_applied)(),
        top_applied: (crop.top_applied)(),
        bottom_applied: (crop.bottom_applied)(),
    })
}

fn apply_params(
    snap: &ParamSnapshot,
    mut resize: ResizeState,
    mut crop: CropSignal,
    mut hsv: HSVState,
    mut blur: BlurState,
) {
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
}

fn apply_image_edit(
    snap: &ImageEditSnapshot,
    mut image: ImageState,
    mut resize: ResizeState,
    mut crop: CropSignal,
) {
    {
        let mut images = image.image_vector.write();
        if let Some(slot) = images.get_mut(snap.index) {
            *slot = snap.image.clone();
        }
    }
    {
        let mut base64 = image.base64_vector.write();
        if let Some(slot) = base64.get_mut(snap.index) {
            *slot = snap.base64.clone();
        }
    }
    image.curr_image_index.set(snap.index);
    image.img_size.set(snap.img_size);
    resize.width.set(snap.resize_w);
    resize.height.set(snap.resize_h);
    crop.left_applied.set(snap.left_applied);
    crop.right_applied.set(snap.right_applied);
    crop.top_applied.set(snap.top_applied);
    crop.bottom_applied.set(snap.bottom_applied);
    image.image_modified.set(true);
}

fn apply_entry(
    entry: &UndoEntry,
    image: ImageState,
    resize: ResizeState,
    crop: CropSignal,
    hsv: HSVState,
    blur: BlurState,
) {
    match entry {
        UndoEntry::Params(params) => apply_params(params, resize, crop, hsv, blur),
        UndoEntry::ImageEdit(edit) => apply_image_edit(edit, image, resize, crop),
    }
}

fn push_undo_entry(mut undo_redo: UndoRedoState, entry: UndoEntry) {
    {
        let mut u = undo_redo.undo_stack.write();
        if u.len() >= MAX_UNDO_STACK {
            u.remove(0);
        }
        u.push(entry);
        undo_redo.undo_len.set(u.len());
    }
    undo_redo.redo_stack.write().clear();
    undo_redo.redo_len.set(0);
}

pub fn clear_undo_redo_history(mut undo_redo: UndoRedoState) {
    undo_redo.undo_stack.write().clear();
    undo_redo.redo_stack.write().clear();
    undo_redo.undo_len.set(0);
    undo_redo.redo_len.set(0);
}

fn capture_matching_current(
    entry: &UndoEntry,
    image: &ImageState,
    resize: &ResizeState,
    crop: &CropSignal,
    hsv: &HSVState,
    blur: &BlurState,
) -> Option<UndoEntry> {
    match entry {
        UndoEntry::Params(_) => Some(UndoEntry::Params(capture_params(resize, crop, hsv, blur))),
        UndoEntry::ImageEdit(edit) => {
            capture_image_edit_at(image, resize, crop, edit.index).map(UndoEntry::ImageEdit)
        }
    }
}

pub fn record_params_checkpoint(
    undo_redo: UndoRedoState,
    image: &ImageState,
    resize: &ResizeState,
    crop: &CropSignal,
    hsv: &HSVState,
    blur: &BlurState,
) {
    if image.image_vector.read().is_empty() {
        return;
    }
    push_undo_entry(
        undo_redo,
        UndoEntry::Params(capture_params(resize, crop, hsv, blur)),
    );
}

pub fn record_image_edit_checkpoint(
    undo_redo: UndoRedoState,
    image: &ImageState,
    resize: &ResizeState,
    crop: &CropSignal,
) {
    let index = (image.curr_image_index)();
    let Some(snap) = capture_image_edit_at(image, resize, crop, index) else {
        return;
    };
    push_undo_entry(undo_redo, UndoEntry::ImageEdit(snap));
}

pub fn perform_undo(
    mut undo_redo: UndoRedoState,
    image: &ImageState,
    resize: &ResizeState,
    crop: &CropSignal,
    hsv: &HSVState,
    blur: &BlurState,
) -> bool {
    log_1(&"Performing undo".into());
    let previous = {
        let mut u = undo_redo.undo_stack.write();
        let Some(prev) = u.pop() else {
            return false;
        };
        undo_redo.undo_len.set(u.len());
        prev
    };

    if let Some(current) = capture_matching_current(&previous, image, resize, crop, hsv, blur) {
        let mut r = undo_redo.redo_stack.write();
        r.push(current);
        undo_redo.redo_len.set(r.len());
    }

    apply_entry(&previous, *image, *resize, *crop, *hsv, *blur);
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
    let next = {
        let mut r = undo_redo.redo_stack.write();
        let Some(n) = r.pop() else {
            return false;
        };
        undo_redo.redo_len.set(r.len());
        n
    };

    if let Some(current) = capture_matching_current(&next, image, resize, crop, hsv, blur) {
        let mut u = undo_redo.undo_stack.write();
        if u.len() >= MAX_UNDO_STACK {
            u.remove(0);
        }
        u.push(current);
        undo_redo.undo_len.set(u.len());
    }

    apply_entry(&next, *image, *resize, *crop, *hsv, *blur);
    true
}
