use dioxus::prelude::*;

#[component]
pub fn Other() -> Element {
    rsx! {
        p { class: "text-2xl font-bold font-bold text-fuchsia-300 other-levels", "" }

        p {
            "HRT comes with certain risks and complications, measuring these is important\
        for staying on top of that."
        }

        br {}

        p { class: "text-1xl font-bold font-bold text-fuchsia-300", "ELETROLYTES INC CREAT" }
        p {
            "This test is used to measure your electrolytes as well as kidney health (both can be\
                    impacted by HRT)"
        }
        br {}

        p { class: "text-1xl font-bold font-bold text-fuchsia-300", "HEAM MASTER" }
        p {
            "Measures iron levels, platelet count and white cell count. Important for\
            blood clot risk assessment and immune health"
        }
        br {}

        p { class: "text-1xl font-bold font-bold text-fuchsia-300", "LIVER FUNCTION" }
        p { "Measures liver indicators, some medications can adversely effect the liver" }

        p { class: "text-1xl font-bold font-bold text-fuchsia-300", "" }

        br {}

        br {}
        p { class: "text-1xl font-bold font-bold text-fuchsia-300", "Every 3 months" }
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
