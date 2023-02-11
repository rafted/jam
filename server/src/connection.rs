use std::io::{Cursor, BufReader};

use protocol::{state::State, varint::VarInt, encoding::Encodable};
use tokio::{net::TcpStream, io::{AsyncRead, AsyncReadExt}};

pub struct Connection {
    pub stream: TcpStream,
    pub state: State,
}

impl Connection {
    pub async fn handle_loop(self) -> anyhow::Result<()> {
        let mut stream = self.stream;
        let mut reader = BufReader::new(stream);
        // let mut cursor = Cursor::new(reader);

        loop {
            // read packet frame
            let id = VarInt::decode(&mut reader)?;

            println!("id: {}", id.0);

        }
    }
}
