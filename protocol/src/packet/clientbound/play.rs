use protocol_macro::PacketDef;

use crate::varint::VarInt;

#[derive(PacketDef)]
pub struct KeepAlivePacket {
    pub id: VarInt,
}

#[derive(PacketDef)]
pub struct JoinGamePacket {
    pub entity_id: i32,
    pub gamemode: u16,
    pub dimension: i8,
    pub difficulty: u8,
    pub max_players: u8,
    pub level_type: String,
    pub reduced_debug_info: bool,
}

// #[derive(PacketDef)]
// pub struct ChatMessage {
//     pub data: Chat,
//     pub position: i8
// }

#[derive(PacketDef)]
pub struct TimeUpdatePacket {
    pub world_age: i64,
    pub time: i64,
}
