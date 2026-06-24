use crate::components::style::Doc;
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Pbs() -> Element {
    rsx! {

        p { class: "{Doc::TITLE} title" }

        p { class: Doc::HEADING, "What is the PBS?" }
        p { "The PBS is a government run program that subsidises the cost of common medicines." }

        p { class: Doc::HEADING, "What does it cover" }
        p { "At this point its a very long list, here." }

        p { class: Doc::HEADING, "What do PBS medicines cost?" }
        p { "$25 or $7 if you're a concession card holder, for a 2 month prescription." }
    }
}
