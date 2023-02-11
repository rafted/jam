use bytes::BytesMut;
use protocol::{encoding::Encodable, state::State, varint::VarInt, packet::serverbound::handshaking::HandshakePacket};
use tokio::{
    io::{AsyncReadExt, Interest},
    net::TcpStream,
};

pub struct Connection {
    pub state: State,
}

impl Connection {
    pub async fn handle_loop(mut self, mut stream: TcpStream) -> anyhow::Result<()> {
        let mut buf = BytesMut::new();

        loop {
            let ready = stream.ready(Interest::READABLE).await?;

            if !ready.is_readable() {
                continue;
            }

            let read = stream.read_buf(&mut buf).await?;
            println!("read {} bytes", read);

            // read packet frame
            let _length = VarInt::decode(&mut buf)?;
            let id = VarInt::decode(&mut buf)?;

            self.handle_packet(id.0, &mut buf)?;
        }
    }

    pub fn handle_packet(&mut self, id: i32, buf: &mut BytesMut) -> anyhow::Result<()> {

        match self.state {
            State::Handshaking => {

                match id {
                    0 => {
                        let packet = HandshakePacket::decode(buf)?;

                        println!("next_state: {}", packet.next_state.0);
                        println!("port: {}", packet.server_port);
                        println!("address: {}", packet.server_address);
                        println!("protocol: {}", packet.protocol_version.0);
                    }

                    _ => todo!("implement packet")
                }

            },
            State::Status => todo!("status state"),
            State::Login => todo!("login state"),
            State::Play => todo!("play state"),
            State::Closed => todo!("closed state"),
        }

        Ok(())
    }
}
