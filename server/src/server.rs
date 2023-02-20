use std::net::TcpListener;

use bevy_ecs::system::{Commands, Query, Res, Resource};
use bytes::Buf;
use protocol::{chat::ChatComponent, encoding::Encodable, state::State, varint::VarInt};
use typed_builder::TypedBuilder;

use crate::connection::{Connection, PacketContainer};

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
    for stream in server.listener.incoming() {
        match stream {
            Ok(stream) => {
                let addr = stream.peer_addr().unwrap();

                println!("{}: opened", addr);

                // spawn entity for the connection
                commands.spawn(Connection {
                    state: State::default(),
                    buf: Connection::buf_prep(),
                    stream,
                });
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                break;
            }
            Err(e) => panic!("encountered IO error: {e}"),
        }
    }
}

pub fn handle_connections(mut commands: Commands, mut query: Query<&mut Connection>) {
    for mut connection in &mut query {
        // connection
        //     .read()
        //     .expect("unable to read buffer from connection");

        // let buf = &mut connection.buf;

        // while !buf.is_empty() {
        //     // read packet frame
        //     let length = VarInt::decode(buf).expect("unable to decode length as VarInt");
        //     let id = VarInt::decode(buf).expect("unable to decode id as VarInt");

        //     // self.handle_packet(id.0, &mut buf)?;
        //     println!("== packet id: {}", id.0);
        //     println!("== packet length: {}", length.0);

        //     // let container = PacketContainer {
        //     //     id,
        //     //     length,
        //     //     data: buf,
        //     //     // data: buf.take(length.0.try_into().unwrap()).chunk(),
        //     // };

        //     // commands.spawn(container);
        // }

        // let empty = &buf.is_empty();

        println!(
            "handling connection {}",
            connection.stream.peer_addr().unwrap(),
            // empty
        );
    }
}
