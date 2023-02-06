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

#[derive(PacketDef)]
pub struct ChatMessage {
    // pub data: Chat,
    pub position: i8
}

#[derive(PacketDef)]
pub struct TimeUpdatePacket {
    pub world_age: i64,
    pub time: i64,
}

#[derive(PacketDef)]
pub struct EntityEquipmentPacket {
    pub entity_id: VarInt,
    pub slot: i16,
    // pub item: Slot,
}

#[derive(PacketDef)]
pub struct SpawnPositionPacket {
    // pub location: Position,
}

#[derive(PacketDef)]
pub struct UpdateHealthPacket {
    pub health: i16,
    pub food: VarInt,
    pub saturation: i16,
}

#[derive(PacketDef)]
pub struct RespawnPacket {
    pub dimension: i32,
    pub difficulty: u8,
    pub gamemode: u8,
    pub level_type: String,
}

#[derive(PacketDef)]
pub struct PlayerPositionAndLookPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    // pub flags: BitField
}

#[derive(PacketDef)]
pub struct HeldItemChangePacket {
    pub slot: i8
}

#[derive(PacketDef)]
pub struct UseBedPacket {
    pub entity_id: VarInt,
    pub location: Position
}

#[derive(PacketDef)]
pub struct AnimationPacket {
    pub entity_id: VarInt,
    pub animation: u8
}
