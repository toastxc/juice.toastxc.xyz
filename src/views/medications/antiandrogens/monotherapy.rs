use crate::components::style::*;
use dioxus::prelude::*;

#[component]
pub fn Monotherapy() -> Element {
    rsx! {
           div { class: "my-1 border-2 bg-red-100 p-1 text-red-900 dark:bg-red-800 dark:text-red-50 dark:shadow-white",
            "monotherapy often requires exceeding recommended doses of estrogen"

        }

        p { class: "{Doc::TITLE} title-monotherapy" }

        p { class: "{Doc::HEADING} ", "What is monotherapy?" }

        p { "Monotherapy uses estradiol as the only antiandrogenic drug, this requires \
        a very high dose of estrogen. The practise is not very well studied." }


             p { class: "{Doc::HEADING} ", "Suitable Drugs" }

        p { "Many forms of estrogen either lack to potency needed to achieve monotherapy or are too dangerous. \
        Patches (you'll likely need to use multiple), injections (high dose), and implants are the most likely to work." }
    }
}
