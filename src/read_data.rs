use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct VerbDatabase {
    pub meta: Meta,
    pub verbs: HashMap<String, Verb>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Meta {
    pub title: String,
    pub total_verbs: u32,
    pub original_base: u32,
    pub verbs_added: u32,
    pub example_structure: String,
    pub tenses_and_forms: Vec<String>,
    pub pronunciation_note: String,
    pub pattern_note: String,
    pub frequency_note: String,
    pub confusion_note: String,
    pub language_policy: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Verb {
    pub id: u32,
    pub verb: String,

    pub spanish_translation: Vec<String>,

    pub core_idea_definition: String,

    pub context: Context,

    pub pronunciation: Pronunciation,

    pub forms: VerbForms,

    pub secondary_meanings_and_expressions: String,

    pub common_pairs_and_patterns: Vec<Pattern>,

    pub collocations_and_common_combinations: Vec<Collocation>,

    pub differences_and_do_not_confuse: Vec<String>,

    pub examples_by_tense: ExamplesByTense,

    pub estimated_usage_1_5: u8,

    pub study_priority_1_5: u8,

    pub notes: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Context {
    pub when_to_use: String,
    pub typical_situations: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Pronunciation {
    pub reference_us_ipa: String,
    pub latin_american_reading_approximation: String,
    pub us_uk_audio_reference: String,
    pub note: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct VerbForms {
    pub base: String,
    pub third_person_singular: String,
    pub simple_past: String,
    pub past_participle: String,
    pub ing_form: String,

    #[serde(rename = "type")]
    pub verb_type: String,

    pub special_present_forms: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Pattern {
    pub expression: String,
    pub structure: String,
    pub meaning_or_usage_note: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Collocation {
    pub expression: String,
    pub structure: String,
    pub meaning_or_use: String,
    pub example: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct ExamplesByTense {
    pub simple_present: TenseExamples,
    pub simple_past: TenseExamples,
    pub present_perfect: TenseExamples,
    pub future_will: TenseExamples,
    pub ing_form: TenseExamples,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TenseExamples {
    pub grammar_use: String,
    pub examples: Vec<Example>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Example {
    pub en: String,
    pub key_vocabulary: Option<String>,
}

pub fn load_verbs() -> VerbDatabase {
    let json = include_str!("data/data_verbs.json");

    serde_json::from_str(json).expect("No se pudo leer el JSON de verbos")
}
