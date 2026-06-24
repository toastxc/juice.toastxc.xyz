use dioxus::prelude::*;
#[component]
pub fn NotFound() -> Element {
    rsx! {
        p { "404" }
    }
}
