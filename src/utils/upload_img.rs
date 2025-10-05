use crate::state::app_state::{
    DragSignal, HSVState, ImageVec, ImageZoom, NextImage, ResizeState, WGPUSignal,
};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as base64_engine;
use dioxus::html::FileEngine;
use dioxus::{html::HasFileData, prelude::*};
use image::{DynamicImage, GenericImageView, load_from_memory};
use std::collections::VecDeque;
use std::io::Cursor;
use std::sync::Arc;

pub fn upload_img(file_engine: Arc<dyn FileEngine>) {
    let mut image_size = use_context::<ImageZoom>().img_size;
    let mut wgpu_on = use_context::<WGPUSignal>().signal;
    let mut next_img_signal = use_context::<NextImage>().count;
    let mut ready_signal = use_context::<WGPUSignal>().ready_signal;
    let mut zoom_signal = use_context::<ImageZoom>().zoom;
    let mut image_vector_base64 = use_context::<ImageVec>().base64_vector;
    let mut image_data_q = use_context::<ImageVec>().vector;
    let file_names = file_engine.files();

    zoom_signal.set(100);

    spawn(async move {
        wgpu_on.set(false);
        ready_signal.set(false);
        next_img_signal.set(0);
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
