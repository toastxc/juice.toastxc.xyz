use crate::components::style::Doc;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Pills() -> Element {
    rsx! {

        div { class: "{Doc::WARN}",
            "carries higher risk of blood clots"

        }
        br {}
        p { class: "{Doc::TITLE} title-pills" }

        p { class: Doc::HEADING, "Cost" }
        Link { to: Route::Pbs {}, class: Doc::LINK, "Under the PBS" }

        p { class: Doc::HEADING, "Dosing" }
        p {
            "Standard starting dose is between "

            a {
                href: "https://auspath.org.au/wp-content/uploads/2024/06/AusPATH_Informed-Consent-Guidelines_DIGI_2024_RLv01.pdf",
                class: Doc::LINK,
                "2.4mg/day"
            }

            ", with a recommended maximum dose of "

            a {
                href: "https://auspath.org.au/wp-content/uploads/2024/06/AusPATH_Informed-Consent-Guidelines_DIGI_2024_RLv01.pdf",
                class: Doc::LINK,
                "8mg/day"
            }
            "."
        }

        // 2-4mg/day
        p { class: Doc::HEADING, "Effectiveness" }
        p {
            "Anecdotally a lot of people struggle to maintain good levels. As the drug passes\
        through the liver some* of it is converted to estrone (a form of estrogen several orders \
        of magnitude weaker)."
        }

        p { class: Doc::HEADING, "Higher risks" }
        p {
            "While all feminizing HRT carries risk of increasing the chance of blood clots \
        and other problems. The risk is higher as the drug passes through the liver."
        }

    }
}
