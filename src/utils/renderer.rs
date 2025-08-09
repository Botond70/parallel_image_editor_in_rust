use crate::state::customlib::*;
use image::DynamicImage;

pub async fn start_wgpu(initial_image: &DynamicImage) -> State {
    #[cfg(target_arch = "wasm32")]
    {
        let renderer = State::new(initial_image).await;
        return renderer;
    }
}
