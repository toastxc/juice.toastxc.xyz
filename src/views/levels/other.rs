use dioxus::prelude::*;
use crate::components::style::*;
#[component]
pub fn Other() -> Element {
    rsx! {
        p { class: "{Doc::TITLE} other-levels", "" }

        p {
            "HRT comes with certain risks and complications, measuring these is important\
        for staying on top of that."
        }

        br {}

        p { class: "{Doc::HEADING}", "ELETROLYTES INC CREAT" }
        p {
            "This test is used to measure your electrolytes as well as kidney health (both can be\
                    impacted by HRT)"
        }
        br {}

        p { class: "{Doc::HEADING}", "HEAM MASTER" }
        p {
            "Measures iron levels, platelet count and white cell count. Important for\
            blood clot risk assessment and immune health"
        }
        br {}

        p { class: "{Doc::HEADING}", "LIVER FUNCTION" }
        p { "Measures liver indicators, some medications can adversely effect the liver" }

        p { class: "{Doc::HEADING}", "" }

        br {}

        br {}
        p { class: "{Doc::HEADING}", "Every 3 months" }
        hr {}
        div { class: "text-sm",

            pre { "NAME                     MEASURES   " }
            pre { "HORMONE ENQUIRES         Estradiol" }
            pre { "ANDROGEN STUDY           Total T,SHBG" }
            pre { "ELETROLYTES INC CREAT    Electrolytes" }
            pre { "HEAM MASTER              iron/WCC/platelets " }
            pre { "LIVER FUNCTION           Liver indicators" }
        }
        hr {}
        p { "WCC: White Cell Count (immune cells)" }
        p { "Total T: total testosterone (NOT FREE)" }
    }
}
