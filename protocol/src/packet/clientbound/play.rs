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
///
/// # Gamemode Table:
/// | ID | Gamemode  |
/// |----|-----------|
/// | 0  | Survival  |
/// | 1  | Creative  |
/// | 2  | Adventure |
/// | 3  | Spectator |
#[derive(PacketDef)]
pub struct JoinGamePacket {
    /// The player's Entity ID (EID)
    pub entity_id: i32,

    /// ID of the gamemode.
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
    // /// If true, a Notchian client shows reduced information on the debug screen.
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
    // /// Item in slot format.
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
    // /// Bit field.
    // ///
    // /// ```
    // /// <Dinnerbone> It's a bitfield, X/Y/Z/Y_ROT/X_ROT. If X is set, the x value is relative and not absolute.
    // /// ```
    // ///
    // /// Field      Bit
    // /// X          0x01
    // /// Y          0x02
    // /// Z          0x04
    // /// Y_ROT      0x08
    // /// X_ROT      0x10
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
    /// ID     Animation
    /// 0      Swing arm
    /// 1      Take damage
    /// 2      Leave bed
    /// 3      Eat food
    /// 4      Critical effect
    /// 5      Magic critical effect
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
/// ```text
/// <+Grum> i will never confirm this as a feature you know that :)
/// ```
///
/// In an example UUID, `xxxxxxxx-xxxx-Yxxx-xxxx-xxxxxxxxxxxx`, the UUID version is specified by Y.
/// So, for UUID v3, Y will always be 3, and for UUID v2, Y will always be 2.
#[derive(PacketDef)]
pub struct SpawnPlayerPacket {
    /// Player's EID.
    pub entity_id: VarInt,

    // /// The UUID of the player.
    // pub uuid: UUID,
    /// Player X as a Fixed-Point number.
    pub x: i32,

    /// Player Y as a Fixed-Point number.
    pub y: i32,

    /// Player Z as a Fixed-Point number.
    pub z: i32,

    // /// Player rotation on the X Axis.
    // pub yaw: Angle,

    // /// Player rotation on the Y Axis.
    // pub pitch: Angle,
    /// The item the player is currently holding. Note that this should be 0 for “no item”, unlike
    /// -1 used in other packets.
    pub current_item: i16,
    // /// The entity metadata.
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

    // /// Player rotation on the Y Axis.
    // pub pitch: Angle,

    // /// Player rotation on the X Axis.
    // pub yaw: Angle,
    /// Meaning dependent on the value of the Type field, see Object Data for details.
    pub data: i32,
    // /// Velocity on the X axis. Only sent if the Data field is nonzero.
    // pub velocity_x: Option<i16>,

    // /// Velocity on the Y axis. Only sent if the Data field is nonzero.
    // pub velocity_y: Option<i16>,

    // /// Velocity on the Z axis. Only sent if the Data field is nonzero.
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

    // /// Player rotation on the X Axis.
    // pub yaw: Angle,

    // /// Player rotation on the Y Axis.
    // pub pitch: Angle,

    // /// Head rotation on the Y Axis.
    // pub head_pitch: Angle,
    /// Velocity on the X Axis.
    pub velocity_x: i16,

    /// Velocity on the Y Axis.
    pub velocity_y: i16,

    /// Velocity on the Z Axis.
    pub velocity_z: i16,
    // /// Entity metadata.
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

    /// ID of the Direction the painting faces.
    ///
    /// | ID | Direction  |
    /// |----|------------|
    /// | 0  | North (-z) |
    /// | 1  | West (-x)  |
    /// | 2  | South (+z) |
    /// | 3  | East (+x)  |
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

/// Sent by the server when a list of entities is to be destroyed on the client.
#[derive(PacketDef)]
pub struct DestroyEntitiesPacket {
    /// Number of elements in the following array.
    pub count: VarInt,
    // /// The list of entities of destroy.
    // pub entities_id: Vec<VarInt>,
}

/// This packet may be used to initialize an entity.
///
/// For player entities, either this packet or any move/look packet is sent every game tick. So the
/// meaning of this packet is basically that the entity did not move/look since the last such
/// packet.
#[derive(PacketDef)]
pub struct EntityPacket {
    /// EID of the Entity.
    pub entity_id: VarInt,
}

/// This packet is sent by the server when an entity moves less then 4 blocks; if an entity moves
/// more than 4 blocks Entity Teleport should be sent instead.
///
/// This packet allows at most four blocks movement in any direction, because byte range is from
/// -128 to 127.
#[derive(PacketDef)]
pub struct EntityRelativeMovePacket {
    /// EID of the Entity.
    pub entity_id: VarInt,

    /// Change in X position as a Fixed-Point number
    pub delta_x: i8,

    /// Change in Y position as a Fixed-Point number
    pub delta_y: i8,

    /// Change in Z position as a Fixed-Point number
    pub delta_z: i8,
    // /// Whether the player is touching the ground or not.
    // pub on_ground: bool,
}

/// This packet is sent by the server when an entity rotates.
#[derive(PacketDef)]
pub struct EntityLookPacket {
    /// The EID of the Entity.
    pub entity_id: VarInt,
    // /// New angle, not a delta.
    // pub yaw: Angle,

    // /// New angle, not a delta.
    // pub pitch: Angle,

    // /// Whether the player is touching the ground or not.
    // pub on_ground: bool,
}

/// This packet is sent by the server when an entity rotates and moves. Since a byte range is
/// limited from -128 to 127, and movement is offset of fixed-point numbers, this packet allows at
/// most four blocks movement in any direction. (-128/32 == -4)
#[derive(PacketDef)]
pub struct EntityLookAndRelativeMovePacket {
    /// The EID of the Entity.
    pub entity_id: VarInt,

    /// Change in X position as a Fixed-Point number.
    pub delta_x: i8,

    /// Change in Y position as a Fixed-Point number.
    pub delta_y: i8,

    /// Change in Z position as a Fixed-Point number.
    pub delta_z: i8,
    // /// New angle, not a delta.
    // pub yaw: Angle,

    // /// New angle, not a delta.
    // pub pitch: Angle,

    // /// Whether the player is touching the ground or not.
    // pub on_ground: bool,
}

/// This packet is sent by the server when an entity moves more than 4 blocks.
#[derive(PacketDef)]
pub struct EntityTeleportPacket {
    /// The EID of the Entity.
    pub entity_id: VarInt,

    /// Player X as a Fixed-Point number.
    pub x: i8,

    /// Player Y as a Fixed-Point number.
    pub y: i8,

    /// Player Z as a Fixed-Point number.
    pub z: i8,
    // /// New angle, not a delta
    // pub yaw: Angle,

    // /// New angle, not a delta
    // pub pitch: Angle,

    // /// Whether the player is touching the ground or not.
    // pub on_ground: bool,
}

/// Changes the direction an entity's head is facing.
#[derive(PacketDef)]
pub struct EntityHeadLookPacket {
    /// The EID of the Entity.
    pub entity_id: VarInt,

    /// New angle, not a delta
    pub yaw: Angle,
}

/// Updates an Entity's status.
///
/// # Entity Status Table
/// | Entity Status | Meaning
/// |---------------|--------------------------------------------------------------------------|
/// | 1             | Sent when resetting a mob spawn minecart's timer / Rabbit jump animation |
/// | 2             | Living Entity hurt                                                       |
/// | 3             | Living Entity dead                                                       |
/// | 4             | Iron Golem throwing up arms                                              |
/// | 6             | Wolf/Ocelot/Horse taming — Spawn “heart” particles                       |
/// | 7             | Wolf/Ocelot/Horse tamed — Spawn “smoke” particles                        |
/// | 8             | Wolf shaking water — Trigger the shaking animation                       |
/// | 9             | (of self) Eating accepted by server                                      |
/// | 10            | Sheep eating grass                                                       |
/// | 10            | Play TNT ignite sound                                                    |
/// | 11            | Iron Golem handing over a rose                                           |
/// | 12            | Villager mating — Spawn “heart” particles                                |
/// | 13            | Spawn particles indicating that a villager is angry and seeking revenge  |
/// | 14            | Spawn happy particles near a villager                                    |
/// | 15            | Witch animation — Spawn “magic” particles                                |
/// | 16            | Play zombie converting into a villager sound                             |
/// | 17            | Firework exploding                                                       |
/// | 18            | Animal in love (ready to mate) — Spawn “heart” particles                 |
/// | 19            | Reset squid rotation                                                     |
/// | 20            | Spawn explosion particle — works for some living entities                |
/// | 21            | Play guardian sound — works for only for guardians                       |
/// | 22            | Enables reduced debug for players                                        |
/// | 23            | Disables reduced debug for players

#[derive(PacketDef)]
pub struct EntityStatusPacket {
    /// The EID of the Entity.
    pub entity_id: i32,

    /// The status value.
    pub status: i8,
}

/// This packet is sent when a player has been attached to an entity (e.g. Minecart.)
#[derive(PacketDef)]
pub struct AttachEntityPacket {
    /// Attached Entity's EID.
    pub entity_id: i32,

    /// EID of the Vehicle. Set to -1 to detach.
    pub vehicle_id: i32,
    // /// If true leashes the entity to the vehicle.
    // pub leash: bool,
}

/// Updates one or more metadata properties for an existing entity. Any properties not included in
/// the Metadata field are left unchanged.
#[derive(PacketDef)]
pub struct EntityMetadataPacket {
    /// EID of the Entity.
    pub entity_id: VarInt,
    // /// Metadata of Entity.
    // pub metadata: Metadata
}

#[derive(PacketDef)]
pub struct EntityEffectPacket {
    /// EID of the Entity.
    pub entity_id: VarInt,

    /// ID of the Effect.
    pub effect_id: i8,

    /// Notchian client displays effect level as Amplifier + 1.
    pub amplifier: i8,

    /// Duration in seconds.
    pub duration: VarInt,
    // /// Whether particles should be hidden or not.
    // pub hide_particles: bool,
}

#[derive(PacketDef)]
pub struct RemoveEntityEffectPacket {
    /// EID of the Entity.
    pub entity_id: VarInt,

    /// ID of the Effect.
    pub effect_id: i8,
}

/// Sent by the server when the client should change experience levels.
#[derive(PacketDef)]
pub struct SetExperiencePacket {
    /// Between 0 and 1.
    pub bar: f32,

    /// The level.
    pub level: VarInt,

    /// See [Experience#Leveling](https://minecraft.fandom.com/wiki/Experience%23Leveling_up) up on
    /// the Minecraft Wiki for Total Experience to Level conversion.
    pub total_exp: VarInt,
}

/// Sets attributes on the given entity.
///
/// # Property Table
/// | Key                              Default              Min          Max                  Label                       |
/// |--------------------------------|--------------------|------------|--------------------|-----------------------------|
/// | generic.maxHealth              | 20.0               | 0.0        | Double.MaxValue    | Max Health                  |
/// | generic.followRange            | 32.0               | 0.0        | 2048.0             | Follow Range                |
/// | generic.knockbackResistance    | 0.0                | 0.0        | 1.0                | Knockback Resistance        |
/// | generic.movementSpeed          | 0.699999988079071  | 0.0        | Double.MaxValue    | Movement Speed              |
/// | generic.attackDamage           | 2.0                | 0.0        | Double.MaxValue    |                             |
/// | horse.jumpStrength             | 0.7                | 0.0        | 2.0                | Jump Strength               |
/// | zombie.spawnReinforcements     | 0.0                | 0.0        | 1.0                | Spawn Reinforcements Chance |
#[derive(PacketDef)]
pub struct EntityPropertiesPacket {
    /// EID of the Entity.
    pub entity_id: VarInt,

    /// Number of elements in the following array.
    pub properties_amount: i32,
    // TODO: ??
}

/// Chunks are not unloaded by the client automatically. To unload chunks, send this packet with
/// Ground-Up Continuous=true and no 16^3 chunks (eg. Primary Bit Mask=0). The server does not send
/// skylight information for nether-chunks, it's up to the client to know if the player is
/// currently in the nether. You can also infer this information from the primary bitmask and the
/// amount of uncompressed bytes sent.
#[derive(PacketDef)]
pub struct ChunkDataPacket {
    /// Chunk X coordinate.
    pub chunk_x: i32,

    /// Chunk Z coordinate.
    pub chunk_z: i32,

    ///// This is true if the packet represents all sections in this vertical column, where the
    ///// Primary Bit Mask specifies exactly which sections are included, and which are air
    // pub ground_up_continuous: bool,
    /// Bitmask with 1 for every 16x16x16 section whose data follows in the compressed data.
    pub primary_bit_mask: u16,

    /// Size of Data.
    pub size: VarInt,
    // pub data: Chunk
}

/// Fired whenever 2 or more blocks are changed within the render distance.
#[derive(PacketDef)]
pub struct MultiBlockChangePacket {
    /// Chunk X coordinate.
    pub chunk_x: i32,

    /// Chunk Z coordinate.
    pub chunk_z: i32,

    /// Number of elements in the following array, i.e. the number of blocks affected.
    pub record_count: VarInt,
    // TODO: ??
}

/// Fired whenever a block is changed within the render distance.
#[derive(PacketDef)]
pub struct BlockChangePacket {
    /// Block Coordinates.
    pub location: Position,

    /// The new block state ID for the block as given in the global palette (When reading data:
    /// `type = id >> 4`, `meta = id & 15`, when writing data: `id = type << 4 | (meta & 15))`
    pub block_id: VarInt,
}

/// This packet is used for a number of things:
///
/// - Chests opening and closing
/// - Pistons pushing and pulling
/// - Note blocks playing
/// - Updating beacons
#[derive(PacketDef)]
pub struct BlockActionPacket {
    /// Block coordinates.
    pub location: Position,

    /// Varies depending on block — see Block Actions.
    pub byte_1: u8,

    /// Varies depending on block — see Block Actions.
    pub byte_2: u8,

    /// The block type ID for the block, not including metadata/damage value.
    pub block_type: VarInt,
}

/// 0–9 are the displayable destroy stages and each other number means that there is no animation
/// on this coordinate.
///
/// You can also set an animation to air! The animation will still be visible.
///
/// If you need to display several break animations at the same time you have to give each of them
/// a unique Entity ID.
///
/// Also if you set the coordinates to a special block like water etc. it won't show the actual
/// break animation but some other interesting effects. For example, water will lose its
/// transparency.
#[derive(PacketDef)]
pub struct BlockBreakAnimationPacket {
    /// EID for the animation.
    pub entity_id: VarInt,

    /// Block position.
    pub location: Position,

    /// 0–9 to set it, any other value to remove it.
    pub destroy_stage: i8,
}

/// 1.8 changes at Chunk Data
///
/// To reduce the number of bytes, this packet is used to send chunks together for better
/// compression results.
#[derive(PacketDef)]
pub struct MapChunkBulkPacket {
    // /// Whether or not Chunk Data contains light nibble arrays. This is true in the Overworld,
    // /// false in the End + Nether.
    // pub sky_light_sent: bool,
    pub column_count: VarInt,
    // TODO: ??

    // /// Each chunk in this array corresponds to the data at the same position in Chunk Meta.
    // pub chunk_data: Vec<Chunk>
}

/// Sent when an explosion occurs (creepers, TNT, and ghast fireballs).
///
/// Each block in Records is set to air. Coordinates for each axis in record is int(X) + record.x
#[derive(PacketDef)]
pub struct ExplosionPacket {
    /// Explosion location on the X Axis.
    pub x: f32,

    /// Explosion location on the Y Axis.
    pub y: f32,

    /// Explosion location on the Z Axis.
    pub z: f32,

    /// Currently unused in the client.
    pub radius: f32,

    /// Number of elements in the following array.
    pub record_count: i32,

    /// Each record is 3 signed bytes long, each bytes are the XYZ (respectively) offsets of
    /// affected blocks.
    // pub records: Vec<(i8, i8, i8)>,

    /// X velocity of the player being pushed by the explosion
    pub player_motion_x: f32,

    /// Y velocity of the player being pushed by the explosion
    pub player_motion_y: f32,

    /// Z velocity of the player being pushed by the explosion
    pub player_motion_z: f32,
}

/// Sent when a client is to play a sound or particle effect.
///
/// By default, the Minecraft client adjusts the volume of sound effects based on distance. The
/// final boolean field is used to disable this, and instead the effect is played from 2 blocks
/// away in the correct direction. Currently this is only used for effect 1013 (mob.wither.spawn),
/// and is ignored for any other value by the client.
#[derive(PacketDef)]
pub struct EffectPacket {
    /// The ID of the effect.
    pub effect_id: i32,

    /// The location of the effect.
    pub location: Position,

    /// Extra data for certain effects, see below.
    pub data: i32,
    // pub disable_relative_volume: bool,
}

/// Used to play a sound effect on the client.
///
/// Custom sounds may be added by resource packs.
#[derive(PacketDef)]
pub struct SoundEffectPacket {
    /// All known sound effect names can be seen here:
    /// https://github.com/SirCmpwn/Craft.Net/blob/master/source/Craft.Net.Common/SoundEffect.cs
    pub sound_name: String,

    /// Effect X multiplied by 8.
    pub effect_position_x: i32,

    /// Effect Y multiplied by 8.
    pub effect_position_y: i32,

    /// Effect Z multiplied by 8.
    pub effect_position_z: i32,

    /// 1 is 100%, can be more.
    pub volume: f32,

    /// 63 is 100%, can be more.
    pub pitch: u8,
}

/// Displays the named particle.
#[derive(PacketDef)]
pub struct ParticlePacket {
    /// The ID of the Particle.
    pub particle_id: i32,

    // /// If true, particle distance increases from 256 to 65536.
    // pub long_distance: bool,
    /// X position of the particle.
    pub x: f32,

    /// Y position of the particle.
    pub y: f32,

    /// Z position of the particle.
    pub z: f32,

    /// This is added to the X position after being multiplied by random.nextGaussian()
    pub offset_x: f32,

    /// This is added to the Y position after being multiplied by random.nextGaussian()
    pub offset_y: f32,

    /// This is added to the Z position after being multiplied by random.nextGaussian()
    pub offset_z: f32,

    /// The data of each particle.
    pub particle_data: f32,

    /// The number of particles to create.
    pub particle_count: i32,
    // /// Length depends on particle. "iconcrack" has length of 2, "blockcrack", and "blockdust" have
    // /// lengths of 1, the rest have 0.
    // pub data: Vec<VarInt>,
}

/// It appears when a bed can't be used as a spawn point and when the rain state changes.
///
/// # Reason codes Table
/// | ID | Reason                                 | Note                                                                                                                     |
/// |---------------------------------------------|--------------------------------------------------------------------------------------------------------------------------|
/// | 0  | Invalid Bed                            |                                                                                                                          |
/// | 1  | End raining                            |                                                                                                                          |
/// | 2  | Begin raining                          |                                                                                                                          |
/// | 3  | Change game mode                       | 0: Survival, 1: Creative, 2: Adventure, 3: Spectator                                                                     |
/// | 4  | Enter credits                          |                                                                                                                          |
/// | 5  | Demo message                           | 0: Show welcome to demo screen, 101: Tell movement controls, 102: Tell jump control, 103: Tell inventory control         |
/// | 6  | Arrow hitting player                   | Appears to be played when an arrow strikes another player in Multiplayer                                                 |
/// | 7  | Fade value                             | The current darkness value. 1 = Dark, 0 = Bright, Setting the value higher causes the game to change color and freeze    |
/// | 8  | Fade time                              | Time in ticks for the sky to fade                                                                                        |
/// | 10 | Play mob appearance (effect and sound) | Unknown                                                                                                                  |
#[derive(PacketDef)]
pub struct ChangeGameStatePacket {
    /// Reason code.
    pub reason: u8,

    /// Depends on reason.
    pub value: f32,
}

/// With this packet, the server notifies the client of thunderbolts striking within a 512 block
/// radius around the player. The coordinates specify where exactly the thunderbolt strikes.
#[derive(PacketDef)]
pub struct SpawnGlobalEntityPacket {
    /// The EID of the thunderbolt.
    pub entity_id: VarInt,

    /// The global entity type, currently always 1 for thunderbolt.
    pub type_: i8,

    /// Thunderbolt X, a fixed-point number.
    pub x: i32,

    /// Thunderbolt Y, a fixed-point number.
    pub y: i32,

    /// Thunderbolt Z, a fixed-point number.
    pub z: i32,
}

// /// This is sent to the client when it should open an inventory, such as a chest, workbench, or
// /// furnace. This message is not sent anywhere for clients opening their own inventory.
// #[derive(PacketDef)]
// pub struct OpenWindowPacket<'a> {
//     /// A unique id number for the window to be displayed. Notchian server implementation is a
//     /// counter, starting at 1.
//     pub id: u8,
//
//     /// The window type to use for display. See Inventory for a list.
//     pub type_: String,
//
//     /// The title of the window.
//     pub title: ChatComponent<'a>,
//
//     /// Number of slots in the window (excluding the number of slots in the player inventory).
//     pub slots: u8,
//
//     /// EntityHorse's EID. Only sent when Window Type is “EntityHorse".
//     pub entity_id: Option<i32>,
// }

/// This packet is sent from the server to the client when a window is forcibly closed, such as
/// when a chest is destroyed while it's open.
///
/// Note, notchian clients send a close window packet with Window ID 0 to close their inventory
/// even though there is never an Open Window packet for inventory.
#[derive(PacketDef)]
pub struct CloseWindowPacket {
    /// This is the ID of the window that was closed. 0 for inventory.
    pub id: u8,
}

/// Sent by the server when an item in a slot (in a window) is added/removed.
#[derive(PacketDef)]
pub struct SetSlotPacket {
    /// The window which is being updated. 0 for player inventory. Note that all known window types
    /// include the player inventory. This packet will only be sent for the currently opened window
    /// while the player is performing actions, even if it affects the player inventory. After the
    /// window is closed, a number of these packets are sent to update the player's inventory
    /// window (0).
    pub id: i8,

    /// The slot that should be updated.
    pub slot: i16,
    // pub data: Slot
}

/// Sent by the server when items in multiple slots (in a window) are added/removed. This includes
/// the main inventory, equipped armour and crafting slots.
#[derive(PacketDef)]
pub struct WindowItemsPacket {
    /// The ID of window which items are being sent for. 0 for player inventory.
    pub id: u8,

    /// Number of elements in the following array.
    pub count: i16,
    // pub data: Vec<Slot>
}

/// This packet is used to inform the client that part of a GUI window should be updated.
///
/// TODO: add table
#[derive(PacketDef)]
pub struct WindowPropertyPacket {
    /// The ID of a window.
    pub id: u8,

    /// The property to be updated, see below.
    pub property: i16,

    /// The new value for the property, see below.
    pub value: i16,
}

/// A packet from the server indicating whether a request from the client was accepted, or whether
/// there was a conflict (due to lag).
#[derive(PacketDef)]
pub struct ConfirmTransactionPacket {
    /// The ID of the window that the action occurred in.
    pub id: i8,

    /// Every action that is to be accepted has a unique number. This field corresponds to that number.
    pub action_number: i16,
    // /// Whether the action was accepted.
    // pub accepted: bool,
}

/// This message is sent from the server to the client whenever a sign is discovered or created.
/// This message is NOT sent when a sign is destroyed or unloaded.
// #[derive(PacketDef)]
// pub struct UpdateSignPacket<'a> {
//     /// Location of the sign.
//     pub location: Position,
//
//     /// First line of text in the sign.
//     pub line1: ChatComponent<'a>,
//
//     /// Second line of text in the sign.
//     pub line2: ChatComponent<'a>,
//
//     /// Third line of text in the sign.
//     pub line3: ChatComponent<'a>,
//
//     /// Fourth line of text in the sign.
//     pub line4: ChatComponent<'a>,
// }

/// Updates a rectangular area on a map.
#[derive(PacketDef)]
pub struct MapPacket {
    /// The damage value (map ID) of the map being modified.
    pub item_damage: VarInt,

    pub scale: i8,

    /// Number of elements in the following array.
    pub icon_amount: VarInt,

    // TODO: ??
    /// Number of columns updated.
    pub column: i8,
    // /// Only if Columns is more than 0; number of rows updated.
    // pub rows: Option<i8>,

    // /// Only if Columns is more than 0; x offset of the westernmost column.
    // pub x: Option<i8>,

    // /// Only if Columns is more than 0; z offset of the northernmost row.
    // pub z: Option<i8>,

    // /// Only if Columns is more than 0; length of the following array.
    // pub length: Option<VarInt>,

    // /// Only if Columns is more than 0; see Map item format.
    // pub data: Option<Vec<u8>>,
}

/// Essentially a block update on a block entity.
///
/// # Action Table                                                          |
/// | Action | Description                                                  |
/// |--------|--------------------------------------------------------------|
/// |    1   | Set SpawnPotentials of a mob spawner                         |
/// |    2   | Set command block text (command and last execution status)   |
/// |    3   | Set the level, primary, and secondary powers of a beacon     |
/// |    4   | Set rotation and skin of mob head                            |
/// |    5   | Set type of flower in flower pot                             |
/// |    6   | Set base color and patterns on a banner                      |
#[derive(PacketDef)]
pub struct UpdateBlockEntityPacket {
    /// TODO: think what to write here
    pub location: Position,

    /// The type of update to perform, see below.
    pub action: u8,
    // /// If not present then it's a TAG_END (0)
    // pub data: NBTTag
}

/// Sent when the client has placed a sign and is allowed to send Update Sign.
#[derive(PacketDef)]
pub struct OpenSignEditorPacket {
    /// TODO: think what to write here
    pub location: Position,
}

#[derive(PacketDef)]
pub struct StatisticsPacket {
    /// Number of elements in the following array.
    pub count: VarInt,
    // TODO: ??
}

/// Sent by the notchian server to update the user list (<tab> in the client.)
#[derive(PacketDef)]
pub struct PlayerListItemPacket {
    /// Determines the rest of the Player format after the UUID.
    pub action: VarInt,

    /// Number of elements in the following array.
    pub players_amount: VarInt,
    // TODO: ??
}

/// The latter 2 floats are used to indicate the field of view and flying speed respectively, while
/// the first byte is used to determine the value of 4 booleans.
///
/// # Flags
/// TODO
#[derive(PacketDef)]
pub struct PlayerAbilitiesPacket {
    /// Bit field.
    pub flags: i8,

    pub flying_speed: f32,

    /// Modifies the field of view, like a speed potion. A Notchian server will use the same value
    /// as the movement speed (send in the Entity Properties packet.)
    pub fov_modifier: f32,
}

/// The server responds with a list of auto-completions of the last word sent to it. In the case of
/// regular chat, this is a player username. Command names and parameters are also supported.
#[derive(PacketDef)]
pub struct TabCompletePacket {
    /// Number of elements in the following array.
    pub flags: VarInt,
    // /// One eligible command, note that each command is sent separately instead of in a single
    // /// string, hence the need for Count
    // pub matches: Vec<String>,
}

/// This is sent to the client when it should create a new scoreboard objective or remove one.
#[derive(PacketDef)]
pub struct ScoreboardObjectivePacket {
    /// An unique name for the objective.
    pub name: String,

    /// 0 to create the scoreboard. 1 to remove the scoreboard. 2 to update the display text.
    pub mode: i8,
    // /// Only if mode is 0 or 2. The text to be displayed for the score.
    // pub value: Option<String>,

    // /// Only if mode is 0 or 2. “integer” or “hearts”
    // pub type_: Option<String>,
}

/// This is sent to the client when it should update a scoreboard item.
#[derive(PacketDef)]
pub struct UpdateScorePacket {
    /// The name of the score to be updated or removed.
    pub name: String,

    /// 0 to create/update an item. 1 to remove an item.
    pub action: i8,

    /// The name of the objective the score belongs to.
    pub objective_name: String,
    // /// The score to be displayed next to the entry. Only sent when Action does not equal 1.
    // pub value: Option<VarInt>,
}

/// This is sent to the client when it should display a scoreboard.
#[derive(PacketDef)]
pub struct DisplayScoreboardPacket {
    /// The position of the scoreboard. 0: list, 1: sidebar, 2: below name.
    pub position: i8,

    /// The unique name for the scoreboard to be displayed.
    pub name: String,
}

/// Creates and updates teams.
#[derive(PacketDef)]
pub struct TeamPacket {
    /// A unique name for the team. (Shared with scoreboard).
    pub name: String,

    /// If 0 then the team is created.
    /// If 1 then the team is removed.
    /// If 2 the team team information is updated.
    /// If 3 then new players are added to the team.
    /// If 4 then players are removed from the team.
    pub mode: i8,
    // /// Only if Mode = 0 or 2.
    // pub display_name: Option<String>,

    // /// Only if Mode = 0 or 2. Displayed before the players' name that are part of this team.
    // pub prefix: Option<String>,

    // /// Only if Mode = 0 or 2. Displayed after the players' name that are part of this team.
    // pub suffix: Option<String>,

    // /// Only if Mode = 0 or 2. 0 for off, 1 for on, 3 for seeing friendly invisibles.
    // pub friendly_fire: Option<i8>,

    // /// Only if Mode = 0 or 2. always, hideForOtherTeams, hideForOwnTeam, never.
    // pub name_tag_visibility: Option<String>,

    // /// Only if Mode = 0 or 2. Same as Chat colors.
    // pub color: Option<i8>,

    // /// Only if Mode = 0 or 3 or 4. Number of players in the array.
    // pub players_amount: Option<VarInt>,

    // /// Only if Mode = 0 or 3 or 4. Players to be added/remove from the team. Max 40 characters so
    // /// may be uuid's later.
    // pub players: Option<String>,
}

/// Mods and plugins can use this to send their data. Minecraft itself uses a number of plugin
/// channels. These internal channels are prefixed with MC|.
///
/// More documentation on this:
/// http://dinnerbone.com/blog/2012/01/13/minecraft-plugin-channels-messaging/
#[derive(PacketDef)]
pub struct PluginMessagePacket {
    /// Name of the plugin channel used to send the data.
    pub channel: String,
    // /// Any data, depending on the channel. MC| channels are documented here.
    // pub data: Vec<i8>,
}

// /// Sent by the server before it disconnects a client. The client assumes that the server has already closed the connection by the time the packet arrives.
// #[derive(PacketDef)]
// pub struct DisconnectPacket<'a> {
//     /// Displayed to the client when the connection terminates.
//     pub reason: ChatComponent<'a>,
// }

/// Changes the difficulty setting in the client's option menu.
#[derive(PacketDef)]
pub struct ServerDifficultyPacket {
    /// 0: peaceful, 1: easy, 2: normal, 3: hard.
    pub difficulty: u8,
}

#[derive(PacketDef)]
pub struct CombatEventPacket {
    /// 0: enter combat, 1: end combat, 2: entity dead.
    pub event: VarInt,

    // /// Only for end combat.
    // pub duration: Option<VarInt>,

    // /// Only for entity dead.
    // pub player_id: Option<VarInt>,

    // /// Only for end combat and entity dead.
    // pub entity_id: Option<i32>,
    /// Only for entity dead.
    pub message: String,
}

/// Sets the entity that the player renders from. This is normally used when the left-clicks an
/// entity while in spectator mode.
///
/// The player's camera will move with the entity and look where it is looking. The entity is often
/// another player, but can be any type of entity. The player is unable to move this entity (move
/// packets will act as if they are coming from the other entity).
///
/// If the given entity is not loaded by the player, this packet is ignored. To return control to
/// the player, send this packet with their entity ID.
///
/// The Notchian server resets this (sends it back to the default entity) whenever the spectated
/// entity is killed or the player sneaks, but only if they were spectating an entity. It also
/// sends this packet whenever the player switches out of spectator mode (even if they weren't
/// spectating an entity).
#[derive(PacketDef)]
pub struct CameraPacket {
    /// ID of the entity to set the client's camera to.
    pub id: VarInt,
}

/// TODO
#[derive(PacketDef)]
pub struct WorldBorderPacket {
    /// Determines the format of the rest of the packet
    pub action: VarInt,
    // TODO: ??
}

/// Warning: This packet is completely broken and has been removed in the 1.9 snapshots. The
/// packet Set Compression (Login, 0x03, clientbound) should be used instead.
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
    /// The URL to the resource pack.
    pub url: String,

    /// A 40 character hexadecimal and lowercase SHA-1 hash of the resource pack file. (must be
    /// lower case in order to work)
    ///
    /// If it's not a 40 character hexadecimal string, the client will not use it for hash
    /// verification and likely waste bandwidth — but it will still treat it as a unique id
    pub hash: String,
}

#[derive(PacketDef)]
pub struct UpdateEntityNBTPacket {
    pub id: VarInt,
    // pub tag: NBTTag,
}
