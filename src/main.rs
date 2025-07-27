mod dioxusui;
mod renderer;

use dioxus::prelude::*;
use crate::dioxusui::App;
use wgpu::{Features, Limits};
// use winit::dpi::LogicalSize;
use winit::window::WindowAttributes;
use std::any::Any;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

const FEATURES: Features = Features::PUSH_CONSTANTS;
fn limits() -> Limits {
    Limits {
        max_push_constant_size: 16,
        ..Limits::default()
    }
}

fn window_attributes() -> WindowAttributes {
    WindowAttributes::default()
        .with_title("Editor")
        .with_min_inner_size(LogicalSize::new(800, 600))
}

fn main() {
    let config: Vec<Box<dyn Any>> = vec![
        Box::new(FEATURES),
        Box::new(limits()),
        Box::new(Config::default().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_title("Editor")
                    .with_min_inner_size(LogicalSize::new(800.0, 500.0)),
            )),
    ];
    dioxus_native::launch_cfg(App, Vec::new(), config);

    // LaunchBuilder::new()
    //     .with_cfg(
    //         Config::default().with_menu(None).with_window(
    //             WindowBuilder::new()
    //                 .with_title("Editor")
    //                 .with_min_inner_size(LogicalSize::new(800.0, 500.0)),
    //         ),
    //     )
    //     .launch(App);
}
