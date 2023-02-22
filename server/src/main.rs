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
    server::accept_loop,
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

    // setup ECS
    let mut world = World::new();

    world.insert_resource(Events::<PacketContainer>::default());
    world.insert_resource(config.clone());

    let (connection_sender, connection_receiver) = unbounded::<Connection>();

    let channels = ChannelsRes {
        connection_sender: connection_sender.clone(),
        connection_receiver: connection_receiver.clone(),
    };

    world.insert_resource(channels);

    // run schedule loop
    let mut schedule = Schedule::default();

    #[derive(StageLabel)]
    pub struct Sync;

    schedule.add_stage(
        Sync,
        SystemStage::single_threaded().with_system(sync_connections),
    );

    // bind server
    let listener = server::bind(&config).await?;

    tokio::task::spawn(accept_loop(listener, connection_sender.clone()));

    loop {
        schedule.run(&mut world);
    }
}
