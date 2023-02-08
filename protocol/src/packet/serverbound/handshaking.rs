use protocol_macro::PacketDef;

use crate::{encoding::Encodable, varint::VarInt};

#[derive(PacketDef)]
pub struct HandshakePacket {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}
