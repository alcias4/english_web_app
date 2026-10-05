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

        main {
            class: "flex flex-col items-center sm:p-4 sm:w-[650px] gap-5" ,
            Home {}
        }

    }
}

#[component]
fn Home() -> Element {
    let data_n = use_signal(|| read_data::load_verbs());
    let mut count = use_signal(|| 1 as u32);

    let button = r"
        cursor-pointer bg-[#8B5CF6] p-4 w-[150px]  rounded-xl hover:bg-[#A78BFA] hover:-translate-y-1 hover:-translate-x-1 transition-transform duration-300 ease-in-out
    ";
    rsx! {

        header {
            class: "flex flex-col items-center p-4",
            h1 { "English World"}
        }


        if let Some(verb) = data_n.read().verbs.values().find(|verb| verb.id == *count.read()) {
            div {
                class: "flex flex-col gap-4 p-4 min-h-[200px] w-full rounded-lg bg-[#1D1F27]",
                h2 {
                    class: "flex gap-4",
                    "English: {verb.verb}"

                    span {
                        "{verb.forms.verb_type}"
                    }

                    span {
                       class: "opacity-50",
                        "{verb.id}"
                    }

                }

                p {
                    class: "",
                    "Translate: {verb.spanish_translation.join(\", \")}"
                }

                ul {
                    li { "{verb.forms.simple_past}" }
                    li { "{verb.forms.past_participle}" }
                    li { "{verb.forms.ing_form}" }
                }


                p {
                    span { "Definition: "  }
                    "{verb.core_idea_definition}"
                }

                p {
                     span { "When to use: "  }
                    "{verb.context.when_to_use}"
                }



            }
        }

        section {
            class: "flex flex-row-reverse w-full justify-between" ,
            button {
                class: button,
                onclick:move |_| count += 1 ,"Next", 
            }
            button {
                class: button,
                onclick:move |_| { if *count.read()> 1 {count -= 1} } ,"Before", 
            }
        }

    }
}
