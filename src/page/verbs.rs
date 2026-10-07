use dioxus::prelude::*;
use dioxus_icons::lucide::{Lightbulb, AudioLines, Sprout};

use crate::read_data::{Verb, VerbDatabase};

#[component]
pub fn Verbs(data_verbs: Signal<VerbDatabase>, count: Signal<u32>) -> Element {
    let mut percentages = use_signal(|| 0.0);
    let length = data_verbs().verbs.len() as u32;
    let number = count();

    let data = data_verbs();



    let verb_selected = data.verbs.values().find(|verb| verb.id == number).cloned();

    use_effect(move || {
        percentages.set(count() as f32);
    });

    rsx! {
        if let Some(verb) = verb_selected {

            MainVerbInfo { verb: verb.clone(), length, percentage: percentages() }
            ContextVerb { verb: verb.clone() }

            SoundBox { verb: verb.clone() }
            WorthBox { verb }
        }

        ButtonVerbs {
            count,
            number,
            length,
            percentages,
        }

    }
}



#[component]
fn ButtonVerbs(count: Signal<u32> , number: u32, length: u32,percentages: Signal<f32>) -> Element  {
    rsx! {
        section { class: "flex flex-row-reverse w-full justify-between pt-[30px] gap-4 sticky bottom-0 py-4 ",
            button {
                class: "button shadow-xl shadow-[black]",
                class: if count() == 500 { "opacity-[0.5] cursor-none" },
                onclick: move |_| {

                    if count() < 500 {
                        count += 1;
                        let per = (number as f32 * 100.0) * 500.0;
                        percentages.set(per as f32);
                    }

                },
                "Next"
            }
            button {
                class: "cursor-pointer border-[1px] border-[#767882] p-4 w-[50%] rounded-xl transition-transform duration-300 ease-in-out font-bold bg-[#1f1f27] shadow-lg shadow-xl shadow-[black] hover:-translate-y-1 hover:-translate-x-1",
                class: if count() == 1 { "opacity-[0.8]" },
                onclick: move |_| {
                    if count() > 1 {
                        count -= 1;
                        let per = (number as f32 * 100.0) / 500 as f32;
                        percentages.set(per);

                    }
                },
                "Before"
            }
        }
    }
}



#[component]
fn WorthBox(verb: Verb) -> Element {
    rsx! {
        section { class: "flex flex-col w-full mt-6 bg-[#21222a] p-4 rounded-[20px] border-[1px] border-[#585963] py-8 gap-6 mb-[10px]",
            div { class: "flex items-center gap-4",
                span { class: "bg-[#213230] p-2 rounded-[10px]",
                    Sprout { stroke: "#23be5d" }
                }
                h3 { class: "font-bold text-[18px]", "Worth learning" }
            }

            div { class: "flex justify-between",
                p { class: "opacity-[0.5]", "Usage estimate" }
                div { class: "flex items-center gap-3",
                    for _ in 1..=verb.estimated_usage_1_5 {
                        span { class: "w-[10px] h-[10px] rounded-[5px] bg-[#23be5d]",
                            ""
                        }
                    }
                    p { class: "opacity-[0.5]", "{verb.estimated_usage_1_5}/5" }
                }
            }

            div { class: "flex justify-between",
                p { class: "opacity-[0.5]", "Study priority" }
                div { class: "flex items-center gap-3",
                    for _ in 1..=verb.study_priority_1_5 {
                        span { class: "w-[10px] h-[10px] rounded-[5px] bg-[#23be5d] ",
                            ""
                        }
                    }
                    p { class: "opacity-[0.5]", "{verb.study_priority_1_5}/5" }
                }
            }

            p { class: "opacity-[0.5] text-[14px]",
                "Learning estimates, not an exact frequency ranking."
            }
        }
    }
}


#[component]
fn SoundBox(verb: Verb) -> Element {
    rsx! {
        section { class: "flex flex-col w-full mt-6 bg-[#21222a] p-4 rounded-[20px] border-[1px] border-[#585963] py-8",
            div { class: "flex flex-col gap-4",
                div { class: "flex gap-4 items-center",
                    span { class: "bg-[#2d2941] p-[10px] rounded-[10px]",
                        AudioLines { stroke: "#a388f2" }
                    }
                    h3 { class: "font-bold text-[18px]", "Say it confidently" }
                }

                div { class: "flex flex-col gap-4",
                    h4 { class: "text-[30px]", "{verb.pronunciation.reference_us_ipa}" }
                    span { class: "text-[#9ea0a4] flex gap-2",
                        "Reading aid:"
                        p { class: "text-white",
                            "{verb.pronunciation.latin_american_reading_approximation}"
                        }
                    }
                    a {
                        class: "bg-[#2f313c] p-2 rounded-[10px] text-center hover:bg-[#3e375f]",
                        href: "{verb.pronunciation.us_uk_audio_reference}",
                        target: "_blank",
                        "Hear US & UK audio"
                    }
                }
            }
        }
    }
}



#[component]
fn ContextVerb(verb: Verb) -> Element {
    let navigation = vec!["Overview", "Examples", "Patterns", "Notes"];

    let explore_tense =vec!["Simple present", "Simple past", "Present perfect", "Future with will", "-ing from"]; 

    let mut button = use_signal(|| "Overview");

    let mut select_tense = use_signal(|| "Simple present".to_string());

    rsx! {
        nav { class: "flex w-full justify-between list-none",
            for n in navigation.into_iter() {
                li {
                    class: "cursor-pointer  w-[22%] text-center p-4  transition-all duration-200 ease-in-out border-b-[1px] border-[#0f1116]",
                    class: if button() == n { "border-b-[1px] text-[#a388f2] border-[#a388f2]" },
                    onclick: move |_| button.set(n),
                    "{n}"
                }
            }
        
        }

        if button() == "Overview" {
            section { class: "flex flex-col w-full gap-4",
                h3 { "When to use it" }
                p { class: "text-[#b6b6b9]", "{verb.context.when_to_use}" }

                div { class: "flex gap-4",
                    for situations in verb.context.typical_situations.iter() {
                        span { class: "bg-[#22242d] p-2 text-[14px] rounded-[5px] text-[#797b85] border-[1px]",
                            "{situations}"
                        }
                    }
                }
            
            }
        }

        if button() == "Examples" {
            section { class: "flex flex-col w-full gap-4",
                h3 { class: "text-[#797b85]", "Explore a tense" }

                select {
                    class: "w-full bg-[#22242d] p-2 text-[14px] rounded-[5px]  border-[1px] border-[#797b85]",
                    oninput: move |event| {
                        select_tense.set(event.value());
                    },
                    for opt in explore_tense.iter() {
                        option { value: "{opt}", "{opt}" }
                    }
                }

                if select_tense() == "Simple present" {
                    div { class: "flex flex-col gap-4",
                        p { class: "text-[#767881]",
                            "{verb.examples_by_tense.simple_present.grammar_use}"
                        }

                        for example in verb.examples_by_tense.simple_present.examples.iter() {
                            div { class: "bg-[#22242d] p-4 rounded-[12px] border-[1px] border-[#797b85]",
                                p { "{example.en}" }

                                if let Some(ref key) = example.key_vocabulary {
                                    span { class: "text-[14px] text-[#767881]", "Key vocabulary: {key}" }
                                }
                            }
                        }
                    }
                }

                // simple past
                if select_tense() == "Simple past" {
                    div { class: "flex flex-col gap-4",
                        p { class: "text-[#767881]",
                            "{verb.examples_by_tense.simple_past.grammar_use}"
                        }

                        for example in verb.examples_by_tense.simple_past.examples.iter() {
                            div { class: "bg-[#22242d] p-4 rounded-[12px] border-[1px] border-[#797b85]",
                                p { "{example.en}" }

                                if let Some(ref key) = example.key_vocabulary {
                                    span { class: "text-[14px] text-[#767881]", "Key vocabulary: {key}" }
                                }
                            }
                        }
                    }
                }

                // Present perfect
                if select_tense() == "Present perfect" {
                    div { class: "flex flex-col gap-4",
                        p { class: "text-[#767881]",
                            "{verb.examples_by_tense.present_perfect.grammar_use}"
                        }

                        for example in verb.examples_by_tense.present_perfect.examples.iter() {
                            div { class: "bg-[#22242d] p-4 rounded-[12px] border-[1px] border-[#797b85]",
                                p { "{example.en}" }

                                if let Some(ref key) = example.key_vocabulary {
                                    span { class: "text-[14px] text-[#767881]", "Key vocabulary: {key}" }
                                }
                            }
                        }
                    }
                }

                // Future with will
                if select_tense() == "Future with will" {
                    div { class: "flex flex-col gap-4",
                        p { class: "text-[#767881]",
                            "{verb.examples_by_tense.future_will.grammar_use}"
                        }

                        for example in verb.examples_by_tense.future_will.examples.iter() {
                            div { class: "bg-[#22242d] p-4 rounded-[12px] border-[1px] border-[#797b85]",
                                p { "{example.en}" }

                                if let Some(ref key) = example.key_vocabulary {
                                    span { class: "text-[14px] text-[#767881]", "Key vocabulary: {key}" }
                                }
                            }
                        }
                    }
                }

                // -ing form
                if select_tense() == "-ing from" {
                    div { class: "flex flex-col gap-4",
                        p { class: "text-[#767881]", "{verb.examples_by_tense.ing_form.grammar_use}" }

                        for example in verb.examples_by_tense.ing_form.examples.iter() {
                            div { class: "bg-[#22242d] p-4 rounded-[12px] border-[1px] border-[#797b85]",
                                p { "{example.en}" }

                                if let Some(ref key) = example.key_vocabulary {
                                    span { class: "text-[14px] text-[#767881]", "Key vocabulary: {key}" }
                                }
                            }
                        }
                    }
                }
            
            }
        }

        if button() == "Patterns" {

            div { class: "flex flex-col px-4 ",
                h3 { class: "w-full font-bold", "Words that work together" }
                for common_patter in verb.common_pairs_and_patterns.iter() {
                    div { class: "flex flex-col p-4 border-b-[1px] border-[#2f303c] gap-2",
                        h4 { class: "font-bold", "{common_patter.expression}" }
                        span { class: "p-2 bg-[#2d2e34] w-fit text-[14px] rounded-[10px] text-[#a185eb]",
                            "{common_patter.structure}"
                        }
                        p { class: "text-[#b6b6b9]", "{common_patter.meaning_or_usage_note}" }
                    }
                }
            }
        }

        // notes
        if button() == "Notes" {
            section { class: "flex flex-col gap-4",
                div { class: "flex flex-col gap-4",
                    h2 { class: "font-bold", "Keep in mind" }
                    p { class: "text-[#b6b6b9]", "{verb.notes}" }
                }

                div { class: "flex flex-col gap-2",
                    h3 { class: "font-bold", "Do not confuse" }
                    p { class: "text-[#b6b6b9]", "No contrast notes are included for this verb" }
                }
            }
        }
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

                div { class: " flex w-full h-[2px] bg-[#595b64] mt-4",

                    span {
                        class: " bg-[#875aee] h-[2px]",
                        style: "width: {percentage/5.0}%;",
                    }
                }

                h3 { class: "text-[#595b64] mt-6", "Let's make it click." }

                h2 { class: "text-[3.4rem] font-bold flex items-center gap-4",

                    "{verb.verb}"

                    span { class: "text-[15px] bg-[#2c253f] p-2 rounded-lg", "{verb.forms.verb_type}" }
                }

                p { class: "text-[#dedede] text-[20px] flex gap-4 items-center",

                    "{verb.pronunciation.reference_us_ipa}"

                    span { class: "text-[#595b64] text-[15px]", "US pronunciation" }
                }

                div { class: "flex flex-col gap-4 mt-4 pb-4",

                    h2 { "Spanish translation" }

                    div { class: "flex gap-4 flex-wrap",
                        for trans in verb.spanish_translation.iter() {
                            p { class: "bg-[#23252f] p-[5px] px-[10px] rounded-[10px] border-solid border-[1px] border-[#585963] ",
                                "{trans}"
                            }
                        }
                    }
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