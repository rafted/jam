use crate::position::Position;
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
    // pub target_x: Option<f32>,
    // pub target_y: Option<f32>,
    // pub target_z: Option<f32>,
}

#[derive(PacketDef)]
pub struct PlayerPacket {
    // pub on_ground: bool,
}

#[derive(PacketDef)]
pub struct PlayerPositionPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    // pub on_ground: bool,
}

#[derive(PacketDef)]
pub struct PlayerPositionLook {
    pub yaw: f32,
    pub pitch: f32,
    // pub on_ground: bool,
}

#[derive(PacketDef)]
pub struct PlayerPositionAndLookPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    // pub on_ground: bool,
}

#[derive(PacketDef)]
pub struct PlayerDiggingPacket {
    pub status: i8,
    pub location: Position,
    pub face: i8,
}

#[derive(PacketDef)]
pub struct PlayerBlockPlacementPacket {
    pub location: Position,
    pub face: i8,
    // pub held_item: Slot,
    pub cursor_position_x: i8,
    pub cursor_position_y: i8,
    pub cursor_position_z: i8,
}

#[derive(PacketDef)]
pub struct HeldItemChangePacket {
    pub slot: i16,
}

#[derive(PacketDef)]
pub struct AnimationPacket {}

#[derive(PacketDef)]
pub struct EntityActionPacket {
    pub entity_id: VarInt,
    pub action_id: VarInt,
    pub action_parameter: VarInt,
}

#[derive(PacketDef)]
pub struct SteerVehiclePacket {
    pub sideways: f32,
    pub forward: f32,
    pub flags: u8,
}

#[derive(PacketDef)]
pub struct CloseWindowPacket {
    pub id: u8,
}

#[derive(PacketDef)]
pub struct ClickWindowPacket {
    pub id: u8,
    pub slot: i16,
    pub button: i8,
    pub action_number: i16,
    // TODO: ??

    // pub clicked_item: Slot
}

#[derive(PacketDef)]
pub struct ConfirmTransactionPacket {
    pub window_id: i8,
    pub action_number: i16,
    // pub accepted: bool,
}

#[derive(PacketDef)]
pub struct CreativeInventoryActionPacket {
    pub slot: i16,
    // pub clicked_item: Slot,
}

#[derive(PacketDef)]
pub struct EnchantItemPacket {
    pub window_id: i8,
    pub enchantment: i8,
}

#[derive(PacketDef)]
pub struct UpdateSignPacket {
    pub location: Position,
    pub line_1: String,
    pub line_2: String,
    pub line_3: String,
    pub line_4: String,
}

#[derive(PacketDef)]
pub struct PlayerAbilitiesPacket {
    pub flags: i8,
    pub flying_speed: f32,
    pub walking_speed: f32,
}

#[derive(PacketDef)]
pub struct TabCompletePacket {
    pub text: String,
    // pub has_position: bool,
    // pub looked_at_block: Option<Position>,
}

#[derive(PacketDef)]
pub struct ClientSettingsPacket {
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: i8,
    // pub chat_colors: bool,
    pub display_skin_parts: u8,
}

#[derive(PacketDef)]
pub struct ClientStatusPacket {
    pub action_id: VarInt,
}

#[derive(PacketDef)]
pub struct PluginMessagePacket {
    pub channel: String,
    // TODO: According to #mcdevs, the length of Data is known only from the packet length,
    // since the packet has no length field of any kind.
    // pub data: Vec<i8>,
}

#[derive(PacketDef)]
pub struct SpectatePacket {
    // pub target: UUID,
}

#[derive(PacketDef)]
pub struct ResourcePackStatusPacket {
    pub hash: String,
    pub result: VarInt,
}
