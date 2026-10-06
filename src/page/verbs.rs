use dioxus::prelude::*;
use dioxus_icons::lucide::Lightbulb;

use crate::read_data::{Verb, VerbDatabase};

#[component]
pub fn Verbs(data_verbs: Signal<VerbDatabase>, count: Signal<u32>) -> Element {
    let mut percentages = use_signal(|| 0.0);
    let length = data_verbs.read().verbs.len() as u32;
    let number = *count.read();

    let data = data_verbs.read();

    let verb_selected = data.verbs.values().find(|verb| verb.id == number).cloned();

    rsx! {
        if let Some(verb) = verb_selected {

            MainVerbInfo {
                verb: verb.clone(),
                length,
                percentage: *percentages.read(),
            }
            ContextVerb { verb_prom: verb }
        }

        section { class: "flex flex-row-reverse w-full justify-between",
            button {
                class: "button",
                onclick: move |_| {
                    count += 1;

                    let per = (number as f32 * 100.0) / length as f32;
                    percentages.set(per);

                },
                "Next"
            }
            button {
                class: "button",
                onclick: move |_| {
                    if *count.read() > 1 {
                        count -= 1;
                        let per = (number as f32 * 100.0) / length as f32;
                        percentages.set(per);

                    }
                },
                "Before"
            }
        }
    }
}





#[component]
fn ContextVerb(verb_prom: Verb) -> Element {
    rsx! {
        nav { "hello world" }
    }
}


#[component]
fn MainVerbInfo(
    verb: Verb,
    length: u32,
    percentage: f32,
) -> Element {
    let special_present = verb
        .forms
        .special_present_forms
        .as_ref()
        .map(|forms| forms.join(" / "))
        .unwrap_or_else(|| "None".to_string());

    rsx! {
        section { class: "w-full",

            div {
                div { class: "flex w-full text-[#595b64] justify-between",

                    h2 { "YOUR VERB GUIDE" }

                    span { "{verb.id}/{length} verbs" }
                }

                div { class: "w-full h-[2px] bg-[#595b64] mt-4",

                    span {
                        class: "block bg-[#875aee] h-[2px]",
                        style: "width: {percentage}%;",
                    }
                }

                h3 { class: "text-[#595b64] mt-6", "Let's make it click." }

                h2 { class: "text-[60px] font-bold flex items-center gap-4",

                    "{verb.verb}"

                    span { class: "text-[15px] bg-[#2c253f] p-2 rounded-lg", "{verb.forms.verb_type}" }
                }

                p { class: "text-[#dedede] text-[20px] flex gap-4 items-center",

                    "{verb.pronunciation.reference_us_ipa}"

                    span { class: "text-[#595b64] text-[15px]", "US pronunciation" }
                }

                div { class: "flex flex-col bg-[#21222b] p-4 mt-6 gap-4 rounded-[15px] border-solid border-[#a388f2] border-l-[3px]",

                    h3 { class: "flex gap-4 font-bold text-[#a388f2] text-[12px] items-center",

                        Lightbulb { size: 20 }

                        "THE CORE IDEA"
                    }

                    p { "{verb.core_idea_definition}" }
                }

                h3 { class: "mt-6 font-bold w-full", "Know the forms" }

                ul { class: "grid grid-cols-3 gap-4 mt-4",

                    li { class: "li-verbs",

                        "Base form"

                        p { class: "p-verbs", "{verb.forms.base}" }
                    }

                    li { class: "li-verbs",

                        "Simple past"

                        p { class: "p-verbs", "{verb.forms.simple_past}" }
                    }

                    li { class: "li-verbs",

                        "Past participle"

                        p { class: "p-verbs", "{verb.forms.past_participle}" }
                    }

                    li { class: "li-verbs",

                        "-ing form"

                        p { class: "p-verbs", "{verb.forms.ing_form}" }
                    }

                    li { class: "li-verbs",

                        "He / she / it"

                        p { class: "p-verbs", "{verb.forms.third_person_singular}" }
                    }

                    li { class: "li-verbs",

                        "Special present"

                        p { class: "p-verbs", "{special_present}" }
                    }
                }
            }
        }
    }
}