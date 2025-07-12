pub mod dioxusui;

use dioxus::{
    html::{image, img},
    prelude::*,
};
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
use crate::dioxusui::App;

fn main() {
    LaunchBuilder::new()
        .with_cfg(
            Config::default().with_menu(None).with_window(
                WindowBuilder::new()
                    .with_title("Editor")
                    .with_min_inner_size(LogicalSize::new(800.0, 500.0)),
            ),
        )
        .launch(App);
}
