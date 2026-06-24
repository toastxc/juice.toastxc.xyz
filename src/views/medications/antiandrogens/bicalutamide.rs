use crate::components::style::*;
use dioxus::prelude::*;

#[component]
pub fn Bicalutamide() -> Element {
    rsx! {
        p { class: "{Doc::TITLE} title" }

        p { class: "{Doc::HEADING} ", "Heading" }

        p { "text" }
    }
}
