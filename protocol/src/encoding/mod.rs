use bytes::{Buf, BufMut};

pub trait Encodable
where
    Self: Sized,
{
    fn decode(reader: &mut dyn Buf) -> anyhow::Result<Self>;
    fn encode(&self, writer: &mut dyn BufMut) -> anyhow::Result<()>;
}

pub mod array;
pub mod chat;
pub mod numbers;
pub mod string;
