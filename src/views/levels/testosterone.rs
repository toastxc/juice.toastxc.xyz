use crate::components::style::TableGrey as Table;
use crate::components::style::*;
use dioxus::prelude::*;
#[component]
pub fn Testosterone() -> Element {
    rsx! {
        p { class: "{Doc::TITLE} testosterone-levels", "" }

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

        table { class: Table::TABLE,
            thead {

                tr { class: Table::HEAD,
                    th { class: "{Table::HEAD_CELL}", "MARKER" }

                    th { class: Table::HEAD_CELL, "LEVEL" }
                    th { class: Table::HEAD_CELL, "RANGE" }
                    th { class: Table::HEAD_CELL, "UNITS" }
                }
            }
            tbody {

                tr {
                    td { "Testosterone" }
                    td { "0.6*" }
                    td { "0.4-2" }
                    td { "nmol/L" }
                }
                tr {
                    td { "SHBG" }
                    td { "40*" }
                    td { "25-120" }
                    td { "nmol/L" }
                }

            }
        }
        p { "*level chosen at random" }

    }
}
