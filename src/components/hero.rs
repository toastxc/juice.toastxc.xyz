use crate::components::tree;

use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {

        p { "here you'll find a basic guide on HRT medications (and what to do with them)" }

        tree::Index {}

    }
}
