use crate::components::style::*;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Cyproterone() -> Element {
    rsx! {

        div { class: "{Doc::WARN}",
            "Doctors will often perscribe over 12.5mg/day of cyproterone. This is excessive and dangerous"

        }

        p { class: "{Doc::TITLE} title" }

        p { class: "{Doc::HEADING} ", "Effectiveness" }
        p {
            "cyproterone is very antiandrogen, with estrogen (which is also antiandrogenic) \
        the drug can easily be used to achieve "

            Link {
                to: Route::LevelsRoute {
                    name: "estrogen".to_string(),
                },
                class: Doc::LINK,
                "clinical levels."
            }
        }

        p { class: Doc::HEADING, "Cost" }
        Link { to: Route::Pbs {}, class: Doc::LINK, "Under the PBS" }

        p { class: Doc::HEADING, "Dosing" }
        p {
            "Does can start at 12mg/day. (one quarter pill) "
            strong { "never exceed 12.5mg/day. " }
            p {
                "Doses can go as low as "

                a {
                    href: "https://pmc.ncbi.nlm.nih.gov/articles/PMC12573561/",
                    class: Doc::LINK,
                    "12mg/3days"
                }

                " or 12mg/week if estrogen levels are good. Doses of 5-10mg/day have a "

                a {
                    href: "https://transfemscience.org/articles/cpa-dosage/",
                    class: Doc::LINK,
                    "\"maximal or near-maximal effectiveness\""
                }

                " (but its hard to split a 50mg pill into 5, so 12mg is good enough)"
            }
        }

        p { class: "{Doc::HEADING} ", "Side effects" }
        p {
            "Cyproterone can have a profound impact on one's mental health, which increases with \
        dosage."
        }

    }
}
