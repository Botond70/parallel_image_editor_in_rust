mod customlib;
mod dioxusui;
mod renderer;

use crate::dioxusui::App;
use dioxus::prelude::*;
use std::any::Any;
use wgpu::{Features, Limits};

fn main() {
    #[cfg(target_arch = "wasm32")]
    dioxus::launch(App);
}
