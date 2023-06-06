#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use dioxus_i18n::*;
use freya::prelude::*;
use std::{collections::HashMap, str::FromStr};
use unic_langid::LanguageIdentifier;

fn main() {
    launch(app);
}

static EN_US: &str = include_str!("./en-US.json");

#[allow(non_snake_case)]
fn Body(cx: Scope) -> Element {
    let i18 = use_i18(cx);

    render!(
        label { translate!(i18, "messages.hello", name: "Dioxus")  }
    )
}

fn app(cx: Scope) -> Element {
    let en_us_code: LanguageIdentifier = "en-US".parse().unwrap();
    use_init_i18n(cx, en_us_code.clone(), en_us_code, || {
        let en_us = Language::from_str(EN_US).unwrap();
        vec![en_us]
    });

    render!(Body {})
}
