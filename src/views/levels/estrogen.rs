use dioxus::prelude::*;

#[component]
pub fn Estrogen() -> Element {
    rsx! {
        p { class: "text-2xl font-bold font-bold text-fuchsia-300 estrogen-levels",
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
        p { class: "text-1xl font-bold font-bold text-fuchsia-300", "Test Example" }
        hr {}
        pre { "MARKER     LEVEL     RANGE     UNITS" }
        pre { "Estradiol  400*       250-100   pmol/L" }
        hr {}
        p { "*level chosen at random" }
    }
}
