mod antagonists;
mod bicalutamide;
mod cyproterone;
mod monotherapy;
mod spironolactone;

use crate::components::style::{Doc, Table};
use crate::views::errors::Error;
use crate::Route;
use dioxus::core_macro::{component, rsx};
use dioxus::prelude::*;

#[component]
pub fn AntiAndrogenRoute(name: String) -> Element {
    match name.as_str() {
        "bicalutamide" => bicalutamide::Bicalutamide(),
        "cyproterone" => cyproterone::Cyproterone(),
        "spironolactone" => spironolactone::Spironolactone(),
        "antagonists" => antagonists::Antagonists(),
        "monotherapy" => monotherapy::Monotherapy(),
        _ => rsx! {
            Error { code: 404 }
        },
    }
}

#[component]
pub fn AntiAndrogenPage() -> Element {
    rsx! {
        p { class: "{Doc::TITLE} title", "" }

        p { class: "{Doc::HEADING}", "Types" }

        p { "There are 4 antiandrogens that are 'commonly' used in Australia." }

        table { class: Table::TABLE,
            thead {

                tr { class: Table::HEAD,
                    th { class: "{Table::HEAD_CELL}", "NAME" }

                    th { class: Table::HEAD_CELL, "PBS" }
                    th { class: Table::HEAD_CELL, "TGAA" }
                }
            }
            tbody {

                tr {
                    td { class: Table::LINK,
                        Link {
                            to: Route::AntiAndrogenRoute {
                                name: "cyproterone".to_string(),
                            },
                            "cyproterone"
                        }
                    }

                    td { "yes" }
                    td { "yes" }
                }
                tr {
                    td { class: Table::LINK,
                        Link {
                            to: Route::AntiAndrogenRoute {
                                name: "spironolactone".to_string(),
                            },
                            "spironolactone"
                        }
                    }

                    td { "yes" }
                    td { "yes" }
                }

                tr {
                    td { class: Table::LINK,
                        Link {
                            to: Route::AntiAndrogenRoute {
                                name: "bicalutamide".to_string(),
                            },
                            "bicalutamide"
                        }
                    }

                    td { "no" }
                    td { "yes" }
                }

                tr {
                    td { class: Table::LINK,
                        Link {
                            to: Route::AntiAndrogenRoute {
                                name: "antagonists".to_string(),
                            },
                            "antagonists"
                        }
                    }

                    td { "no" }
                    td { "yes" }
                }

                tr {
                    td { class: Table::LINK,
                        Link {
                            to: Route::AntiAndrogenRoute {
                                name: "monotherapy".to_string(),
                            },
                            "monotherapy"
                        }
                    }

                    td { "-" }
                    td { "-" }
                }

            }
        }

    }
}

#[component]
pub fn Template() -> Element {
    rsx! {
        p { class: "{Doc::TITLE} title" }

        p { class: "{Doc::HEADING} ", "Heading" }

        p { "text" }
    }
}
