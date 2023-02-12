use protocol_macro::PacketDef;

/// Status response packet.
///
/// It consists of a JSON string. Instead of a schema, here's an example:
/// ```json
/// {
///     "version": {
///         "name": "1.19",
///         "protocol": 759
///     },
///     "players": {
///         "max": 100,
///         "online": 5,
///         "sample": [
///             {
///                 "name": "thinkofdeath",
///                 "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
///             }
///         ]
///     },
///     "description": {
///         "text": "Hello world"
///     },
///     "favicon": "data:image/png;base64,<data>",
///     "previewsChat": true,
///     "enforcesSecureChat": true,
/// }
/// ```
///
/// An id of a sample needs to be a valid UUID format, else the connection will abort.
///
/// The description field is a Chat object. Note that the Notchian server has no way of providing
/// actual chat component data; instead section sign-based codes are embedded within the text of
/// the object. However, third-party servers such as Spigot and Paper will return full components,
/// so make sure you can handle both.
///
/// The favicon field is optional. The sample field may be missing if the server has no online
/// players.
///
/// The favicon should be a PNG image that is Base64 encoded (without newlines: \n, new lines no
/// longer work since 1.13) and prepended with data:image/png;base64,. It should also be noted that
/// the source image must be exactly 64x64 pixels, otherwise the Notchian client will not render
/// the image.
///
/// When the previewsChat field is set to true, the client will display a warning upon joining and
/// send Chat Preview (serverbound) packets while the player types chat messages. This field is
/// optional, although the Notchian server will always include it.
///
/// After receiving the Response packet, the client may send the next packet to help calculate the
/// server's latency, or if it is only interested in the above information it can disconnect here.
///
/// If the client does not receive a properly formatted response, then it will instead attempt a
/// legacy ping.
#[derive(PacketDef)]
pub struct ResponsePacket {
    /// JSON data.
    pub response: String,
}

#[derive(PacketDef)]
pub struct PongPacket {
    /// Long that was sent by the client.
    pub payload: i64,
}
