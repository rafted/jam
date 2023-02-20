use std::net::TcpListener;

use bevy_ecs::{
    schedule::{Schedule, Stage, StageLabel, SystemStage},
    world::World,
};
use protocol::chat::ChatComponent;
use server::{Server, ServerConfiguration};

pub mod connection;
pub mod server;

fn main() -> anyhow::Result<()> {
    // create config
    let motd = ChatComponent::builder()
        .text("A Lightweight and High-Performance Minecraft Server".to_string())
        .build();

    let config = ServerConfiguration::builder().motd(motd).build();

    // bind server (we handle connections inside the ECS)
    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port))?;
    listener.set_nonblocking(true)?;

    // setup ECS
    let mut world = World::new();

    world.insert_resource(config);
    world.insert_resource(Server { listener });

    let mut schedule = Schedule::default();

    // setup networking schedule stage
    #[derive(StageLabel)]
    pub struct Networking;

    schedule.add_stage(
        Networking,
        SystemStage::parallel()
            .with_system(server::accept_connections)
            .with_system(server::handle_connections),
    );

    loop {
        schedule.run(&mut world);
    }
}
