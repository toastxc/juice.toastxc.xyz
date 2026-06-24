mod gel;
mod implants;
mod injections;
mod patches;
mod pills;
use crate::components::style::{Doc, Table};
use crate::Route;
use dioxus::core_macro::{component, rsx};
use dioxus::prelude::*;

#[component]
pub fn EstrogenRoute(name: String) -> Element {
    match name.as_str() {
        "implants" => implants::Implants(),
        "injections" => injections::Injections(),
        "gel" => gel::Gel(),
        "patches" => patches::Patches(),
        "pills" => pills::Pills(),
        _ => fourofour(),
    }
}
#[component]
pub fn fourofour() -> Element {
    rsx! {
        p { "404" }
    }
}

#[component]
pub fn EstrogenPage() -> Element {
    rsx! {
        p { class: "{Doc::TITLE} estrogen-medications", "" }

        p { class: "{Doc::HEADING}", "Forms" }

        p {
            "Generally
        speaking, there are five different forms of estrogen available. Gels, pills, patches,
        injections, and implants. I've made a table for comparing them"
        }

        table { class: Table::TABLE,
            thead {

                tr { class: Table::HEAD,
                    th { class: "{Table::HEAD_CELL}", "FORM" }

                    th { class: Table::HEAD_CELL, "HALF-LIFE" }
                    th { class: Table::HEAD_CELL, "PBS" }
                    th { class: Table::HEAD_CELL, "TGAA" }
                }
            }
            tbody {

                tr {
                    td { class: Table::LINK,
                        Link {
                            to: Route::EstrogenRoute {
                                name: "implants".to_string(),
                            },
                            "implants"
                        }
                    }
                    td { "3-12/months" }
                    td { "no" }
                    td { "no" }
                }

                tr {
                    td { class: Table::LINK,
                        Link {
                            to: Route::EstrogenRoute {
                                name: "injections".to_string(),
                            },
                            "injections"
                        }
                    }
                    td { "0.5-1/week" }
                    td { "no" }
                    td { "no" }
                }
                tr {
                    td { class: Table::LINK,
                        Link {
                            to: Route::EstrogenRoute {
                                name: "gel".to_string(),
                            },
                            "gel"
                        }
                    }
                    td { "0.5-1/days" }
                    td { "yes" }
                    td { "yes" }
                }

                tr {
                    td { class: Table::LINK,
                        Link {
                            to: Route::EstrogenRoute {
                                name: "patches".to_string(),
                            },
                            "patches"
                        }
                    }
                    td { "3-5/days" }
                    td { "yes" }
                    td { "yes" }
                }

                tr {
                    td { class: Table::LINK,
                        Link {
                            to: Route::EstrogenRoute {
                                name: "pills".to_string(),
                            },
                            "pills"
                        }
                    }
                    td { "0.3-1/day" }
                    td { "yes" }
                    td { "yes" }
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
