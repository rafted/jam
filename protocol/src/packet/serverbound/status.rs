use protocol_macro::PacketDef;

#[derive(PacketDef)]
pub struct RequestPacket {}

#[derive(PacketDef)]
pub struct PingPacket {
    /// Long that the server is supposed to respond with through a Pong packet.
    pub payload: i64,
}
