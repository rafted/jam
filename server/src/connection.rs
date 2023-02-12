use anyhow::anyhow;
use bytes::BytesMut;
use protocol::{
    encoding::Encodable,
    packet::{
        clientbound::status::ResponsePacket,
        serverbound::{handshaking::HandshakePacket, status::RequestPacket},
    },
    state::State,
    varint::VarInt,
};
use tokio::{
    io::{AsyncReadExt, Interest},
    net::TcpStream,
};

pub struct Connection {
    pub state: State,
    pub stream: TcpStream,
}

impl Connection {
    pub async fn handle_loop(mut self) -> anyhow::Result<()> {
        let mut buf = BytesMut::new();

        loop {
            let stream = &mut self.stream;
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
            State::Status => {
                match id {
                    0 => {
                        // read status request packet (and completely ignore it as it has no fields)
                        RequestPacket::decode(buf)?;

                        // craft response packet
                        let packet = ResponsePacket {
                            response: serde_json::json!({
                                "version": {
                                    "name": "1.8.9",
                                    "protocol": 47
                                },
                                "players": {
                                    "max": 20,
                                    "online": 0,
                                },
                                "description": {
                                    "text": "A Lightweight and High Performant Server"
                                },
                            })
                            .to_string(),
                        };

                        todo!("send response")
                    }

                    _ => todo!("implement packet"),
                }
            }
            State::Login => todo!("login state"),
            State::Play => todo!("play state"),
            State::Closed => todo!("closed state"),
        }

        Ok(())
    }
}
