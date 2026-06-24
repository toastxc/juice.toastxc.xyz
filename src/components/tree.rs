use crate::components::style::Tree;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Levels() -> Element {
    rsx! {
        a { class: Tree::STEM, "levels" }
        Link { to: Route::LevelsPage {}, class: Tree::LEAF, "Estrogen" }
        ul {
            li {
                Link {
                    to: Route::LevelsRoute {
                        name: "estrogen".to_string(),
                    },
                    class: Tree::LEAF,
                    "Estrogen"
                }

            }
            li {
                Link {
                    to: Route::LevelsRoute {
                        name: ">testosterone".to_string(),
                    },
                    class: Tree::LEAF,
                    "Testosterone"
                }

            }
            li {
                Link {
                    to: Route::LevelsRoute {
                        name: "other".to_string(),
                    },
                    class: Tree::LEAF,
                    "Other"
                }

            }
        }

    }
}
#[component]
pub fn Medications() -> Element {
    rsx! {
        a { class: Tree::STEM, "medications" }
        ul {
            li {
                Link { to: Route::EstrogenPage {}, class: Tree::LEAF, "Estrogen" }

            }
            li {
                Link { to: Route::AntiAndrogenPage {}, class: Tree::LEAF, "Anti Androgen" }

            }
            li {
                a { class: Tree::LEAF, "Other" }

            }

        }

    }
}
#[component]
pub fn Index() -> Element {
    rsx! {
        ul {
            li { Levels {} }
            li { Medications {} }

        }
    }
}
