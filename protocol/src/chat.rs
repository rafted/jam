use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Serialize, Deserialize)]
pub struct ChatComponent<'a> {
    text: String,

    #[builder(default)]
    bold: Option<bool>,

    #[builder(default)]
    italic: Option<bool>,

    #[builder(default)]
    underlined: Option<bool>,

    #[builder(default)]
    strikethrough: Option<bool>,

    #[builder(default)]
    obfuscated: Option<bool>,

    #[builder(default)]
    font: Option<String>,

    // #[builder(default)]
    // color: Option<Color>,
    #[builder(default)]
    insertion: Option<String>,

    #[builder(default)]
    click_event: Option<ClickEvent>,

    #[builder(default)]
    hover_event: Option<HoverEvent<'a>>,

    #[builder(default)]
    extra: Option<Vec<ChatComponent<'a>>>,
}

#[derive(TypedBuilder, Serialize, Deserialize)]
pub struct ClickEvent {
    #[builder(default)]
    pub open_url: Option<String>,

    #[builder(default)]
    pub run_command: Option<String>,

    #[builder(default)]
    pub suggest_command: Option<String>,

    #[builder(default)]
    pub change_page: Option<String>,

    #[builder(default)]
    pub copy_to_clipboard: Option<String>,
}

#[derive(TypedBuilder, Serialize, Deserialize)]
pub struct HoverEvent<'a> {
    #[builder(default)]
    // pub show_text: Option<&'a ChatComponent<'a>>,
    phantom: PhantomData<&'a str>,
}
