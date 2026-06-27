use crate::components::style::TableGrey as Table;
use crate::components::style::*;
use dioxus::prelude::*;
#[component]
pub fn Other() -> Element {
    rsx! {
        p { class: "{Doc::TITLE} other-levels", "" }

        p {
            "HRT comes with certain risks and complications, measuring these is important \
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
        table { class: "{Table::TABLE} ",
            thead {

                tr { class: Table::HEAD,
                    th { class: Table::HEAD_CELL, "TEST NAME" }

                }
            }
            tbody {

                tr {
                    td { "HORMONE ENQUIRES" }

                }
                tr {
                    td { "ANDROGEN STUDY" }

                }

                tr {
                    td { "ELETROLYTES INC CREAT" }

                }

                tr {
                    td { "HEAM MASTER" }

                }
                tr {
                    td { "LIVER FUNCTION (LFT)" }

                }

            }
        }

    }
}
