use std::io::{Read, Write};

use crate::encoding::Encodable;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VarInt(pub i32);

impl Into<usize> for VarInt {
    fn into(self) -> usize {
        self.0 as usize
    }
}

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
