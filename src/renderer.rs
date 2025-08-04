use crate::customlib::*;
use image::DynamicImage;
use wasm_bindgen::JsCast;
use web_sys::*;
use wgpu::util::DeviceExt;
use wgpu::*;

pub async fn start_wgpu(initial_image: DynamicImage) -> State {
    #[cfg(target_arch = "wasm32")]
    {
        use crate::customlib;

        let renderer = customlib::State::new(initial_image).await;
        return renderer;
    }
}
