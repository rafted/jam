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

#[derive(PacketDef)]
pub struct SpawnPlayerPacket {
    pub entity_id: VarInt,
    // pub uuid: UUID,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    // pub yaw: Angle,
    // pub pitch: Angle,
    pub current_item: i16,
    // pub metadata: Metadata
}

#[derive(PacketDef)]
pub struct CollectItemPacket {
    pub collected_entity_id: VarInt,
    pub collector_entity_id: VarInt,
}

#[derive(PacketDef)]
pub struct SpawnObjectPacket {
    pub entity_id: VarInt,
    pub type_: i8,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    // pub pitch: Angle,
    // pub yaw: Angle,
    pub data: i32,
    pub velocity_x: Optional<i16>,
    pub velocity_y: Optional<i16>,
    pub velocity_z: Optional<i16>,
}

#[derive(PacketDef)]
pub struct SpawnMobPacket {
    pub entity_id: VarInt,
    pub type_: u8,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    // pub yaw: Angle,
    // pub pitch: Angle,
    // pub head_pitch: Angle,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
    // pub metadata: Metadata,
}

#[derive(PacketDef)]
pub struct SpawnPaintingPacket {
    pub entity_id: VarInt,
    pub title: String,
    // pub location: Position,
    pub direction: u8,
}

#[derive(PacketDef)]
pub struct SpawnExperienceOrbPacket {
    pub entity_id: VarInt,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub count: i16
}

#[derive(PacketDef)]
pub struct EntityVelocityPacket {
    pub entity_id: VarInt,
    pub velocity_x: i16,
    pub velocity_y: i16,
    pub velocity_z: i16,
}

#[derive(PacketDef)]
pub struct DestroyEntitiesPacket {
    pub count: VarInt,
    pub entities_id: Vec<VarInt>,
}

#[derive(PacketDef)]
pub struct EntityPacket {
    pub entity_id: VarInt,
}

#[derive(PacketDef)]
pub struct EntityRelativeMovePacket {
    pub entity_id: VarInt,
    pub delta_x: i8,
    pub delta_y: i8,
    pub delta_z: i8,
    pub on_ground: bool,
}