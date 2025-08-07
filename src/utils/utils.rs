use crate::dioxus_elements::geometry::WheelDelta;

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
