use std::io::{Write};

use byteorder::{BigEndian, ReadBytesExt};
use bytes::{Buf, BufMut};

use super::Encodable;

impl Encodable for i8 {
    fn decode(reader: &mut dyn Buf) -> anyhow::Result<Self> {
        let val = reader.reader().read_i8()?;
        Ok(val)
    }

    fn encode(&self, writer: &mut dyn BufMut) -> anyhow::Result<()> {
        writer.writer().write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

impl Encodable for u8 {
    fn decode(reader: &mut dyn Buf) -> anyhow::Result<Self> {
        let val = reader.reader().read_u8()?;
        Ok(val)
    }

    fn encode(&self, writer: &mut dyn BufMut) -> anyhow::Result<()> {
        writer.writer().write_all(&self.to_be_bytes())?;
        Ok(())
    }
}

macro_rules! gen_num_encode {
    ($ty:ty) => {
        ::paste::paste! {
            impl Encodable for $ty {
                fn decode(reader: &mut dyn Buf) -> anyhow::Result<Self> {
                    let val = reader.reader().[<read_ $ty>]::<BigEndian>()?;
                    Ok(val)
                }

                fn encode(&self, writer: &mut dyn BufMut) -> anyhow::Result<()> {
                    writer.writer().write_all(&self.to_be_bytes())?;
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
