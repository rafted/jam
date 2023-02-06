use protocol_macro::PacketDef;

use crate::varint::VarInt;

#[derive(PacketDef)]
pub struct KeepAlivePacket {
    pub id: VarInt,
}
