use std::io::{Read, Write};

use crate::chat::ChatComponent;

use super::Encodable;

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
