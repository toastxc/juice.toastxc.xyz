use crate::components::style::Doc;
use dioxus::prelude::*;

#[component]
pub fn Implants() -> Element {
    rsx! {
        p { class: "{Doc::TITLE} title-implants" }

        p { class: Doc::HEADING, "Availability" }
        p {
            "There are two companies who compound implants, "

            a {
                href: "https://www.stenlake.com.au/order-prescription/",
                class: Doc::LINK,
                "Stenlake"
            }

            " and complimentary compounding services ("
            a {
                href: "https://custommedicine.com.au/prescription-order/",
                class: Doc::LINK,
                "CCS"
            }
            "). "
        }

        p { class: Doc::HEADING, "Cost" }
        p {
            "As its "
            a {
                href: "https://en.wikipedia.org/wiki/Compounding",
                class: Doc::LINK,
                "compounded"
            }

            " you'll be paying more than PBS medications \
        although it may be cheaper than you think. for 100-150 dollars."
        }

        p { class: Doc::HEADING, "Dosing" }
        p {
            "Once your body acclimates to being on implants, it will last between "

            a {
                href: "https://auspath.org.au/wp-content/uploads/2022/05/AusPATH_Informed-Consent-Guidelines_DIGITAL.pdf",
                class: Doc::LINK,
                "6-12"
            }

            // https://auspath.org.au/wp-content/uploads/2022/05/AusPATH_Informed-Consent-Guidelines_DIGITAL.pdf
            " months. \
        The first often does not last as long (3 months). Implants are typically "

            a {
                href: "https://auspath.org.au/wp-content/uploads/2024/06/AusPATH_Informed-Consent-Guidelines_DIGI_2024_RLv01.pdf",
                class: Doc::LINK,
                "100mg"
            }
            " or "

            a {
                href: "https://auspath.org.au/wp-content/uploads/2024/06/AusPATH_Informed-Consent-Guidelines_DIGI_2024_RLv01.pdf",
                class: Doc::LINK,
                "200mg"
            }
            "."

        }

        p { class: Doc::HEADING, "Effectiveness" }
        p { "Anecdotally implants and injections yield the best results." }

        p { class: Doc::HEADING, "Compounders" }
        p {
            "Both Stenlake and CCS are well known to be very slow to respond at times, \
            this is normal. In my case CCS became awfully quiet after I had sent over the money... \
            In the end it was fine."
        }
    }
}
