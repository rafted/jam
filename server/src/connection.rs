use std::{
    io::{Read, Write},
    net::TcpStream,
};

use anyhow::{anyhow, Result};
use bevy_ecs::prelude::Component;
use bytes::BytesMut;
use protocol::{encoding::Encodable, state::State, varint::VarInt};

#[derive(Component)]
pub struct Connection {
    pub state: State,
    pub stream: TcpStream,
    pub buf: Vec<u8>,
}

#[derive(Component, Debug)]
pub struct PacketContainer {
    pub id: VarInt,
    pub length: VarInt,
    pub data: Vec<u8>,
}

impl Connection {
    pub fn buf_prep() -> BytesMut {
        BytesMut::new()
    }

    pub fn read(&mut self) -> Result<()> {
        match self.stream.read_to_end(&mut self.buf) {
            Ok(0) | Err(_) => Err(anyhow!("couldn't read")),
            Ok(_) => Ok(()),
        }
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
        self.stream.write(&len_buf)?;
        self.stream.write(&pkt_buf)?;

        Ok(())
    }

    //     pub async fn handle_loop(mut self) -> anyhow::Result<()> {
    //         let mut buf = BytesMut::new();
    //
    //         loop {
    //             let stream = &mut self.stream;
    //
    //             stream.read_buf(&mut buf)?;
    //
    //             while !buf.is_empty() {
    //                 // read packet frame
    //                 let _length = VarInt::decode(&mut buf)?;
    //                 let id = VarInt::decode(&mut buf)?;
    //
    //                 self.handle_packet(id.0, &mut buf)?;
    //             }
    //         }
    //     }
    //
    //
    //     pub async fn handle_packet(
    //         &mut self,
    //         id: i32,
    //         buf: &mut BytesMut,
    //     ) -> anyhow::Result<(), anyhow::Error> {
    //         match self.state {
    //             State::Handshaking => {
    //                 match id {
    //                     0 => {
    //                         // read handshake packet
    //                         let packet = HandshakePacket::decode(buf)?;
    //
    //                         // switch to next state
    //                         match packet.next_state.0 {
    //                             1 => self.state = State::Status,
    //                             2 => self.state = State::Login,
    //
    //                             _ => return Err(anyhow!("invalid next_state (must be either 1 or 2), are you reading the wrong packet?")),
    //                         }
    //                     }
    //
    //                     _ => println!("unimplemented packet: {}", id),
    //                 }
    //             }
    //             State::Status => {
    //                 match id {
    //                     0 => {
    //                         // read status request packet (and completely ignore it as it has no fields)
    //                         RequestPacket::decode(buf)?;
    //
    //                         // craft response packet
    //                         let response_pkt = ResponsePacket {
    //                             response: serde_json::json!({
    //                                 "version": {
    //                                     "name": "1.8.9",
    //                                     "protocol": 47
    //                                 },
    //                                 "players": {
    //                                     "max": 20,
    //                                     "online": 0,
    //                                 },
    //                                 "description":
    //                             })
    //                             .to_string(),
    //                         };
    //
    //                         // prepare buffer
    //                         let mut buf = Self::buf_prep();
    //
    //                         // write packet to buffer
    //                         Self::buf_write_id(&mut buf, 0)?;
    //                         response_pkt.encode(&mut buf)?;
    //
    //                         // send packet
    //                         self.buf_send(&mut buf)?;
    //                     }
    //
    //                     1 => {
    //                         let pkt = PingPacket::decode(buf)?;
    //
    //                         // craft response packet
    //                         let response_pkt = PongPacket {
    //                             payload: pkt.payload,
    //                         };
    //
    //                         // prepare buffer
    //                         let mut buf = Self::buf_prep();
    //
    //                         // write packet to buffer
    //                         Self::buf_write_id(&mut buf, 1)?;
    //                         response_pkt.encode(&mut buf)?;
    //
    //                         // send packet
    //                         self.buf_send(&mut buf)?;
    //
    //                         // close connection
    //                         self.stream.shutdown(std::net::Shutdown::Write)?;
    //                     }
    //
    //                     _ => println!("unimplemented packet: {}", id),
    //                 }
    //             }
    //             State::Login => println!("login state"),
    //             State::Play => println!("play state"),
    //             State::Closed => println!("closed state"),
    //         }
    //
    //         self.stream.flush()?;
    //         Ok(())
    //     }
}
