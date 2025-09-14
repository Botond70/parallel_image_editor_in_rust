use crate::dioxus_elements::geometry::WheelDelta;
use image::ImageEncoder; // Import the trait to bring encode into scope
use image::codecs::png::PngEncoder;
use image::{ImageBuffer, Rgba};
use wasm_bindgen::prelude::*;
use web_sys::js_sys;
use web_sys::{Blob, HtmlAnchorElement, HtmlElement, Url, window};

pub fn clamp_translate_value(
    tx: f64,
    ty: f64,
    viewport: (f64, f64),
    image_size: (f64, f64),
) -> (f64, f64) {
    (
        tx.min(image_size.0 + viewport.0)
            .max(-image_size.0 - viewport.0),
        ty.min(image_size.1 + viewport.1)
            .max(-image_size.1 - viewport.1),
    )
}

pub fn get_scroll_value(delta: WheelDelta) -> f64 {
    match delta {
        WheelDelta::Pixels(pixels) => pixels.y,
        WheelDelta::Lines(lines) => lines.y,
        WheelDelta::Pages(pages) => pages.y,
        _ => 0.0,
    }
}

#[wasm_bindgen]
pub fn save_png(buffer: Vec<u8>, width: u32, height: u32, filename: String) {
    let img_buf =
        ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, buffer).expect("Invalid buffer size");

    let mut png_data = Vec::new();
    PngEncoder::new(&mut png_data)
        .write_image(
            &img_buf,
            img_buf.width(),
            img_buf.height(),
            image::ColorType::Rgba8.into(),
        )
        .expect("Failed to encode PNG");

    let array = js_sys::Uint8Array::from(png_data.as_slice());
    let blob = Blob::new_with_u8_array_sequence(&js_sys::Array::of1(&array)).unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();

    let window = window().unwrap();
    let document = window.document().unwrap();
    let a = document.create_element("a").unwrap();
    a.set_attribute("href", &url).unwrap();
    a.set_attribute("download", &filename).unwrap();
    document.body().unwrap().append_child(&a).unwrap();
    let a_elem: HtmlElement = a.unchecked_into();
    a_elem.click();
    document.body().unwrap().remove_child(&a_elem).unwrap();
    Url::revoke_object_url(&url).unwrap();
}

pub fn save_file_via_dialog(buffer: Vec<u8>, width: u32, height: u32, filename: String) {
    let img_buf =
        ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, buffer).expect("Invalid buffer size");

    let mut png_data = Vec::new();
    PngEncoder::new(&mut png_data)
        .write_image(
            &img_buf,
            img_buf.width(),
            img_buf.height(),
            image::ColorType::Rgba8.into(),
        )
        .expect("Failed to encode PNG");

    let array = js_sys::Uint8Array::from(png_data.as_slice());
    let blob_parts = js_sys::Array::new();
    blob_parts.push(&array);
    let blob = Blob::new_with_u8_array_sequence(&blob_parts).unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();
    let document = window().unwrap().document().unwrap();
    let a = document
        .create_element("a")
        .unwrap()
        .dyn_into::<HtmlAnchorElement>()
        .unwrap();
    a.set_href(&url);
    a.set_download(&*filename);
    document.body().unwrap().append_child(&a).unwrap();
    a.click();
    document.body().unwrap().remove_child(&a).unwrap();
    Url::revoke_object_url(&url).unwrap();
}

pub fn align_to_256(x: u32) -> u32 {
    ((x + 255) / 256) * 256
}
