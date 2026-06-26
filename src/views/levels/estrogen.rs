use dioxus::prelude::*;
use crate::components::style::*;
#[component]
pub fn Estrogen() -> Element {
    rsx! {
        p { class: "{Doc::TITLE} estrogen-levels",
            ""
        }

        p {
            "You should be getting blood tests (for estrogen and the others) once every 3 months. \
            The clinical standard range for a cis women is between "
            strong { "250pmol/L " }
            "and "
            strong { "1000pmol/L" }
            "."
        }

        br {}
        p { class: "{Doc::HEADING}", "Test Example" }
        hr {}
        pre { "MARKER     LEVEL     RANGE     UNITS" }
        pre { "Estradiol  400*       250-100   pmol/L" }
        hr {}
        p { "*level chosen at random" }
    }
}
