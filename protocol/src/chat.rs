#[derive(TypedBuilder)]
#[derive(Serialize, Deserialize)]
pub struct ChatComponent<'a> {
    text: String,

    bold: Option<bool>,
    italic: Option<bool>,
    underlined: Option<bool>,
    strikethrough: Option<bool>,
    obfuscated: Option<bool>,
    font: Option<String>,

    color: Option<Color>,
    insertion: Option<String>,

    click_event: Option<ClickEvent>,
    hover_event: Option<HoverEvent<'a>>,

    extra: Option<Vec<ChatComponent<'a>>>,
}

#[derive(Serialize, Deserialize)]
pub struct ClickEvent {
    pub open_url: Option<String>,
    pub run_command: Option<String>,
    pub suggest_command: Option<String>,
    pub change_page: Option<String>,
    pub copy_to_clipboard: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct HoverEvent<'a> {
    pub show_text: Option<&'a ChatComponent>,
    // pub show_item: Option<NBTItem>,
    // pub show_entity: Option<NBTEntity>,
    // pub show_achievement: Option<NBTAchievement>
}

