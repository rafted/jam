use std::io::{Read, Write};

use crate::varint::VarInt;

use super::Encodable;

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
