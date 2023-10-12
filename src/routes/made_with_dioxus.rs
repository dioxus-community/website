use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::components::ProjectGrid;
use crate::MADE_WITH_DIOXUS;

#[allow(non_snake_case)]
#[inline_props]
pub fn MadeWithDioxus(cx: Scope) -> Element {
    render! {
        div { class: "text-white",
            p { class: "text-white w-2/3 text-center mx-auto pb-4",
                "This is a list of "
                i { "apps" }
                " made with Dioxus. For a list of Dioxus libraries, check out "
                Link { class: "underline", to: "https://dioxuslabs.com/awesome", "Awesome Dioxus" }
                "."
                br {}
                br {}
                "Would you like to see your project here? Just edit the "
                code { "PROJECTS" }
                " constant in the "
                Link {
                    class: "underline",
                    to: "https://github.com/dioxus-community/dioxus-community.github.io/blob/main/src/projects.rs",
                    "src/made_by_dioxus"
                }
                " file and send us a pull request!"
            }
            div { class: "pl-24 pr-24", ProjectGrid { projects: &MADE_WITH_DIOXUS } }
        }
    }
}