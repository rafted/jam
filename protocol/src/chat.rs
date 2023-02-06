#[derive(TypedBuilder)]
#[derive(Serialize, Deserialize)]
pub struct ChatComponent {
    text: String,

    bold: bool,
    italic: bool,
    underlined: bool,
    strikethrough: bool,
    obfuscated: bool,
    font: String,

    color: Color,
    insertion: String,

    click_event: ClickEvent,
    hover_event: HoverEvent,

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

