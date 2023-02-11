use bytes::{Buf, BufMut};

use crate::chat::ChatComponent;

use super::Encodable;

impl<'a> Encodable for ChatComponent<'a> {
    fn decode(reader: &mut dyn Buf) -> anyhow::Result<Self> {
        let encoded = String::decode(reader)?;
        let component = serde_json::from_str(&encoded)?;

        return Ok(component);
    }

    fn encode(&self, writer: &mut dyn BufMut) -> anyhow::Result<()> {
        let encoded = serde_json::to_string(self)?;

        String::encode(&encoded, writer)
    }
}
