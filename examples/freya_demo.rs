#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_i18n::*;
use freya::prelude::*;
use std::{collections::HashMap, str::FromStr};
use unic_langid::LanguageIdentifier;

fn main() {
    launch_with_props(app, "freya + dioxus_i18n", (300.0, 200.0));
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
        label { translate!(i18, "messages.hello_world") }
        label { translate!(i18, "messages.hello", name: "Dioxus")  }
    )
}

fn app(cx: Scope) -> Element {
    let en_us_code: LanguageIdentifier = "en-US".parse().unwrap();
    use_init_i18n(cx, en_us_code.clone(), en_us_code, || {
        let en_us = Language::from_str(EN_US).unwrap();
        let es_es = Language::from_str(ES_ES).unwrap();
        vec![en_us, es_es]
    });

    render!(Body {})
}
