mod components;
mod page;
mod read_data;

use dioxus::prelude::*;

use components::header::Header;
use page::verbs::Verbs;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const OWN_CSS: Asset = asset!("/assets/own_class.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: OWN_CSS }

        main { class: "flex flex-col w-full items-center p-4 sm:p-4 sm:w-[600px] gap-5 relative",
            Home {}
        }

    }
}

#[component]
fn Home() -> Element {
    let data_verbs = use_signal(|| read_data::load_verbs());
    let mut  count = use_signal(|| 1 as u32);

    let active = use_signal(|| false);

    use_effect(move || {
        if active() {
            count.set(500);
        } else {
            count.set(1);
        }
    });

    rsx! {

        Header { active }
        Verbs { data_verbs, count }

    }
}
