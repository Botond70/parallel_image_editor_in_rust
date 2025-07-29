mod dioxusui;
mod renderer;

use dioxus::prelude::*;
use crate::dioxusui::{App};
use wgpu::{Features, Limits};
use std::any::Any;

fn main() {
    #[cfg(target_arch = "wasm32")]
    dioxus::launch(App);
}
