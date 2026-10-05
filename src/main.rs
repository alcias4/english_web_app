use dioxus::prelude::*;
mod read_data;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Home {}
    }
}

#[component]
fn Home() -> Element {
    let data_n = use_signal(|| read_data::load_verbs());
    let mut count = use_signal(|| 1 as u32);

    rsx! {

        header {
            class: "flex flex-col items-center p-4",
            h1 { "English World"}
        }


        if let Some(verb) = data_n.read().verbs.values().find(|verb| verb.id == *count.read()) {
            div {
                class: "flex flex-col gap-4 bg-black p-4",
                h2 {
                    class: "flex gap-4",
                    "English: {verb.verb}"

                    span {
                       class: "opacity-50",
                        "{verb.id}"
                    }
                }

                p {
                    class: "",
                    "Translate: {verb.spanish_translation.join(\", \")}"
                }

                p {
                    "{verb.core_idea_definition}"
                }

            }
        }

        section {
            class: "flex flex-row-reverse w-full justify-between" ,
            button {
                class: "cursor-pointer",
                onclick:move |_| count += 1 ,"Next", }
            button {
                class: "cursor-pointer",
                onclick:move |_| { if *count.read()> 1 {count -= 1} } ,"Before", }
        }

    }
}
