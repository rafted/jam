use std::{io::BufRead, net::TcpListener};

use crate::connection::{Connection, PacketContainer};
use anyhow::Result;
use bevy_ecs::{
    prelude::{EventReader, EventWriter},
    system::{Query, Resource},
};
use bytes::{Buf, BytesMut};
use crossbeam_channel::Sender;
use protocol::{chat::ChatComponent, encoding::Encodable, state::State, varint::VarInt};
use typed_builder::TypedBuilder;

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

/// Binds the server on a set host and port (given by a [ServerConfiguration].)
pub async fn bind<'a>(config: &ServerConfiguration<'a>) -> Result<TcpListener> {
    let host = config.host;
    let port = config.port;

    Ok(TcpListener::bind(format!("{}:{}", host, port))?)
}

/// Accepts connections from a [TcpListener].
pub async fn accept_loop(listener: TcpListener, sender: Sender<Connection>) -> Result<()> {
    loop {
        // accept connection (this is very blocking)
        let (stream, _) = listener.accept()?;

        // create component for the connection
        let connection = Connection {
            state: State::default(),
            stream,
            buf: vec![],
        };

        sender.send(connection)?;
    }
}

pub fn handle_connections(
    mut query: Query<&mut Connection>,
    mut writer: EventWriter<PacketContainer>,
) {
    for mut connection in &mut query {
        // this is currently blocking the thread. any idea?
        if let Err(_) = connection.read() {
            continue;
        }

        let buf = &connection.buf;
        let buf = &mut BytesMut::from(buf.as_slice());

        while !buf.is_empty() {
            // read packet frame
            let length = VarInt::decode(buf).expect("unable to decode length as VarInt");
            let id = VarInt::decode(buf).expect("unable to decode id as VarInt");

            let mut buffer = Vec::<u8>::new();

            buf.reader()
                .read_until(length.0 as u8, &mut buffer)
                .expect("unable to read packet");

            let container = PacketContainer {
                id,
                length,
                data: buffer,
            };

            writer.send(container);
        }
    }
}

pub fn handle_packet(mut event: EventReader<PacketContainer>) {
    for event in event.iter() {
        dbg!(event);
    }
}
