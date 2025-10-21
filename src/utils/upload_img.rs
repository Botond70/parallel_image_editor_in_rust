use base64::Engine;
use base64::engine::general_purpose::STANDARD as base64_engine;
use dioxus::hooks;
use dioxus::html::FileEngine;
use dioxus::{html::HasFileData, prelude::*};
use image::{DynamicImage, GenericImageView, load_from_memory};
use std::collections::VecDeque;
use std::io::Cursor;
use std::sync::Arc;

pub fn upload_img(
    file_engine: Arc<dyn FileEngine>,
    mut image_size: Signal<(f64, f64)>,
    mut wgpu_on: Signal<bool>,
    mut ready_signal: Signal<bool>,
    mut zoom_signal: Signal<i64>,
    mut image_vector_base64: Signal<VecDeque<String>>,
    mut image_data_q: Signal<VecDeque<DynamicImage>>,
) {
    let file_names = file_engine.files();

    zoom_signal.set(100);

    spawn(async move {
        wgpu_on.set(false);
        ready_signal.set(false);
        let mut image_datas = VecDeque::<DynamicImage>::new();
        let mut image_datas_base64 = VecDeque::<String>::new();
        for file_name in file_names {
            if let Some(bytes) = file_engine.read_file(&file_name).await {
                match load_from_memory(&bytes) {
                    Ok(img) => {
                        let max_width = 480;
                        let resized =
                            img.resize(max_width, u32::MAX, image::imageops::FilterType::Triangle);
                        let rgb_img = resized.to_rgb8();
                        let dynamic_rgb = DynamicImage::ImageRgb8(rgb_img);
                        let mut cursor = Cursor::new(Vec::new());
                        if let Err(err) =
                            dynamic_rgb.write_to(&mut cursor, image::ImageFormat::Jpeg)
                        {
                            println!("Error during formatting: {err:?}");
                        }

                        let jpg_bytes = cursor.into_inner();
                        let base64_str = base64_engine.encode(&jpg_bytes);

                        image_datas_base64
                            .push_back(format!("data:image/jpeg;base64,{}", base64_str));
                        image_datas.push_back(img);
                    }
                    Err(err) => {
                        println!("UNSUPPORTED IMAGE FORMAT: {err:?}");
                    }
                }
            }
        }
        image_size.set((
            image_datas.front().unwrap().dimensions().0 as f64,
            image_datas.front().unwrap().dimensions().1 as f64,
        ));
        let mut img_vec = image_data_q();
        img_vec.append(&mut image_datas);
        image_data_q.set(img_vec);
        let mut img_vec_base64 = image_vector_base64();
        img_vec_base64.append(&mut image_datas_base64);
        image_vector_base64.set(img_vec_base64);
        wgpu_on.set(true);
    });
}
