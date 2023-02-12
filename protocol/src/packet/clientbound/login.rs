use protocol_macro::PacketDef;

use crate::{chat::ChatComponent, varint::VarInt};

#[derive(PacketDef)]
pub struct DisconnectPacket<'a> {
    pub reason: ChatComponent<'a>,
}

#[derive(PacketDef)]
pub struct EncryptionRequestPacket {
    pub server_id: String,

    /// Length of shared secret.
    pub shared_secret_length: VarInt,

    // /// Shared secret.
    // pub shared_secret: Vec<i8>,
    /// Length of verify token.
    pub verify_token_length: VarInt,
    // /// Verify token.
    // pub verify_token: Vec<i8>,
}

/// This packet switches the connection state to play.
#[derive(PacketDef)]
pub struct LoginSuccessPacket {
    /// Unlike in other packets, this field contains the UUID as a string with hyphens.
    pub uuid: String,

    /// Username of Player.
    pub username: String,
}

#[derive(PacketDef)]
pub struct SetCompressionPacket {
    /// Maximum size of a packet before its compressed.
    pub threshold: VarInt,
}
