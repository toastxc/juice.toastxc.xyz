use crate::components::style::Doc;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Patches() -> Element {
    rsx! {

        div { class: "my-1 border-2 bg-red-100 p-1 text-red-900 dark:bg-red-800 dark:text-red-50 dark:shadow-white",
            "subject to supply shortages"

        }

        div { class: "my-1 border-2 bg-red-100 p-1 text-red-900 dark:bg-red-800 dark:text-red-50 dark:shadow-white",
            "can cause adverse skin reactions"
        }
        br {}
        p { class: "{Doc::TITLE} title-patches" }
        p { class: Doc::HEADING, "Forms" }
        p { "They come in weekly and biweekly forms, (although they often don't last a week)." }

        p { class: Doc::HEADING, "Cost" }
        Link { to: Route::Pbs {}, class: Doc::LINK, "Under the PBS" }

        p { class: Doc::HEADING, "Dosing" }
        p {
            "Typical starting dose for patches is between "

            a {
                href: "https://auspath.org.au/wp-content/uploads/2024/06/AusPATH_Informed-Consent-Guidelines_DIGI_2024_RLv01.pdf",
                class: Doc::LINK,
                "50-100mcg"
            }
            " twice weekly, with a maximum of "
            a {
                href: "https://auspath.org.au/wp-content/uploads/2024/06/AusPATH_Informed-Consent-Guidelines_DIGI_2024_RLv01.pdf",
                class: Doc::LINK,
                "200mcg"
            }
            "."

        }

        p { class: Doc::HEADING, "Effectiveness" }
        p {
            "Anecdotally a lot of people struggle to maintain good levels. There is \
        allegedly a big difference in effectiveness between brands. Not all brands \
        are under PBS."
        }

        p { class: Doc::HEADING, "Allergy" }
        p { "May cause skin rashes or irritaiton in some people." }

        p { class: Doc::HEADING, "Availability" }

        p {
            "There have been some serious shortages for patches over the last two years \
            it was bad enough that we can't recommend it."
        }
    }
}
