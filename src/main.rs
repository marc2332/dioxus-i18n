#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;
use std::collections::{HashMap, HashSet};
use unic_langid::LanguageIdentifier;

#[derive(Debug, Clone, Default)]
struct Language {
    id: LanguageIdentifier,
    texts: HashMap<String, String>,
}

impl Language {
    pub fn new(id: LanguageIdentifier) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn add_text(&mut self, id: impl ToString, text: impl ToString) {
        self.texts.insert(id.to_string(), text.to_string());
    }

    pub fn get_text(&self, id: &str, params: HashMap<String, String>) -> Option<String> {
        let mut text = self.texts.get(id)?.clone();
        for (name, value) in params {
            text = text.replacen(&format!("{{{name}}}"), &value.to_string(), 1);
        }
        Some(text)
    }
}

fn main() {
    launch(app)
}

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

#[allow(non_snake_case)]
fn Body(cx: Scope) -> Element {
    let i18 = use_i18(cx);
    render!(
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
        let mut language = Language::new("en-US".parse().unwrap());

        language.add_text("messages.hello_world", "Hello World!");
        language.add_text("messages.hello", "Hello {name}");

        vec![language]
    });

    render!(Body {})
}
