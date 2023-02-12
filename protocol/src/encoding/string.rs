use std::io::{Read, Write};

use bytes::{Buf, BufMut};

use crate::varint::VarInt;

use super::Encodable;

impl Encodable for String {
    fn decode(reader: &mut dyn Buf) -> anyhow::Result<Self> {
        let string_len = VarInt::decode(reader)?;

        let mut buf = Vec::with_capacity(string_len.0 as usize);
        buf.resize(string_len.0 as usize, 0);

        reader.reader().read(&mut buf[..])?;

        Ok(String::from_utf8(buf)?)
    }

    fn encode(&self, writer: &mut dyn BufMut) -> anyhow::Result<()> {
        let string_len = self.len();

        VarInt::encode(&VarInt(string_len as i32), writer)?;
        writer.writer().write(String::as_bytes(self))?;

        Ok(())
    }
}
