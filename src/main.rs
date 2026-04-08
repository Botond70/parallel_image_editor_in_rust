mod dioxusui;
mod state;
mod components;
mod utils;
mod app_router;

use dioxusui::App;
use dioxus::prelude::*;

fn main() {
    #[cfg(target_arch = "wasm32")]
    dioxus::launch(App);
}
