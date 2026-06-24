use dioxus::prelude::*;

#[component]
pub fn Testosterone() -> Element {
    rsx! {
        p { class: "text-2xl font-bold font-bold text-fuchsia-300 testosterone-levels",
            ""
        }

        p { class: "text-1xl font-bold font-bold text-fuchsia-300", "Total Testosterone" }
        p {
            "The standard clinical range for testosterone in cis women is between "
            strong { "0.4nmol/L " }
            "and "
            strong { "2nmol/L" }
            "."
        }
        br {}
        p { class: "text-1xl font-bold font-bold text-fuchsia-300", "SHBG" }
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
        p { class: "text-1xl font-bold font-bold text-fuchsia-300", "Test Example" }
        hr {}
        pre { "MARKER        LEVEL    RANGE    UNITS" }
        pre { "Testosterone  0.6*     0.4-2    nmol/L" }
        pre { "SHBG          40*      25-120   nmol/L" }
        hr {}
        p { "*level chosen at random" }
    }
}
