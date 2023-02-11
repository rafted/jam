use std::io::Cursor;

use bytes::BytesMut;
use protocol::{state::State, varint::VarInt, encoding::Encodable};
use tokio::{net::TcpStream, io::{AsyncRead, AsyncReadExt, BufReader, Interest}};

pub struct Connection {
    pub stream: TcpStream,
    pub state: State,
}

impl Connection {
    pub async fn handle_loop(self) -> anyhow::Result<()> {
        let mut stream = self.stream;
        let mut buf = BytesMut::new();

        loop {
            let ready = stream.ready(Interest::READABLE).await?;

            if !ready.is_readable() {
                continue;
            }

            let read = stream.read_buf(&mut buf).await?;
            println!("read {} bytes", read);

            // read packet frame
            let length = VarInt::decode(&mut buf)?;
            let id = VarInt::decode(&mut buf)?;

            println!("read: {}, length: {}", read, length.0);
        }
    }
}
