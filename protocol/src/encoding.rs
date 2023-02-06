use crate::array::CountedArray;
use crate::chat::ChatComponent;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Read, Write};

use crate::varint::VarInt;

pub trait Encodable
where
    Self: Sized,
{
    fn decode<T: Read>(reader: &mut T) -> anyhow::Result<Self>;
    fn encode<T: Write>(&self, writer: &mut T) -> anyhow::Result<()>;
}

// move this into separate files
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

impl<'a> Encodable for ChatComponent<'a> {
    fn decode<T: Read>(reader: &mut T) -> anyhow::Result<Self> {
        let encoded = String::decode(reader)?;
        let component = serde_json::from_str(&encoded)?;

        return Ok(component);
    }

    fn encode<T: Write>(&self, writer: &mut T) -> anyhow::Result<()> {
        let encoded = serde_json::to_string(self)?;

        String::encode(&encoded, writer)
    }
}

impl<K, U> Encodable for CountedArray<K, U>
where
    usize: PartialEq<K>,
    K: Encodable + Into<usize> + From<usize>,
    U: Encodable,
{
    fn decode<T: Read>(reader: &mut T) -> anyhow::Result<Self> {
        let mut vec = Vec::<U>::new();
        let len: usize = K::decode(reader)?.into();

        for _ in 0..len {
            vec.push(U::decode(reader)?);
        }

        Ok(CountedArray {
            len: len.into(),
            arr: vec,
        })
    }

    fn encode<T: Write>(&self, writer: &mut T) -> anyhow::Result<()> {
        assert!(self.arr.len() == self.len);
        self.len.encode(writer)?;

        for el in self.arr.iter() {
            el.encode(writer)?;
        }

        Ok(())
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
