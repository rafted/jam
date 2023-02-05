use std::{
    any::Any,
    io::{Read, Write},
};

use crate::encoding::Encodable;

pub mod clientbound;
pub mod serverbound;

pub enum PacketDirection {
    Clientbound,
    Serverbound,
}

pub trait Packet: Any + Encodable
where
    Self: Sized,
{
    fn decode<T: Read>(reader: &mut T) -> anyhow::Result<Self>;
    fn encode<T: Write>(&self, writer: &mut T) -> anyhow::Result<()>;
}

impl<P: Packet> Encodable for P {
    fn decode<T: Read>(reader: &mut T) -> anyhow::Result<Self> {
        <Self as Packet>::decode(reader)
    }

    fn encode<T: Write>(&self, writer: &mut T) -> anyhow::Result<()> {
        <Self as Packet>::encode(self, writer)
    }
}
