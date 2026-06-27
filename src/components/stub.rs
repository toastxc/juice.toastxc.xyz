use crate::components::style::*;
use dioxus::prelude::*;
#[component]
pub fn Stub() -> Element {
    rsx! {
        p {class:"{Doc::CAUTION}", "document has not yet been written"}

    }
}
