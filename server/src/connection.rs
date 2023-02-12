use anyhow::anyhow;
use bytes::{BufMut, BytesMut};
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
    io::{AsyncReadExt, AsyncWriteExt, Interest},
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
            let length = VarInt::decode(&mut buf)?;
            let id = VarInt::decode(&mut buf)?;

            println!("packet length: {:#06x}", length.0);
            println!("packet id: {:#06x}", id.0);

            self.handle_packet(id.0, &mut buf).await?;
        }
    }

    pub async fn handle_packet(
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

                    _ => println!("unimplemented packet: {}", id),
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

                        let mut bytes = BytesMut::new();
                        packet.encode(&mut bytes)?;

                        self.stream.write(&bytes).await?;
                    }

                    _ => println!("unimplemented packet: {}", id),
                }
            }
            State::Login => println!("login state"),
            State::Play => println!("play state"),
            State::Closed => println!("closed state"),
        }

        self.stream.flush().await?;
        Ok(())
    }
}
