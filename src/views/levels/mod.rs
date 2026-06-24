pub mod estrogen;
pub mod other;
pub mod testosterone;

use dioxus::prelude::*;

#[component]
pub fn LevelsRoute(name: String) -> Element {
    match name.as_str() {
        "estrogen" => estrogen::Estrogen(),
        "testosterone" => testosterone::Testosterone(),
        "other" => other::Other(),
        _ => {
            rsx! {
                estrogen::Estrogen {}
                other::Other {}
                testosterone::Testosterone {}
            }
        }
    }
}

#[component]
pub fn LevelsPage() -> Element {
    rsx! {
        estrogen::Estrogen {}
        testosterone::Testosterone {}
        other::Other {}
    }
}
