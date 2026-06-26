use crate::components::style::*;
use dioxus::prelude::*;

#[component]
pub fn Spironolactone() -> Element {
    rsx! {

        div { class: "{Doc::WARN}",
            "spiro is not very effective"
        }
        div { class: "{Doc::WARN}",
            "spiro causes your body to excrete sodium and retain potassium."
        }

        p { class: "{Doc::TITLE} title" }

        p { class: "{Doc::HEADING} ", "TLDR" }

        p {
            "This medication can't be recommended due to its strong impact of the renal system, and \
        lack of evidence for its effectiveness."
        }

        p { class: "{Doc::HEADING} ", "Effectiveness" }
        p { class: "italic",
            "\"Spironolactone shows limited and highly inconsistent effects on testosterone levels in clinical studies in cisgender men, cisgender women, and transfeminine people, with most studies finding no change in levels, some studies finding a decrease in levels, and a small number even finding an increase in levels\""
            ". "

        }
        p { "..." }
        p { class: "italic",
            "\"Consequent to spironolactone’s limited and inconsistent influence on testosterone levels and its relatively weak androgen receptor antagonism, estradiol plus spironolactone regimens will likely not be fully effective in terms of testosterone suppression for many transfeminine people."

        }
        a {
            href: "https://transfemscience.org/articles/transfem-intro/#spironolactone",
            class: Doc::LINK,
            "Source"
        }

    }
}
