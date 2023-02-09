use std::io::{Read, Write};

pub trait Encodable
where
    Self: Sized,
{
    fn decode<T: Read>(reader: &mut T) -> anyhow::Result<Self>;
    fn encode<T: Write>(&self, writer: &mut T) -> anyhow::Result<()>;
}

pub mod array;
pub mod chat;
pub mod numbers;
pub mod string;
