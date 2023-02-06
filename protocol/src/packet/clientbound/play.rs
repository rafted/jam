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
    pub health: f32,
    pub food: VarInt,
    pub saturation: f32,
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

#[derive(PacketDef)]
pub struct EntityLookPacket {
    pub entity_id: VarInt,
    // pub yaw: Angle,
    // pub pitch: Angle,
    pub on_ground: bool,
}

#[derive(PacketDef)]
pub struct EntityLookAndRelativeMovePacket {
    pub entity_id: VarInt,
    pub delta_x: i8,
    pub delta_y: i8,
    pub delta_z: i8,
    // pub yaw: Angle,
    // pub pitch: Angle,
    pub on_ground: bool,
}

#[derive(PacketDef)]
pub struct EntityTeleportPacket {
    pub entity_id: VarInt,
    pub x: i8,
    pub y: i8,
    pub z: i8,
    // pub yaw: Angle,
    // pub pitch: Angle,
    pub on_ground: bool,
}

#[derive(PacketDef)]
pub struct EntityHeadLookPacket {
    pub entity_id: VarInt,
    pub yaw: Angle,
}

#[derive(PacketDef)]
pub struct EntityStatusPacket {
    pub entity_id: i32,
    pub status: i8,
}

#[derive(PacketDef)]
pub struct AttachEntityPacket {
    pub entity_id: i32,
    pub vehicle_id: i32,
    pub leash: bool,
}

#[derive(PacketDef)]
pub struct EntityMetadataPacket {
    pub entity_id: VarInt,
    // pub metadata: Metadata
}

#[derive(PacketDef)]
pub struct EntityEffectPacket {
    pub entity_id: VarInt,
    pub effect_id: i8,
    pub amplifier: i8,
    pub duration: VarInt,
    pub hide_particles: bool,
}

#[derive(PacketDef)]
pub struct RemoveEntityEffectPacket {
    pub entity_id: VarInt,
    pub effect_id: i8,
}

#[derive(PacketDef)]
pub struct SetExperiencePacket {
    pub bar: f32,
    pub level: VarInt,
    pub total_exp: VarInt,
}

#[derive(PacketDef)]
pub struct EntityPropertiesPacket {
    pub entity_id: VarInt,
    pub properties_amount: i32,

    // TODO: ??
}

#[derive(PacketDef)]
pub struct ChunkDataPacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub ground_up_continuous: bool,
    pub primary_bit_mask: u16,
    pub size: VarInt,
    // pub data: Chunk
}

#[derive(PacketDef)]
pub struct MultiBlockChangePacket {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub record_count: VarInt,

    // TODO: ??
}

#[derive(PacketDef)]
pub struct BlockChangePacket {
    // pub location: Position,
    pub block_id: VarInt
}

#[derive(PacketDef)]
pub struct BlockActionPacket {
    // pub location: Position,
    pub byte_1: u8,
    pub byte_2: u8,
    pub block_type: VarInt
}

#[derive(PacketDef)]
pub struct BlockBreakAnimationPacket {
    pub entity_id: VarInt,
    // pub location: Position,
    pub destroy_stage: i8
}

#[derive(PacketDef)]
pub struct MapChunkBulkPacket {
    pub sky_light_sent: boolean,
    pub column_count: VarInt,

    // TODO: ??

    // pub chunk_data: Vec<Chunk>
}

#[derive(PacketDef)]
pub struct ExplosionPacket {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub radius: f32,
    pub record_count: i32,
    pub records: Vec<(i8, i8, i8)>,
    pub player_motion_x: f32,
    pub player_motion_y: f32,
    pub player_motion_z: f32,
}

#[derive(PacketDef)]
pub struct EffectPacket {
    pub effect_id: i32,
    pub location: Position,
    pub data: i32,
    pub disable_relative_volume: bool,
}

#[derive(PacketDef)]
pub struct SoundEffectPacket {
    pub sound_name: String,
    pub effect_position_x: i32,
    pub effect_position_y: i32,
    pub effect_position_z: i32,
    pub volume: f32,
    pub pitch: u8
}

#[derive(PacketDef)]
pub struct ParticlePacket {
    pub particle_id: i32,
    pub long_distance: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_z: f32,
    pub particle_data: f32,
    pub particle_count: i32,
    pub data: Vec<VarInt>,
}

#[derive(PacketDef)]
pub struct ChangeGameStatePacket {
    pub reason: u8,
    pub value: f32,
}

#[derive(PacketDef)]
pub struct SpawnGlobalEntityPacket {
    pub entity_id: VarInt,
    pub type_: i8,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(PacketDef)]
pub struct OpenWindowPacket {
    pub id: u8,
    pub type_: String,
    // pub title: Chat,
    pub slots: u8,
    pub entity_id: Optional<i32>,
}

#[derive(PacketDef)]
pub struct CloseWindowPacket {
    pub id: u8,
}

#[derive(PacketDef)]
pub struct SetSlotPacket {
    pub id: i8,
    pub slot: i16,
    // pub data: Slot
}

#[derive(PacketDef)]
pub struct WindowItemsPacket {
    pub id: u8,
    pub count: i16,
    // pub data: Vec<Slot>
}

#[derive(PacketDef)]
pub struct WindowPropertyPacket {
    pub id: u8,
    pub property: i16,
    pub value: i16,
}


#[derive(PacketDef)]
pub struct ConfirmTransactionPacket {
    pub id: i8,
    pub action_number: i16,
    pub accepted: bool,
}

#[derive(PacketDef)]
pub struct UpdateSignPacket {
    // pub location: Position,
    // pub line1: Chat,
    // pub line2: Chat,
    // pub line3: Chat,
    // pub line4: Chat,
}
#[derive(PacketDef)]
pub struct MapPacket {
    pub item_damage: VarInt,
    pub scale: i8,
    pub icon_amount: VarInt,

    // TODO: ??

    pub column: i8,
    pub rows: Optional<i8>,
    pub x: Optional<i8>,
    pub z: Optional<i8>,
    pub length: Optional<VarInt>,
    pub data: Optional<Vec<u8>>
}

#[derive(PacketDef)]
pub struct UpdateBlockEntityPacket {
    // pub location: Position,
    pub action: u8,
    // pub data: NBTTag
}
