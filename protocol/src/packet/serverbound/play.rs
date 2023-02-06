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

