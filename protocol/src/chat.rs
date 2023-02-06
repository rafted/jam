#[derive(TypedBuilder)]
#[derive(Serialize, Deserialize)]
pub struct ChatComponent {
    text: String,

    bold: Option<bool>,
    italic: Option<bool>,
    underlined: Option<bool>,
    strikethrough: Option<bool>,
    obfuscated: Option<bool>,
    font: Option<String>,

    color: Option<Color>,
    insertion: Option<String>,

    click_event: Option<&ClickEvent>,
    hover_event: Option<&HoverEvent>,

    extra: Vec<ChatComponent>,
}

#[derive(Serialize, Deserialize)]
pub struct ClickEvent {
    pub open_url: String,
    pub run_command: String,
    pub suggest_command: String,
    pub change_page: String,
    pub copy_to_clipboard: String,
}

#[derive(Serialize, Deserialize)]
pub struct HoverEvent {
    pub show_text: ChatComponent,
    // pub show_item: NBTItem,
    // pub show_entity: NBTEntity,
    // pub show_achievement: NBTAchievement
}

