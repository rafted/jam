use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Read, Write};
use crate::chat::ChatComponent;

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
        let string_len = VarInt::decode(reader)?;

        let mut buf = Vec::with_capacity(string_len.0 as usize);
        buf.resize(string_len.0 as usize, 0);

        reader.read(&mut buf[..])?;

        Ok(String::from_utf8(buf)?)
    }

    fn encode<T: Write>(&self, writer: &mut T) -> anyhow::Result<()> {
        let string_len = self.len();

        VarInt::encode(&VarInt(string_len as i32), writer)?;
        writer.write(String::as_bytes(self))?;

        Ok(())
    }
}

impl Encodable for ChatComponent {
    fn decode<T: Read>(reader: &mut T) -> anyhow::Result<Self> {
        let encoded = String::decode(reader)?;

        return serde_json::from_str(&encoded)?
    }

    fn encode<T: Write>(&self, writer: &mut T) -> anyhow::Result<()> {
        let encoded = serde_json::to_string(self)?;

        String::encode(&encoded, writer)
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

impl Encodable for i8 {
    fn decode<T: Read>(reader: &mut T) -> anyhow::Result<Self> {
        let val = reader.read_i8()?;
        Ok(val)
    }

    fn encode<T: Write>(&self, writer: &mut T) -> anyhow::Result<()> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Encodable for u8 {
    fn decode<T: Read>(reader: &mut T) -> anyhow::Result<Self> {
        let val = reader.read_u8()?;
        Ok(val)
    }

    fn encode<T: Write>(&self, writer: &mut T) -> anyhow::Result<()> {
        writer.write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

encde_num!(i16);
encde_num!(i32);
encde_num!(i64);
encde_num!(u16);
encde_num!(u32);
encde_num!(u64);
encde_num!(f32);
encde_num!(f64);
