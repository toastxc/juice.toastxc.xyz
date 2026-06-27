use crate::components::style::TableGrey as Table;
use crate::components::style::*;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Estrogen() -> Element {
    rsx! {
        p { class: "{Doc::TITLE} estrogen-levels" }
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
                    td { "Estradiol" }
                    td { "400*" }
                    td { "250-1000" }
                    td { "pmol/L" }
                }

            }
        }
        p { "*level chosen at random" }

    }
}
