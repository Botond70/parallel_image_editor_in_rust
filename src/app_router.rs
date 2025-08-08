use crate::dioxusui::WorkSpace;
use dioxus::prelude::*;
use crate::components::gallery::Gallery;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    WorkSpace,

    #[route("/gallery")]
    Gallery,
}