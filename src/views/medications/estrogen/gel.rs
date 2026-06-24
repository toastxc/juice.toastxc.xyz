use crate::components::style::Doc;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Gel() -> Element {
    rsx! {
        p { class: "{Doc::TITLE} title-gel" }

        p { class: Doc::HEADING, "Forms" }
        p { "Gel comes in two containers, packets and pumps. Either are fine." }

        p { class: Doc::HEADING, "Cost" }
        Link { to: Route::Pbs {}, class: Doc::LINK, "Under the PBS" }

        p { class: Doc::HEADING, "Dosing" }
        p { "Gel comes in premeasured packets of in a pump. Typical starting dose is, " }

        a {
            href: "https://auspath.org.au/wp-content/uploads/2024/06/AusPATH_Informed-Consent-Guidelines_DIGI_2024_RLv01.pdf",
            class: Doc::LINK,
            "1mg"
        }
        " up to "
        a {
            href: "https://auspath.org.au/wp-content/uploads/2024/06/AusPATH_Informed-Consent-Guidelines_DIGI_2024_RLv01.pdf",
            class: Doc::LINK,
            "2mg"
        }
        "."

        p { class: Doc::HEADING, "Effectiveness" }
        p { "Anecdotally pretty good." }
    }
}
