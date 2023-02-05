use byteorder::{BigEndian, ReadBytesExt};
use std::{
    fs::read,
    io::{Read, Write},
};

use crate::varint::VarInt;

pub trait Encodable
where
    Self: Sized,
{
    fn decode<T: Read>(reader: &mut T) -> anyhow::Result<Self>;
    fn encode<T: Write>(&self, writer: &mut T) -> anyhow::Result<()>;
}

// move this into separate files
impl Encodable for VarInt {
    fn decode<T: Read>(reader: &mut T) -> anyhow::Result<Self> {
        let mut result = 0;
        let mut shift = 0;

        loop {
            let mut byte = [0];

            reader.read_exact(&mut byte)?;

            let value = (byte[0] & 0b01111111) as i32;

            result |= value << shift;
            shift += 7;

            if byte[0] & 0b10000000 == 0 {
                break;
            }
        }

        Ok(VarInt(result))
    }

    fn encode<T: Write>(&self, writer: &mut T) -> anyhow::Result<()> {
        let mut remaining = self.0;
        while remaining >= 0b10000000 {
            let byte = (remaining as u8) | 0b10000000;

            writer.write_all(&[byte])?;
            remaining >>= 7;
        }
        let byte = remaining as u8;

        writer.write_all(&[byte])?;
        Ok(())
    }
}

impl Encodable for String {
    fn decode<T: Read>(reader: &mut T) -> anyhow::Result<Self> {
        todo!()
    }

    fn encode<T: Write>(&self, writer: &mut T) -> anyhow::Result<()> {
        todo!()

    }
}

macro_rules! encde_num {
    ($ty:ty) => {
        ::paste::paste! {
            impl Encodable for $ty {
                fn decode<T: Read>(reader: &mut T) -> anyhow::Result<Self> {
                    let val = reader.[<read_ $ty>]::<BigEndian>()?;
                    Ok(val)
                }

                fn encode<T: Write>(&self, writer: &mut T) -> anyhow::Result<()> {
                    writer.write_all(&self.to_be_bytes())?;
                    Ok(())
                }
            }
        }
    };
}

encde_num!(i16);
encde_num!(i32);
encde_num!(i64);
encde_num!(u16);
encde_num!(u32);
encde_num!(u64);
encde_num!(f32);
encde_num!(f64);
