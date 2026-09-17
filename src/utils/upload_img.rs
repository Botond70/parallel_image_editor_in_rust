use anyhow::Context;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as base64_engine;
use dioxus::html::FileData;
use dioxus::prelude::*;
use image::{DynamicImage, GenericImageView, load_from_memory};
use std::collections::VecDeque;
use std::io::Cursor;

const PREVIEW_MAX_WIDTH: u32 = 480;

async fn decode_uploaded_image(file: &FileData) -> anyhow::Result<(DynamicImage, String)> {
    let bytes = file.read_bytes().await.map_err(|err| {
        anyhow::anyhow!("failed to read {}: {err}", file.name())
    })?;

    let img = load_from_memory(&bytes)
        .with_context(|| format!("unsupported image format: {}", file.name()))?;

    let resized = img.resize(
        PREVIEW_MAX_WIDTH,
        u32::MAX,
        image::imageops::FilterType::Triangle,
    );
    let preview = DynamicImage::ImageRgb8(resized.to_rgb8());

    let mut jpeg = Cursor::new(Vec::new());
    preview
        .write_to(&mut jpeg, image::ImageFormat::Jpeg)
        .with_context(|| format!("failed to encode JPEG preview for {}", file.name()))?;

    let data_url = format!(
        "data:image/jpeg;base64,{}",
        base64_engine.encode(jpeg.into_inner())
    );
    Ok((img, data_url))
}

pub async fn decode_uploaded_images(
    files: Vec<FileData>,
) -> (VecDeque<DynamicImage>, VecDeque<String>) {
    let mut decoded = Vec::new();

    for file in files {
        match decode_uploaded_image(&file).await {
            Ok(image) => decoded.push(image),
            Err(err) => eprintln!("{err:#}"),
        }
    }

    decoded.into_iter().unzip()
}

pub fn upload_img(
    files: Vec<FileData>,
    mut image_size: Signal<(f64, f64)>,
    mut wgpu_on: Signal<bool>,
    mut ready_signal: Signal<bool>,
    mut zoom_signal: Signal<i64>,
    mut image_vector_base64: Signal<VecDeque<String>>,
    mut image_data_q: Signal<VecDeque<DynamicImage>>,
) {
    zoom_signal.set(100);

    spawn(async move {
        wgpu_on.set(false);
        ready_signal.set(false);
        let (mut image_datas, mut image_datas_base64) = decode_uploaded_images(files).await;
        if let Some(front) = image_datas.front() {
            image_size.set((front.dimensions().0 as f64, front.dimensions().1 as f64));
        }
        let mut img_vec = image_data_q();
        img_vec.append(&mut image_datas);
        image_data_q.set(img_vec);
        let mut img_vec_base64 = image_vector_base64();
        img_vec_base64.append(&mut image_datas_base64);
        image_vector_base64.set(img_vec_base64);
        wgpu_on.set(true);
    });
}
