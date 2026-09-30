use anyhow::Context;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as base64_engine;
use dioxus::html::FileData;
use dioxus::prelude::*;
use image::{DynamicImage, GenericImageView, load_from_memory};
use crate::state::app_state::LoadedImage;
use std::collections::VecDeque;
use std::io::Cursor;

const PREVIEW_MAX_WIDTH: u32 = 480;

async fn decode_uploaded_image(file: &FileData) -> anyhow::Result<LoadedImage> {
    let bytes = file.read_bytes().await.map_err(|err| {
        anyhow::anyhow!("failed to read {}: {err}", file.name())
    })?;

    let image_format = image::guess_format(&bytes)
        .with_context(|| format!("failed to guess image format for {}", file.name()))?;

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

    Ok(LoadedImage { 
        image_data: img, 
        base64_data: data_url,
        image_format: image_format,
        file_name: file.name(),
    })
}

pub async fn decode_uploaded_images(
    files: Vec<FileData>,
) -> VecDeque<LoadedImage> {
    let mut decoded = VecDeque::new();

    for file in files {
        match decode_uploaded_image(&file).await {
            Ok(image) => decoded.push_back(image),
            Err(err) => eprintln!("{err:#}"),
        }
    }

    decoded
}

pub fn upload_img(
    files: Vec<FileData>,
    mut image_size: Signal<(f64, f64)>,
    mut wgpu_on: Signal<bool>,
    mut ready_signal: Signal<bool>,
    mut zoom_signal: Signal<i64>,
    mut image_data_q: Signal<VecDeque<LoadedImage>>,
) {
    zoom_signal.set(100);

    spawn(async move {
        wgpu_on.set(false);
        ready_signal.set(false);
        let mut image_datas = decode_uploaded_images(files).await;
        if let Some(front) = image_datas.front() {
            let (width, height) = front.image_data.dimensions();
            image_size.set((width as f64, height as f64));
        }
        let mut img_vec = image_data_q();
        img_vec.append(&mut image_datas);
        image_data_q.set(img_vec);
        wgpu_on.set(true);
    });
}
