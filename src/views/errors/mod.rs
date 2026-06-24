use dioxus::prelude::*;
pub mod notfound;
#[component]
pub fn Error(code: u32) -> Element {
    match code {
        404 => notfound::NotFound(),
        _ => notfound::NotFound(),
    }
}
