use protocol_macro::PacketDef;

use crate::{
    position::{Angle, Position},
    varint::VarInt,
};
// use crate::chat::ChatComponent;

/// The server will frequently send out a keep-alive, each containing a random ID. The client must
/// respond with the same packet. If the client does not respond to them for over 30 seconds, the
/// server kicks the client. Vice versa, if the server does not send any keep-alives for 20
/// seconds, the client will disconnect and yields a "Timed out" exception.
#[derive(PacketDef)]
pub struct KeepAlivePacket {
    /// The Random ID.
    pub id: VarInt,
}

/// Sends the state of the world the player is joining on on, to the player.
#[derive(PacketDef)]
pub struct JoinGamePacket {
    /// The player's Entity ID (EID)
    pub entity_id: i32,

    /// 0: Survival,
    /// 1: Creative,
    /// 2: Adventure,
    /// 3: Spectator.
    ///
    /// Bit 3 (0x8) is the hardcore flag.
    pub gamemode: u16,

    /// -1: Nether,
    /// 0: Overworld,
    /// 1: End.
    pub dimension: i8,

    /// 0: peaceful,
    /// 1: easy,
    /// 2: normal,
    /// 3: hard.
    pub difficulty: u8,

    /// Used by the client to draw the player list
    pub max_players: u8,

    /// default, flat, largeBiomes, amplified, default_1_1
    pub level_type: String,

    /// If true, a Notchian client shows reduced information on the debug screen.
    // pub reduced_debug_info: bool,
}

// /// Identifying the difference between Chat/System Message is important as it helps respect the
// /// user's chat visibility options. While Position 2 accepts json formatting it will not display,
// /// old style formatting works.
// #[derive(PacketDef)]
// pub struct ChatMessagePacket<'a> {
//     /// Limited to 32767 bytes
//     pub data: ChatComponent<'a>,
//
//     /// 0: chat (chat box),
//     /// 1: system message (chat box),
//     /// 2: above hotbar.
//     pub position: i8,
// }

/// Time is based on ticks, where 20 ticks happen every second. There are 24000 ticks in a day,
/// making Minecraft days exactly 20 minutes long.
///
/// The time of day is based on the timestamp modulo 24000. 0 is sunrise, 6000 is noon, 12000 is
/// sunset, and 18000 is midnight.
///
/// The default SMP server increments the time by 20 every second.
#[derive(PacketDef)]
pub struct TimeUpdatePacket {
    /// In ticks; not changed by server commands.
    pub world_age: i64,

    /// The world (or region) time, in ticks. If negative the sun will stop moving at the Math.abs
    /// of the time.
    pub time: i64,
}

/// Updates what an entity is holding in their hand.
#[derive(PacketDef)]
pub struct EntityEquipmentPacket {
    /// Entity's EID.
    pub entity_id: VarInt,

    /// Equipment slot.
    /// 0: held,
    /// 1–4: armor slot (1: boots,
    ///                  2: leggings,
    ///                  3: chestplate,
    ///                  4: helmet.)
    pub slot: i16,

    /// Item in slot format.
    // pub item: Slot,
}

/// Sent by the server after login to specify the coordinates of the spawn point (the point at
/// which players spawn at, and which the compass points to). It can be sent at any time to update
/// the point compasses point at.
#[derive(PacketDef)]
pub struct SpawnPositionPacket {
    /// Spawn location.
    pub location: Position,
}

/// Sent by the server to update/set the health of the player it is sent to. Food saturation acts
/// as a food “overcharge”. Food values will not decrease while the saturation is over zero.
/// Players logging in automatically get a saturation of 5.0. Eating food increases the saturation
/// as well as the food bar.
#[derive(PacketDef)]
pub struct UpdateHealthPacket {
    /// 0 or less = dead,
    /// 20 = full HP.
    pub health: f32,

    /// 0–20
    pub food: VarInt,

    /// Seems to vary from 0.0 to 5.0 in integer increments.
    pub saturation: f32,
}

/// To change the player's dimension (overworld/nether/end), send them a respawn packet with the
/// appropriate dimension, followed by prechunks/chunks for the new dimension, and finally a
/// position and look packet. You do not need to unload chunks, the client will do it
/// automatically.
#[derive(PacketDef)]
pub struct RespawnPacket {
    /// -1: The Nether,
    /// 0: The Overworld,
    /// 1: The End.
    pub dimension: i32,

    /// 0: Peaceful,
    /// 1: Easy,
    /// 2: Normal,
    /// 3: Hard.
    pub difficulty: u8,

    /// 0: survival,
    /// 1: creative,
    /// 2: adventure.
    /// The hardcore flag is not included.
    pub gamemode: u8,

    /// Same as Join Game
    pub level_type: String,
}

/// Updates the player's position on the server. This packet will also close the "Downloading
/// Terrain" screen when joining/respawning.
///
/// If the distance between the last known position of the player on the server and the new
/// position set by this packet is greater than 100 meters, the client will be kicked for “You
/// moved too quickly :( (Hacking?)”.
///
/// Also if the fixed-point number of X or Z is set greater than 3.2E7D the client will be kicked
/// for “Illegal position”.
///
/// Yaw is measured in degrees, and does not follow classical trigonometry rules. The unit circle
/// of yaw on the XZ-plane starts at (0, 1) and turns counterclockwise, with 90 at (-1, 0), 180 at
/// (0, -1) and 270 at (1, 0). Additionally, yaw is not clamped to between 0 and 360 degrees; any
/// number is valid, including negative numbers and numbers greater than 360.
///
/// Pitch is measured in degrees, where 0 is looking straight ahead, -90 is looking straight up,
/// and 90 is looking straight down.
///
/// The yaw and pitch of player (in degrees), standing at point (x0, y0, z0) and looking towards
/// point (x, y, z) one can be calculated with:
/// ```
/// dx = x-x0
/// dy = y-y0
/// dz = z-z0
/// r = sqrt( dx*dx + dy*dy + dz*dz )
/// yaw = -atan2(dx,dz)/PI*180
/// if yaw < 0 then
///     yaw = 360 - yaw
/// pitch = -arcsin(dy/r)/PI*180
/// ```
///
/// You can get a unit vector from a given yaw/pitch via:
/// ```
/// x = -cos(pitch) * sin(yaw)
/// y = -sin(pitch)
/// z =  cos(pitch) * cos(yaw)
/// ```
#[derive(PacketDef)]
pub struct PlayerPositionAndLookPacket {
    /// Absolute or relative position, depending on Flags.
    pub x: f64,

    /// Absolute or relative position, depending on Flags.
    pub y: f64,

    /// Absolute or relative position, depending on Flags.
    pub z: f64,

    /// Absolute or relative rotation on the X Axis, in degrees.
    pub yaw: f32,

    /// Absolute or relative rotation on the Y Axis, in degrees.
    pub pitch: f32,

    /// Bit field.
    ///
    /// ```
    /// <Dinnerbone> It's a bitfield, X/Y/Z/Y_ROT/X_ROT. If X is set, the x value is relative and not absolute.
    /// ```
    ///
    /// Field 	Bit
    /// X 	    0x01
    /// Y 	    0x02
    /// Z 	    0x04
    /// Y_ROT 	0x08
    /// X_ROT 	0x10
    // pub flags: BitField
}

/// Sent to change the player's slot selection.
#[derive(PacketDef)]
pub struct HeldItemChangePacket {
    /// The slot which the player has selected (0–8.)
    pub slot: i8,
}

/// This packet tells that a player goes to bed. The client with the matching Entity ID will go
/// into bed mode. This Packet is sent to all nearby players including the one sent to bed.
#[derive(PacketDef)]
pub struct UseBedPacket {
    /// Sleeping player's EID.
    pub entity_id: VarInt,

    /// Block location of the head part of the bed.
    pub location: Position,
}

/// Sent whenever an entity should change animation.
#[derive(PacketDef)]
pub struct AnimationPacket {
    /// Player ID.
    pub entity_id: VarInt,

    /// Animation ID.
    ///
    /// Animation can be one of the following values:
    /// ID 	Animation
    /// 0 	Swing arm
    /// 1 	Take damage
    /// 2 	Leave bed
    /// 3 	Eat food
    /// 4 	Critical effect
    /// 5 	Magic critical effect
    pub animation: u8,
}

/// This packet is sent by the server when a player comes into visible range, not when a player
/// joins.
///
/// This packet must be sent after the Player List Item (Play, 0x38, clientbound) packet that adds
/// the player data for the client to use when spawning a player. If the tab list entry for the
/// UUID included in this packet is not present when this packet arrives, the entity will not be
/// spawned. The tab includes skin/cape data.
///
/// Servers can, however, safely spawn player entities for players not in visible range. The client
/// appears to handle it correctly.
///
/// When in online-mode the UUIDs must be valid and have valid skin blobs, in offline-mode UUID v3
/// is used.
///
/// For NPCs UUID v2 should be used. Note:
/// ```
/// <+Grum> i will never confirm this as a feature you know that :)
/// ```
///
/// In an example UUID, `xxxxxxxx-xxxx-Yxxx-xxxx-xxxxxxxxxxxx`, the UUID version is specified by Y.
/// So, for UUID v3, Y will always be 3, and for UUID v2, Y will always be 2.
#[derive(PacketDef)]
pub struct SpawnPlayerPacket {
    /// Player's EID.
    pub entity_id: VarInt,

    /// The UUID of the player.
    // pub uuid: UUID,
    
    /// Player X as a Fixed-Point number.
    pub x: i32,

    /// Player Y as a Fixed-Point number.
    pub y: i32,

    /// Player Z as a Fixed-Point number.
    pub z: i32,

    /// Player rotation on the X Axis.
    // pub yaw: Angle,
    
    /// Player rotation on the Y Axis.
    // pub pitch: Angle,
    
    /// The item the player is currently holding. Note that this should be 0 for “no item”, unlike
    /// -1 used in other packets.
    pub current_item: i16,

    /// The entity metadata.
    // pub metadata: Metadata
}

/// Sent by the server when someone picks up an item lying on the ground — its sole purpose appears
/// to be the animation of the item flying towards you. It doesn't destroy the entity in the client
/// memory, and it doesn't add it to your inventory. The server only checks for items to be picked
/// up after each Player Position (and Player Position And Look) packet sent by the client.
#[derive(PacketDef)]
pub struct CollectItemPacket {
    /// EID of the item being collected.
    pub collected_entity_id: VarInt,

    /// EID of the entity collecting the item.
    pub collector_entity_id: VarInt,
}

/// Sent by the server when a vehicle or other object is created.
#[derive(PacketDef)]
pub struct SpawnObjectPacket {
    /// EID of the object.
    pub entity_id: VarInt,

    /// The type of object.
    pub type_: i8,

    /// X position as a Fixed-Point number.
    pub x: i32,

    /// Y position as a Fixed-Point number.
    pub y: i32,

    /// Z position as a Fixed-Point number.
    pub z: i32,

    /// Player rotation on the Y Axis.
    // pub pitch: Angle,
   
    /// Player rotation on the X Axis.
    // pub yaw: Angle,

    /// Meaning dependent on the value of the Type field, see Object Data for details.
    pub data: i32,

    /// Velocity on the X axis. Only sent if the Data field is nonzero.
    // pub velocity_x: Option<i16>,
    
    /// Velocity on the Y axis. Only sent if the Data field is nonzero.
    // pub velocity_y: Option<i16>,
   
    /// Velocity on the Z axis. Only sent if the Data field is nonzero.
    // pub velocity_z: Option<i16>,
}

/// Sent by the server when a vehicle or other object is created.
#[derive(PacketDef)]
pub struct SpawnMobPacket {
    /// EID of the object.
    pub entity_id: VarInt,

    /// The type of object.
    pub type_: u8,

    /// X position as a Fixed-Point number
    pub x: i32,

    /// Y position as a Fixed-Point number
    pub y: i32,

    /// Z position as a Fixed-Point number
    pub z: i32,

    /// Player rotation on the X Axis.
    // pub yaw: Angle,

    /// Player rotation on the Y Axis.
    // pub pitch: Angle,
    
    /// Head rotation on the Y Axis.
    // pub head_pitch: Angle,

    /// Velocity on the X Axis.
    pub velocity_x: i16,

    /// Velocity on the Y Axis.
    pub velocity_y: i16,

    /// Velocity on the Z Axis.
    pub velocity_z: i16,

    /// Entity metadata.
    // pub metadata: Metadata,
}

/// This packet shows location, name, and type of painting.
/// 
/// Calculating the center of an image: given a (width x height) grid of cells, with (0, 0) being
/// the top left corner, the center is (max(0, width / 2 - 1), height / 2). E.g.
/// ```
/// 2x1 (1, 0)
/// 4x4 (1, 2)
/// ```
#[derive(PacketDef)]
pub struct SpawnPaintingPacket {
    /// EID of the entity.
    pub entity_id: VarInt,

    /// Name of the painting. Max length 13.
    pub title: String,

    /// Center coordinates.
    pub location: Position,

    /// Direction the painting faces.
    /// 0: north (-z),
    /// 1: west (-x),
    /// 2: south (+z),
    /// 3: east (+x).
    pub direction: u8,
}

/// Spawns one or more experience orbs.
#[derive(PacketDef)]
pub struct SpawnExperienceOrbPacket {
    /// EID of the entity.
    pub entity_id: VarInt,

    /// Player X as a Fixed-Point number.
    pub x: i32,

    /// Player Y as a Fixed-Point number.
    pub y: i32,

    /// Player Z as a Fixed-Point number.
    pub z: i32,

    /// The amount of experience this orb will reward once collected.
    pub count: i16,
}

/// Velocity is believed to be in units of 1/8000 of a block per server tick (50ms); for example,
/// -1343 would move (-1343 / 8000) = −0.167875 blocks per tick (or −3,3575 blocks per second).
#[derive(PacketDef)]
pub struct EntityVelocityPacket {
    /// EID of the entity.
    pub entity_id: VarInt,
    
    /// Velocity on the X axis.
    pub velocity_x: i16,

    /// Velocity on the Y axis.
    pub velocity_y: i16,

    /// Velocity on the Z axis.
    pub velocity_z: i16,
}

#[derive(PacketDef)]
pub struct DestroyEntitiesPacket {
    pub count: VarInt,
    // pub entities_id: Vec<VarInt>,
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
    // pub on_ground: bool,
}

#[derive(PacketDef)]
pub struct EntityLookPacket {
    pub entity_id: VarInt,
    // pub yaw: Angle,
    // pub pitch: Angle,
    // pub on_ground: bool,
}

#[derive(PacketDef)]
pub struct EntityLookAndRelativeMovePacket {
    pub entity_id: VarInt,
    pub delta_x: i8,
    pub delta_y: i8,
    pub delta_z: i8,
    // pub yaw: Angle,
    // pub pitch: Angle,
    // pub on_ground: bool,
}

#[derive(PacketDef)]
pub struct EntityTeleportPacket {
    pub entity_id: VarInt,
    pub x: i8,
    pub y: i8,
    pub z: i8,
    pub yaw: Angle,
    pub pitch: Angle,
    // pub on_ground: bool,
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
    // pub leash: bool,
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
    // pub hide_particles: bool,
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
    // pub ground_up_continuous: bool,
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
    pub location: Position,
    pub block_id: VarInt,
}

#[derive(PacketDef)]
pub struct BlockActionPacket {
    pub location: Position,
    pub byte_1: u8,
    pub byte_2: u8,
    pub block_type: VarInt,
}

#[derive(PacketDef)]
pub struct BlockBreakAnimationPacket {
    pub entity_id: VarInt,
    pub location: Position,
    pub destroy_stage: i8,
}

#[derive(PacketDef)]
pub struct MapChunkBulkPacket {
    // pub sky_light_sent: bool,
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
    // pub records: Vec<(i8, i8, i8)>,
    pub player_motion_x: f32,
    pub player_motion_y: f32,
    pub player_motion_z: f32,
}

#[derive(PacketDef)]
pub struct EffectPacket {
    pub effect_id: i32,
    pub location: Position,
    pub data: i32,
    // pub disable_relative_volume: bool,
}

#[derive(PacketDef)]
pub struct SoundEffectPacket {
    pub sound_name: String,
    pub effect_position_x: i32,
    pub effect_position_y: i32,
    pub effect_position_z: i32,
    pub volume: f32,
    pub pitch: u8,
}

#[derive(PacketDef)]
pub struct ParticlePacket {
    pub particle_id: i32,
    // pub long_distance: bool,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub offset_z: f32,
    pub particle_data: f32,
    pub particle_count: i32,
    // pub data: Vec<VarInt>,
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

// #[derive(PacketDef)]
// pub struct OpenWindowPacket<'a> {
//     pub id: u8,
//     pub type_: String,
//     pub title: ChatComponent<'a>,
//     pub slots: u8,
//     pub entity_id: Option<i32>,
// }

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
    // pub accepted: bool,
}

// #[derive(PacketDef)]
// pub struct UpdateSignPacket<'a> {
//     pub location: Position,
//     pub line1: ChatComponent<'a>,
//     pub line2: ChatComponent<'a>,
//     pub line3: ChatComponent<'a>,
//     pub line4: ChatComponent<'a>,
// }

#[derive(PacketDef)]
pub struct MapPacket {
    pub item_damage: VarInt,
    pub scale: i8,
    pub icon_amount: VarInt,

    // TODO: ??
    pub column: i8,
    // pub rows: Option<i8>,
    // pub x: Option<i8>,
    // pub z: Option<i8>,
    // pub length: Option<VarInt>,
    // pub data: Option<Vec<u8>>,
}

#[derive(PacketDef)]
pub struct UpdateBlockEntityPacket {
    pub location: Position,
    pub action: u8,
    // pub data: NBTTag
}

#[derive(PacketDef)]
pub struct OpenSignEditorPacket {
    pub location: Position,
}

#[derive(PacketDef)]
pub struct StatisticsPacket {
    pub count: VarInt,
    // TODO: ??
}

#[derive(PacketDef)]
pub struct PlayerListItemPacket {
    pub action: VarInt,
    pub players_amount: VarInt,
    // TODO: ??
}

#[derive(PacketDef)]
pub struct PlayerAbilitiesPacket {
    pub flags: i8,
    pub flying_speed: f32,
    pub fov_modifier: f32,
}

#[derive(PacketDef)]
pub struct TabCompletePacket {
    pub flags: VarInt,
    // pub matches: Vec<String>,
}

#[derive(PacketDef)]
pub struct ScoreboardObjectivePacket {
    pub name: String,
    pub mode: i8,
    // pub value: Option<String>,
    // pub type_: Option<String>,
}

#[derive(PacketDef)]
pub struct UpdateScorePacket {
    pub name: String,
    pub action: i8,
    pub objective_name: String,
    // pub value: Option<VarInt>,
}

#[derive(PacketDef)]
pub struct DisplayScoreboardPacket {
    pub position: i8,
    pub name: String,
}

#[derive(PacketDef)]
pub struct TeamPacket {
    pub name: String,
    pub mode: i8,
    // pub display_name: Option<String>,
    // pub prefix: Option<String>,
    // pub suffix: Option<String>,
    // pub friendly_fire: Option<i8>,
    // pub name_tag_visibility: Option<String>,
    // pub color: Option<i8>,
    // pub players_amount: Option<VarInt>,
    // pub players: Option<String>,
}

#[derive(PacketDef)]
pub struct PluginMessagePacket {
    pub channel: String,
    // pub data: Vec<i8>,
}

// #[derive(PacketDef)]
// pub struct DisconnectPacket<'a> {
//     pub reason: ChatComponent<'a>,
// }

#[derive(PacketDef)]
pub struct ServerDifficultyPacket {
    pub difficulty: u8,
}

#[derive(PacketDef)]
pub struct CombatEventPacket {
    pub event: VarInt,
    // pub duration: Option<VarInt>,
    // pub player_id: Option<VarInt>,
    // pub entity_id: Option<i32>,
    pub message: String,
}

#[derive(PacketDef)]
pub struct CameraPacket {
    pub id: VarInt,
}

#[derive(PacketDef)]
pub struct WorldBorderPacket {
    pub action: VarInt,
    // TODO: ??
}

#[derive(PacketDef)]
pub struct SetCompressionPacket {
    pub threshold: VarInt,
}

// #[derive(PacketDef)]
// pub struct PlayerListHeaderAndFooterPacket<'a> {
//     pub header: ChatComponent<'a>,
//     pub footer: ChatComponent<'a>,
// }

#[derive(PacketDef)]
pub struct ResourcePackSendPacket {
    pub url: String,
    pub hash: String,
}

#[derive(PacketDef)]
pub struct UpdateEntityNBTPacket {
    pub id: VarInt,
    // pub tag: NBTTag,
}
