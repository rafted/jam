use bevy_ecs::{
    prelude::Events,
    schedule::{Schedule, Stage, StageLabel, SystemStage},
    world::World,
};
use crossbeam_channel::unbounded;
use protocol::chat::ChatComponent;
use server::ServerConfiguration;

use crate::{
    connection::{Connection, PacketContainer},
    server::{accept_loop, handle_connections, handle_packet},
    sync::{sync_connections, ChannelsRes},
};

pub mod connection;
pub mod server;
pub mod sync;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // create config
    let motd = ChatComponent::builder()
        .text("A Lightweight and High-Performance Minecraft Server".to_string())
        .build();

    let config = ServerConfiguration::builder()
        .port(25565)
        .motd(motd)
        .build();

    // bind server
    let listener = server::bind(&config).await?;

    // setup ECS
    let mut world = World::new();

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct FlushEvents;

    world.insert_resource(Events::<PacketContainer>::default());
    world.insert_resource(config);

    let (connection_sender, connection_receiver) = unbounded::<Connection>();

    let channels = ChannelsRes {
        connection_sender: connection_sender.clone(),
        connection_receiver: connection_receiver.clone(),
    };

    world.insert_resource(channels);

    // run schedule loop
    let mut schedule = Schedule::default();

    // sync stage
    #[derive(StageLabel)]
    pub struct Sync;

    schedule.add_stage(Sync, SystemStage::parallel().with_system(sync_connections));

    // network stage
    #[derive(StageLabel)]
    pub struct Network;

    schedule.add_stage(
        Network,
        SystemStage::parallel()
            .with_system(Events::<PacketContainer>::update_system)
            .with_system(handle_connections)
            .with_system(handle_packet),
    );

    tokio::task::spawn(accept_loop(listener, connection_sender.clone()));

    loop {
        schedule.run(&mut world);
    }
}
