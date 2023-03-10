use crate::encoding::Encodable;
use byteorder::{BigEndian, ReadBytesExt};
use bytes::{Buf, BufMut};
use std::{io::Write, num::TryFromIntError};

pub type Angle = f32;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

impl Encodable for Position {
    fn decode(reader: &mut dyn Buf) -> anyhow::Result<Self> {
        let value = reader.reader().read_i64::<BigEndian>()?;

        // we technically don't need this function here, but it's easier like this so we can handle the error mapping in a single call - instead of doing it for every single try_into() statement,
        // we also could implement From<TryFromIntError> into std::io::Error, but that's a lot of work, and I think this works just as fine.
        fn calc_x_y_z(value: i64) -> Result<(i32, i16, i32), TryFromIntError> {
            let x: i32 = (value >> 38).try_into()?;
            let y: i16 = (value << 52 >> 52).try_into()?;
            let z: i32 = (value << 26 >> 38).try_into()?;

            Ok((x, y, z))
        }

        let (x, y, z) = calc_x_y_z(value)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        Ok(Position { x, y, z })
    }

    fn encode(&self, writer: &mut dyn BufMut) -> anyhow::Result<()> {
        let value: i64 = (((self.x & 0x3FFFFFF) as i64) << 38) as i64
            | ((self.z & 0x3FFFFFF) << 12) as i64
            | (self.y as i32 & 0xFFF) as i64;

        let buf = value.to_be_bytes();

        writer.writer().write_all(&buf)?;
        Ok(())
    }
}
