use crate::position::Position;
use protocol_macro::PacketDef;
use crate::chat::ChatComponent;
use crate::varint::VarInt;

/// The server will frequently send out a keep-alive, each containing a random ID. The client must
/// respond with the same packet.
#[derive(PacketDef)]
pub struct KeepAlivePacket {
    /// The same random ID that was sent by the server.
    pub id: VarInt,
}

/// The default server will check the message to see if it begins with a '/'. If it doesn't, the
/// username of the sender is prepended and sent to all other clients (including the original
/// sender). If it does, the server assumes it to be a command and attempts to process it. A
/// message longer than 100 characters will cause the server to kick the client. This change was
/// initially done by allowing the client to not slice the message up to 119 (the previous limit),
/// without changes to the server. For this reason, the vanilla server kept the code to cut
/// messages at 119, but this isn't a protocol limitation and can be ignored.
#[derive(PacketDef)]
pub struct ChatMessagePacket {
    /// The client sends the raw input, not [`ChatComponent`]
    pub message: String,
}

/// This packet is sent from the client to the server when the client attacks or right-clicks
/// another entity (a player, minecart, etc).
///
/// A Notchian server only accepts this packet if the entity being attacked/used is visible without
/// obstruction and within a 4-unit radius of the player's position.
///
/// Note that middle-click in creative mode is interpreted by the client and sent as a Creative
/// Inventory Action packet instead.
#[derive(PacketDef)]
pub struct UseEntityPacket {
    pub target: VarInt,

    /// 0: interact,
    /// 1: attack,
    /// 2: interact at
    pub type_: VarInt,

    // /// only if type_ is interacted at (2)
    // pub target_x: Option<f32>,
    // /// only if type_ is interacted at (2)
    // pub target_y: Option<f32>,
    // /// only if type_ is interacted at (2)
    // pub target_z: Option<f32>,
}

/// This packet as well as Player Position (Play, 0x04, serverbound), Player Look (Play, 0x05,
/// serverbound), and Player Position And Look (Play, 0x06, serverbound) are called the
/// “serverbound movement packets”. At least one of them must be sent on each tick to ensure that
/// servers will update things like player health correctly. Vanilla clients will send Player
/// Position once every 20 ticks even for a stationary player, and Player on every other tick.
///
/// This packet is used to indicate whether the player is on ground (walking/swimming), or airborne
/// (jumping/falling).
///
/// When dropping from sufficient height, fall damage is applied when this state goes from false to
/// true. The amount of damage applied is based on the point where it last changed from true to
/// false. Note that there are several movement related packets containing this state.
#[derive(PacketDef)]
pub struct PlayerPacket {
    // /// True if the client is on the ground, false otherwise.
    // pub on_ground: bool,
}

/// Updates the player's XYZ position on the server.
///
/// If the distance between the last known position of the player on the server and the new
/// position set by this packet is greater than 100 units, this will result in the client being
/// kicked for “You moved too quickly :( (Hacking?)”
///
/// If the distance is greater than 10 units, the server will log the warning message "<name> moved
/// too quickly!", followed by two coordinate triples (maybe movement delta?), but will not kick
/// the client.
///
/// Also if the fixed-point number of X or Z is set greater than 3.2×107 the client will be kicked
/// for “Illegal position”.
#[derive(PacketDef)]
pub struct PlayerPositionPacket {
    /// Absolute position.
    pub x: f64,

    /// Absolute position.
    pub y: f64,

    /// Absolute position.
    pub z: f64,

    // /// True if the client is on the ground, false otherwise.
    // pub on_ground: bool,
}

/// Updates the direction the player is looking in.
///
/// Yaw is measured in degrees, and does not follow classical trigonometry rules. The unit circle
/// of yaw on the XZ-plane starts at (0, 1) and turns counterclockwise, with 90 at (-1, 0), 180 at
/// (0,-1) and 270 at (1, 0). Additionally, yaw is not clamped to between 0 and 360 degrees; any
/// number is valid, including negative numbers and numbers greater than 360.
///
/// Pitch is measured in degrees, where 0 is looking straight ahead, -90 is looking straight up,
/// and 90 is looking straight down.
///
/// The yaw and pitch of player (in degrees), standing at point (x0, y0, z0) and looking towards
/// point (x, y, z) one can be calculated with:
///
/// ```text
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
/// ```text
/// x = -cos(pitch) * sin(yaw)
/// y = -sin(pitch)
/// z =  cos(pitch) * cos(yaw)
/// ```
#[derive(PacketDef)]
pub struct PlayerPositionLook {
    /// Absolute rotation on the X Axis, in degrees
    pub yaw: f32,

    /// Absolute rotation on the Y Axis, in degrees
    pub pitch: f32,

    // /// True if the client is on the ground, False otherwise
    // pub on_ground: bool,
}

/// A combination of Player Look and Player Position.
#[derive(PacketDef)]
pub struct PlayerPositionAndLookPacket {
    /// Absolute position.
    pub x: f64,

    /// Absolute feet position, normally Head Y - 1.62.
    pub y: f64,

    /// Absolute position.
    pub z: f64,

    /// Absolute rotation on the X Axis, in degrees.
    pub yaw: f32,

    /// Absolute rotation on the Y Axis, in degrees.
    pub pitch: f32,

    // /// True if the client is on the ground, false otherwise.
    // pub on_ground: bool,
}

/// Sent when the player mines a block. A Notchian server only accepts digging packets with
/// coordinates within a 6-unit radius of the player's position.
///
/// # Status Table
/// | Meaning                     | Value |
/// |-----------------------------|-------|
/// | Started digging             | 0     |
/// | Cancelled digging           | 1     |
/// | Finished digging            | 2     |
/// | Drop item stack             | 3     |
/// | Drop item                   | 4     |
/// | Shoot arrow / finish eating | 5     |
///
/// Notchian clients send a 0 (started digging) when they start digging and a 2 (finished digging)
/// once they think they are finished. If digging is aborted, the client simply send a 1 (cancel
/// digging).
///
/// Status code 4 (drop item) is a special case. In-game, when you use the Drop Item command
/// (keypress 'q'), a dig packet with a status of 4, and all other values set to 0, is sent from
/// client to server. Status code 3 is similar, but drops the entire stack.
///
/// Status code 5 (shoot arrow / finish eating) is also a special case. The x, y and z fields are
/// all set to 0 like above, with the exception of the face field, which is set to 255.
///
/// The face can be one of six values, representing the face being hit:
/// | Value | Offset |
/// |-------|--------|
/// |   0   |   -Y   |
/// |   1   |   +Y   |
/// |   2   |   -Z   |
/// |   3   |   +Z   |
/// |   4   |   -X   |
/// |   5   |   +X   |
#[derive(PacketDef)]
pub struct PlayerDiggingPacket {
    /// The action the player is taking against the block.
    pub status: i8,

    /// Block position.
    pub location: Position,

    /// The face being hit.
    pub face: i8,
}

/// In normal operation (i.e. placing a block), this packet is sent once, with the values set
/// normally.
///
/// This packet has a special case where X, Y, Z, and Face are all -1. (Note that Y is unsigned so
/// set to 255.) This special packet indicates that the currently held item for the player should
/// have its state updated such as eating food, pulling back bows, using buckets, etc.
///
/// In a Notchian Beta client, the block or item ID corresponds to whatever the client is currently
/// holding, and the client sends one of these packets any time a right-click is issued on a
/// surface, so no assumptions can be made about the safety of the ID. However, with the
/// implementation of server-side inventory, a Notchian server seems to ignore the item ID, instead
/// operating on server-side inventory information and holding selection. The client has been
/// observed (1.2.5 and 1.3.2) to send both real item IDs and -1 in a single session.
///
/// Special note on using buckets: When using buckets, the Notchian client might send two packets:
/// first a normal and then a special case. The first normal packet is sent when you're looking at
/// a block (e.g. the water you want to scoop up). This normal packet does not appear to do
/// anything with a Notchian server. The second, special case packet appears to perform the action
/// — based on current position/orientation and with a distance check — it appears that buckets can
/// only be used within a radius of 6 units.
#[derive(PacketDef)]
pub struct PlayerBlockPlacementPacket {
    /// Block position.
    pub location: Position,

    /// The face on which the block is placed.
    pub face: i8,

    // pub held_item: Slot,
    
    /// The position of the crosshair on the block.
    pub cursor_position_x: i8,
    pub cursor_position_y: i8,
    pub cursor_position_z: i8,
}

/// Sent when the player changes the slot selection.
#[derive(PacketDef)]
pub struct HeldItemChangePacket {
    /// The slot which the player has selected (0–8.)
    pub slot: i16,
}

/// Sent when the player's arm swings.
#[derive(PacketDef)]
pub struct AnimationPacket {}

/// Sent by the client to indicate that it has performed certain actions: sneaking (crouching),
/// sprinting, exiting a bed, jumping with a horse, and opening a horse's inventory while riding
/// it.
///
/// # Action ID Table
/// | ID | Action                      |
/// |----|-----------------------------|
/// | 0  | Start sneaking              |
/// | 1  | Stop sneaking               |
/// | 2  | Leave bed                   |
/// | 3  | Start sprinting             |
/// | 4  | Stop sprinting              |
/// | 5  | Jump with horse             |
/// | 6  | Open ridden horse inventory |
///
/// Leave Bed is only sent when the "Leave Bed" button is clicked on the sleep GUI, not when waking
/// up due today time.
///
/// Open ridden horse inventory is only sent when pressing the inventory key on a horse - all other
/// methods of opening a horse's inventory (involving right-clicking or shift-right-clicking it) do
/// not use this packet.
#[derive(PacketDef)]
pub struct EntityActionPacket {
    /// Player ID.
    pub entity_id: VarInt,

    /// The ID of the action.
    pub action_id: VarInt,

    /// Only used by Horse Jump Boost, in which case it ranges from 0 to 100. In all other cases it
    /// is 0.
    pub action_parameter: VarInt,
}

#[derive(PacketDef)]
pub struct SteerVehiclePacket {
    /// Positive to the left of the player.
    pub sideways: f32,

    /// Positive forward.
    pub forward: f32,

    /// Bit mask. 0x1: jump, 0x2: unmount.
    pub flags: u8,
}

/// This packet is sent by the client when closing a window.
///
/// Notchian clients send a Close Window packet with Window ID 0 to close their inventory even
/// though there is never an Open Window packet for the inventory.
#[derive(PacketDef)]
pub struct CloseWindowPacket {
    /// This is the ID of the window that was closed. 0 for player inventory.
    pub id: u8,
}

/// This packet is sent by the player when it clicks on a slot in a window.
///
/// When right-clicking on a stack of items, half the stack will be picked up and half left in the
/// slot. If the stack is an odd number, the half left in the slot will be smaller of the amounts.
///
/// The distinct type of click performed by the client is determined by the combination of the Mode
/// and Button fields.
///
/// | Mode | Button | Slot    | Trigger                                               |
/// |------|--------|---------|-------------------------------------------------------|
/// | 0    | 0      | Normal  | Left mouse click                                      |
/// | 0    | 1      | Normal  | Right mouse click                                     |
/// | 1    | 0      | Normal  | Shift + left mouse click                              |
/// | 1    | 1      | Normal  | Shift + right mouse click                             |
/// | 2    | 0      | Normal  | Number key 1                                          |
/// | 2    | 1      | Normal  | Number key 2                                          |
/// | 2    | 2      | Normal  | Number key 3                                          |
/// | 2    | 3      | Normal  | Number key 4                                          |
/// | 2    | 4      | Normal  | Number key 5                                          |
/// | 2    | 5      | Normal  | Number key 6                                          |
/// | 2    | 6      | Normal  | Number key 7                                          |
/// | 2    | 7      | Normal  | Number key 8                                          |
/// | 2    | 8      | Normal  | Number key 9                                          |
/// | 3    | 2      | Normal  | Middle click                                          |
/// | 4    | 0      | Normal* | Drop key (Q) (* Clicked item is different, see above) |
/// | 4    | 1      | Normal* | Ctrl + Drop key (Ctrl-Q) (drops full stack)           |
/// | 4    | 0      | -999    | Left click outside inventory holding nothing (no-op)  |
/// | 4    | 1      | -999    | Right click outside inventory holding nothing (no-op) |
/// | 5    | 0      | -999    | Starting left mouse drag (or middle mouse)            |
/// | 5    | 4      | -999    | Starting right mouse drag                             |
/// | 5    | 1      | Normal  | Add slot for left-mouse drag                          |
/// | 5    | 5      | Normal  | Add slot for right-mouse drag                         |
/// | 5    | 2      | -999    | Ending left-mouse drag                                |
/// | 5    | 6      | -999    | Ending right-mouse drag                               |
/// | 6    | 0      | Normal  | Double click                                          |
///
/// Starting from version 1.5, “painting mode” is available for use in inventory windows. It is
/// done by picking up stack of something (more than 1 item), then holding mouse button (left,
/// right or middle) and dragging held stack over empty (or same type in case of right button)
/// slots. In that case client sends the following to server after mouse button release (omitting
/// first pickup packet which is sent as usual):
/// 1. packet with mode 5, slot -999, button (0 for left | 4 for right);
/// 2. packet for every slot painted on, mode is still 5, button (1 | 5);
/// 3. packet with mode 5, slot -999, button (2 | 6);
///
/// If any of the painting packets other than the “progress” ones are sent out of order (for
/// example, a start, some slots, then another start; or a left-click in the middle) the painting
/// status will be reset.

/// The server will send back a Confirm Transaction packet. If the click was not accepted, the
/// client must reflect that packet before sending more Click Window packets, otherwise the server
/// will reject them silently. The Notchian server also sends a Window Items packet for the open
/// window and Set Slot packets for the clicked and cursor slot, but only when the click was not
/// accepted, probably to resynchronize client and server.
#[derive(PacketDef)]
pub struct ClickWindowPacket {
    /// The ID of the window which was clicked. 0 for player inventory.
    pub id: u8,

    /// The clicked slot number.
    pub slot: i16,

    /// The button used in the click.
    pub button: i8,

    /// A unique number for the action, implemented by Notchian as a counter, starting at 1. Used
    /// by the server to send back a Confirm Transaction.
    pub action_number: i16,
    // TODO: ??

    // /// The clicked slot. Has to be empty (item ID = -1) for drop mode.
    // pub clicked_item: Slot
}

/// If a transaction sent by the client was not accepted, the server will reply with a Confirm
/// Transaction (Play, 0x32, clientbound) packet with the Accepted field set to false. When this
/// happens, the client must reflect the packet to apologize (as with movement), otherwise the
/// server ignores any successive transactions.
#[derive(PacketDef)]
pub struct ConfirmTransactionPacket {
    /// The ID of the window that the action occurred in.
    pub window_id: i8,

    /// Every action that is to be accepted has a unique number. This field corresponds to that
    /// number.
    pub action_number: i16,

    // /// Whether the action was accepted.
    // pub accepted: bool,
}

/// While the user is in the standard inventory (i.e., not a crafting bench) in Creative mode, the
/// player will send this packet.
///
/// Clicking in the creative inventory menu is quite different from non-creative inventory
/// management. Picking up an item with the mouse actually deletes the item from the server, and
/// placing an item into a slot or dropping it out of the inventory actually tells the server to
/// create the item from scratch. (This can be verified by clicking an item that you don't mind
/// deleting, then severing the connection to the server; the item will be nowhere to be found when
/// you log back in.) As a result of this implementation strategy, the "Destroy Item" slot is just
/// a client-side implementation detail that means "I don't intend to recreate this item.".
/// Additionally, the long listings of items (by category, etc.) are a client-side interface for
/// choosing which item to create. Picking up an item from such listings sends no packets to the
/// server; only when you put it somewhere does it tell the server to create the item in that
/// location.
///
/// This action can be described as "set inventory slot". Picking up an item sets the slot to item
/// ID -1. Placing an item into an inventory slot sets the slot to the specified item. Dropping an
/// item (by clicking outside the window) effectively sets slot -1 to the specified item, which
/// causes the server to spawn the item entity, etc.. All other inventory slots are numbered the
/// same as the non-creative inventory (including slots for the 2x2 crafting menu, even though they
/// aren't visible in the vanilla client).
#[derive(PacketDef)]
pub struct CreativeInventoryActionPacket {
    /// Inventory slot.
    pub slot: i16,

    // pub clicked_item: Slot,
}

/// The ID of the enchantment table window sent by Open Window.
#[derive(PacketDef)]
pub struct EnchantItemPacket {
    /// The ID of the enchantment table window sent by Open Window.
    pub window_id: i8,

    /// The position of the enchantment on the enchantment table window, starting with 0 as the
    /// topmost one.
    pub enchantment: i8,
}

/// This message is sent from the client to the server when the “Done” button is pushed after
/// placing a sign.
///
/// The server only accepts this packet after Open Sign Editor, otherwise this packet is silently
/// ignored.
#[derive(PacketDef)]
pub struct UpdateSignPacket<'a> {
    /// Block Coordinates.
    pub location: Position,

    /// First line of text in the sign.
    pub line_1: ChatComponent<'a>,

    /// Second line of text in the sign.
    pub line_2: ChatComponent<'a>,

    /// Third line of text in the sign.
    pub line_3: ChatComponent<'a>,

    /// Fourth line of text in the sign.
    pub line_4: ChatComponent<'a>,
}

/// The latter 2 bytes are used to indicate the walking and flying speeds respectively, while the
/// first byte is used to determine the value of 4 booleans.
///
/// The vanilla client sends this packet when the player starts/stops flying with the Flags
/// parameter changed accordingly. All other parameters are ignored by the vanilla server.
#[derive(PacketDef)]
pub struct PlayerAbilitiesPacket {
    /// Bit mask. 0x08: damage disabled (god mode),
    /// 0x04: can fly,
    /// 0x02: is flying, 0x01: is Creative.
    pub flags: i8,

    pub flying_speed: f32,

    pub walking_speed: f32,
}

/// Sent when the user presses tab while writing text.
#[derive(PacketDef)]
pub struct TabCompletePacket {
    /// All text behind the cursor.
    pub text: String,

    // pub has_position: bool,

    // /// The position of the block being looked at. Only sent if Has Position is true.
    // pub looked_at_block: Option<Position>,
}

/// Sent when the player connects, or when settings are changed.
///
///
#[derive(PacketDef)]
pub struct ClientSettingsPacket {
    /// e.g. en_GB.
    pub locale: String,

    /// Client-side render distance, in chunks.
    pub view_distance: i8,

    /// 0: enabled,
    /// 1: commands only,
    /// 2: hidden.
    pub chat_mode: i8,

    // /// “Colors” multiplayer setting.
    // pub chat_colors: bool,
    
    /// Skin parts.
    ///
    /// Bit 0 (0x01): Cape enabled
    /// Bit 1 (0x02): Jacket enabled
    /// Bit 2 (0x04): Left Sleeve enabled
    /// Bit 3 (0x08): Right Sleeve enabled
    /// Bit 4 (0x10): Left Pants Leg enabled
    /// Bit 5 (0x20): Right Pants Leg enabled
    /// Bit 6 (0x40): Hat enabled
    ///
    /// The most significant bit (bit 7, 0x80) appears to be unused.
    pub display_skin_parts: u8,
}

/// Sent when the client is ready to complete login and when the client is ready to respawn after
/// death.
///
/// TODO: make table
#[derive(PacketDef)]
pub struct ClientStatusPacket {
    pub action_id: VarInt,
}

/// Mods and plugins can use this to send their data. Minecraft itself uses a number of plugin
/// channels. These internal channels are prefixed with MC|.
///
/// More documentation on this:
/// http://dinnerbone.com/blog/2012/01/13/minecraft-plugin-channels-messaging/
///
/// Note that the length of Data is known only from the packet length, since the packet has no
/// length field of any kind.
#[derive(PacketDef)]
pub struct PluginMessagePacket {
    // Name of the plugin channel used to send the data

    /// Any data, depending on the channel. MC| channels are documented here.
    pub channel: String,

    // /// TODO: According to #mcdevs, the length of Data is known only from the packet length, since
    // /// the packet has no length field of any kind.
    // pub data: Vec<i8>,
}

/// Teleports the player to the given entity. The player must be in spectator mode.
///
/// The Notchian client only uses this to teleport to players, but it appears to accept any type of
/// entity. The entity does not need to be in the same dimension as the player; if necessary, the
/// player will be respawned in the right world. If the given entity cannot be found (or isn't
/// loaded), this packet will be ignored. It will also be ignored if the player attempts to
/// teleport to themselves.
#[derive(PacketDef)]
pub struct SpectatePacket {
    // /// UUID of the player to teleport to (can also be an entity UUID)
    // pub target: UUID,
}

#[derive(PacketDef)]
pub struct ResourcePackStatusPacket {
    /// The hash sent in the Resource Pack Send packet.
    pub hash: String,

    /// 0: successfully loaded,
    /// 1: declined,
    /// 2: failed download,
    /// 3: accepted.
    pub result: VarInt,
}
