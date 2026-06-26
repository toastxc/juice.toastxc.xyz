use dioxus::prelude::*;
use crate::components::style::*;
#[component]
pub fn Testosterone() -> Element {
    rsx! {
        p { class: "{Doc::TITLE} testosterone-levels",
            ""
        }

        p { class: "{Doc::HEADING}", "Total Testosterone" }
        p {
            "The standard clinical range for testosterone in cis women is between "
            strong { "0.4nmol/L " }
            "and "
            strong { "2nmol/L" }
            "."
        }
        br {}
        p { class: "{Doc::HEADING}", "SHBG" }
        p {
            "SHBG binds to \
             sex hormones making them inactive in the body, ensure it is within clinical range. \
              Which is between "
            strong { "25nmol/L " }
            "and "
            strong { "120nmol/L" }
            "."
        }

        br {}
        p { class: "{Doc::HEADING}", "Test Example" }
        hr {}
        pre { "MARKER        LEVEL    RANGE    UNITS" }
        pre { "Testosterone  0.6*     0.4-2    nmol/L" }
        pre { "SHBG          40*      25-120   nmol/L" }
        hr {}
        p { "*level chosen at random" }
    }
}
