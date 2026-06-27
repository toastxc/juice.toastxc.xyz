

use dioxus::prelude::*;
use crate::components::style::*;
#[component]
pub fn Stub() -> Element {
    rsx! {
        p {class:"{Doc::CAUTION}", "document has not yet been written"}

    }
}