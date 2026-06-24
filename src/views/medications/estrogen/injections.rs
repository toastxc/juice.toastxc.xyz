use crate::components::style::Doc;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Injections() -> Element {
    rsx! {
        p { class: "{Doc::TITLE} title-injections" }

        p { class: Doc::HEADING, "Esters" }
        p {
            "While there are "
            a {
                href: "https://transfemscience.org/articles/injectable-e2-meta-analysis/",
                class: Doc::LINK,
                "6 different esters"
            }

            ", in the known transiverse, only two of them are \
            commonly compounded and that's "
            a {
                href: "https://en.wikipedia.org/wiki/Estradiol_valerate",
                class: Doc::LINK,
                "valerate"
            }
            " and "
            a {
                href: "https://en.wikipedia.org/wiki/Estradiol_enanthate",
                class: Doc::LINK,
                "enanthate"
            }
            ". We recommend sticking to those two."
        }

        p { class: Doc::HEADING, "Availability" }
        p {
            a {
                href: "https://greendispensarycompounding.com/prescriptions/",
                class: Doc::LINK,
                "Green Pharmacy"
            }
            " can compound valerate, there have been availability issues in the past."
        }

        p { class: Doc::HEADING, "Cost" }
        p {
            "The cost of compounding + shipping is similar to that of implants ($100-200) \
            However injections don't last anywhere near as long."
        }

        p { class: Doc::HEADING, "Dosing" }
        p {
            "Frankly too complicated, check out this "
            a {
                href: "https://greendispensarycompounding.com/prescriptions/",
                class: Doc::LINK,
                "online calculator"
            }
            ". Click "

            Link {
                to: Route::LevelsRoute {
                    name: "estrogen".to_string(),
                },
                class: Doc::LINK,
                "here"
            }
            " for target levels"

        }

        p { class: Doc::HEADING, "Effectiveness" }
        p { "Anecdotally implants and injections yield the best results." }
    }
}
