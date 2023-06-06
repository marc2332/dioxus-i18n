#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use unic_langid::LanguageIdentifier;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct Language {
    id: LanguageIdentifier,
    texts: Text,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum Text {
    Value(String),
    Texts(HashMap<String, Text>),
}

impl Default for Text {
    fn default() -> Self {
        Self::Texts(HashMap::default())
    }
}

impl Text {
    fn query(&self, steps: &mut Vec<&str>) -> Option<String> {
        match self {
            Text::Texts(texts) => {
                if steps.is_empty() {
                    return None;
                }

                let current_path = steps.join(".");

                let this_step = steps.remove(0);
                let deep = texts.get(this_step)?;
                let res = deep.query(steps);
                if res.is_none() {
                    let res_text = texts.get(&current_path);
                    if let Some(res_text) = res_text {
                        return res_text.query(steps);
                    }
                }
                res
            }
            Text::Value(value) => Some(value.to_owned()),
        }
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|_| ())
    }
}

impl Language {
    pub fn get_text(&self, path: &str, params: HashMap<String, String>) -> Option<String> {
        let mut steps = path.split('.').collect::<Vec<&str>>();

        let mut text = self.texts.query(&mut steps).unwrap_or_default();

        for (name, value) in params {
            text = text.replacen(&format!("{{{name}}}"), &value.to_string(), 1);
        }
        Some(text)
    }
}

fn main() {
    launch(app)
}

#[derive(Clone, Copy)]
struct UseI18<'a> {
    pub selected_language: UseSharedState<'a, LanguageIdentifier>,
    pub languages: UseSharedState<'a, Vec<Language>>,
}

impl<'a> UseI18<'a> {
    fn t_p(&self, id: &str, params: HashMap<String, String>) -> String {
        for language in self.languages.read().iter() {
            if language.id == *self.selected_language.read() {
                return language.get_text(id, params).unwrap_or_default();
            }
        }

        String::default()
    }

    fn t(&self, id: &str) -> String {
        self.t_p(id, HashMap::default())
    }

    fn set_language(&self, id: LanguageIdentifier) {
        *self.selected_language.write() = id;
    }
}

fn init_i18(
    cx: &ScopeState,
    selected_language: LanguageIdentifier,
    languages: impl FnOnce() -> Vec<Language>,
) {
    use_shared_state_provider(cx, || selected_language);
    use_shared_state_provider(cx, languages)
}

fn use_i18(cx: &ScopeState) -> UseI18 {
    let selected_language = use_shared_state::<LanguageIdentifier>(cx).unwrap();
    let languages = use_shared_state::<Vec<Language>>(cx).unwrap();

    UseI18 {
        selected_language,
        languages,
    }
}

static EN_US: &str = include_str!("./en-US.json");
static ES_ES: &str = include_str!("./es-ES.json");

#[allow(non_snake_case)]
fn Body(cx: Scope) -> Element {
    let i18 = use_i18(cx);

    let change_to_english = move |_| i18.set_language("en-US".parse().unwrap());
    let change_to_spanish = move |_| i18.set_language("es-ES".parse().unwrap());

    render!(
        Button {
            onclick: change_to_english,
            label {
                "English"
            }
        }
        Button {
            onclick: change_to_spanish,
            label {
                "Spanish"
            }
        }
        label {
            i18.t("messages.hello_world")
        }
        label {
            i18.t_p("messages.hello", HashMap::from([("name".into(), "Dioxus".into())]))
        }
    )
}

fn app(cx: Scope) -> Element {
    init_i18(cx, "en-US".parse().unwrap(), || {
        let en_us = Language::from_str(EN_US).unwrap();
        let es_es = Language::from_str(ES_ES).unwrap();
        vec![en_us, es_es]
    });

    render!(Body {})
}
