# dioxus_i18n 🦀

This is a `i18n` library for Dioxus applications, it's renderer-agnostic too.

## Usage

### Example with [`freya`](https://github.com/marc2332/freya)

```json
// en-US.json
{
    "id": "en-US",
    "texts": {
        "messages": {
            "hello": "Hello, {name}!"
        }
    }
}
```

```rust
// main.rs

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
```