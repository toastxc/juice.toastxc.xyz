use crate::components::tree;

use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {

        pre { class: "text-xs font-extrabold leading-[1.1]",
            r#"   ___       _
  |_  |     (_)
    | |_   _ _  ___ ___
    | | | | | |/ __/ _ \
/\__/ / |_| | | (_|  __/_
\____/ \__,_|_|\___\___(_)"#
        }
        br {}
        p { "HRT Information and advice, for Aussies with informed consent." }

        tree::Index {}

        footer { class: "text-sm text-gray-300 absolute bottom-0 m-2",
            "If you find any errors or have suggestions, please forward to juice@toastxc.xyz"
        }

    }
}
