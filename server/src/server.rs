use std::net::TcpListener;

use bevy_ecs::system::{Res, Resource, Commands, Query};
use protocol::{chat::ChatComponent, state::State};
use typed_builder::TypedBuilder;

use crate::connection::Connection;

#[derive(Resource)]
pub struct Server {
    pub listener: TcpListener,
}

#[derive(Resource, TypedBuilder)]
pub struct ServerConfiguration<'a> {
    #[builder(default = "0.0.0.0")]
    pub host: &'a str,

    #[builder(default = 25565)]
    pub port: u16,

    #[builder(default = 20)]
    pub max_players: i32,

    #[builder]
    pub motd: ChatComponent<'a>,
}

pub fn accept_connections(server: Res<Server>, mut commands: Commands) {
    let (stream, _) = server.listener.accept().expect("couldn't accept connection");
    let addr = stream.peer_addr().unwrap();

    println!("{}: opened", addr);

    // spawn entity for the connection
    commands.spawn(Connection {
        state: State::default(),
        stream
    });
}

pub fn handle_connections(query: Query<&Connection>) {
    for connection in &query {
        println!("handling connection {}", connection.stream.peer_addr().unwrap());
    }
}
