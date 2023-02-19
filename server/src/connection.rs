use anyhow::{anyhow, Result};
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
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub struct Connection {
    pub state: State,
    pub stream: TcpStream,
}

impl Connection {
    pub async fn handle_loop(mut self) -> anyhow::Result<()> {
        let mut buf = BytesMut::new();

        loop {
            let stream = &mut self.stream;

            match stream.try_read_buf(&mut buf) {
                Ok(0) => {}
                Ok(_read) => {}
                _ => (),
            };

            while !buf.is_empty() {
                // read packet frame
                let _length = VarInt::decode(&mut buf)?;
                let id = VarInt::decode(&mut buf)?;

                self.handle_packet(id.0, &mut buf).await?;
            }
        }
    }

    pub fn buf_prep() -> BytesMut {
        BytesMut::new()
    }

    pub fn buf_write_id(buf: &mut BytesMut, id: i32) -> Result<()> {
        VarInt(id).encode(buf)
    }

    pub async fn buf_send(&mut self, pkt_buf: &mut BytesMut) -> Result<()> {
        // create a buffer for the length of the packet (sent first)
        let mut len_buf = BytesMut::new();

        // write length of the packet to the length buffer
        VarInt(pkt_buf.len() as i32).encode(&mut len_buf)?;

        // write length first, then the packet
        self.stream.write(&len_buf).await?;
        self.stream.write(&pkt_buf).await?;

        Ok(())
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

                            _ => return Err(anyhow!("invalid next_state (must be either 1 or 2), are you reading the wrong packet?")),
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

                        // prepare buffer
                        let mut buf = Self::buf_prep();

                        // write packet to buffer
                        Self::buf_write_id(&mut buf, 0)?;
                        packet.encode(&mut buf)?;

                        self.buf_send(&mut buf).await?;
                    }

                    _ => println!("unimplemented packet: {}", id),
                }
            }
            State::Login => println!("login state"),
            State::Play => println!("play state"),
            State::Closed => println!("closed state"),
        }

        println!("processed packet");
        self.stream.flush().await?;
        Ok(())
    }
}
