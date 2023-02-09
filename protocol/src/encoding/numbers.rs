use std::io::{Read, Write};

use byteorder::{BigEndian, ReadBytesExt};

use super::Encodable;

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

macro_rules! gen_num_encode {
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

gen_num_encode!(i16);
gen_num_encode!(i32);
gen_num_encode!(i64);
gen_num_encode!(u16);
gen_num_encode!(u32);
gen_num_encode!(u64);
gen_num_encode!(f32);
gen_num_encode!(f64);
