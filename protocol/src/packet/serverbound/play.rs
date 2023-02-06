use protocol_macro::PacketDef;

use crate::varint::VarInt;

#[derive(PacketDef)]
pub struct KeepAlivePacket {
    pub id: VarInt,
}

#[derive(PacketDef)]
pub struct ChatMessagePacket {
    pub message: String,
}

#[derive(PacketDef)]
pub struct UseEntityPacket {
    pub target: VarInt,
    pub type_: VarInt,
    pub target_x: Option<f32>,
    pub target_y: Option<f32>,
    pub target_z: Option<f32>,
}


#[derive(PacketDef)]
pub struct PlayerPacket {
    pub on_ground: bool,
}

#[derive(PacketDef)]
pub struct PlayerPositionPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub on_ground: bool,
}

#[derive(PacketDef)]
pub struct PlayerPositionLook {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

#[derive(PacketDef)]
pub struct PlayerPositionAndLookPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

#[derive(PacketDef)]
pub struct PlayerDiggingPacket {
    pub status: i8,
    // pub location: Position,
    pub face: i8
}

#[derive(PacketDef)]
pub struct PlayerBlockPlacementPacket {
    // pub location: Position,
    pub face: i8,
    // pub held_item: Slot,
    pub cursor_position_x: i8,
    pub cursor_position_y: i8,
    pub cursor_position_z: i8,
}

#[derive(PacketDef)]
pub struct HeldItemChangePacket {
    pub slot: i16
}
