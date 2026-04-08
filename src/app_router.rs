use crate::dioxusui::WorkSpace;
use dioxus::prelude::*;
use crate::components::gallery::Gallery;
use crate::components::not_found_page::NotFound;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    WorkSpace,

    #[route("/gallery")]
    Gallery,

    #[route("/:..segments")]
    NotFound {
        segments: Vec<String>,
    },
}