use protocol_macro::PacketDef;

use crate::varint::VarInt;

#[derive(PacketDef)]
pub struct LoginStartPacket {
    pub name: String,
}

#[derive(PacketDef)]
pub struct EncryptionResponsePacket {
    /// Length of shared secret.
    pub shared_secret_length: VarInt,

    // /// Shared secret.
    // pub shared_secret: Vec<i8>,

    /// Length of verify token.
    pub verify_token_length: VarInt,

    // /// Verify token.
    // pub verify_token: Vec<i8>,
}


