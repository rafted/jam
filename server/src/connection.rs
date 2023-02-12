use anyhow::anyhow;
use bytes::BytesMut;
use protocol::{
    encoding::Encodable, packet::serverbound::handshaking::HandshakePacket, state::State,
    varint::VarInt,
};
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

    pub fn handle_packet(
        &mut self,
        id: i32,
        buf: &mut BytesMut,
    ) -> anyhow::Result<(), anyhow::Error> {
        match self.state {
            State::Handshaking => {
                match id {
                    0 => {
                        // read handshake packet
                        let packet = HandshakePacket::decode(buf)?;

                        // switch to next state
                        match packet.next_state.0 {
                            1 => self.state = State::Status,
                            2 => self.state = State::Login,

                            _ => return Err(anyhow!("invalid next_state (must be either 1 or 2)")),
                        }
                    }

                    _ => todo!("implement packet"),
                }
            }
            State::Status => todo!("status state"),
            State::Login => todo!("login state"),
            State::Play => todo!("play state"),
            State::Closed => todo!("closed state"),
        }

        Ok(())
    }
}
