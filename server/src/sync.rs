use bevy_ecs::system::{Commands, Res, Resource};
use crossbeam_channel::{Receiver, Sender};

use crate::connection::Connection;

///! This module takes care of synchronizing the async parts with the ECS.

#[derive(Resource, Clone)]
pub struct ChannelsRes {
    pub connection_sender: Sender<Connection>,
    pub connection_receiver: Receiver<Connection>,
}

/// System that creates entities based on [Connection]s from [ChannelRes].
pub fn sync_connections(channels: Res<ChannelsRes>, mut commands: Commands) {
    // try to read an event from the channel. if not read, just return
    match channels.connection_receiver.try_recv() {
        Ok(connection) => {
            // if an event is read, get the connection and attach it to an entity
            let id = commands.spawn(connection).id().index();

            println!("new connection: [entity id: {}]", id);
        }
        Err(_) => {}
    }
}
