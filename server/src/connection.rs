use std::io::Cursor;

use bytes::BytesMut;
use protocol::{state::State, varint::VarInt, encoding::Encodable};
use tokio::{net::TcpStream, io::{AsyncRead, AsyncReadExt, BufReader}};

pub struct Connection {
    pub stream: TcpStream,
    pub state: State,
}

impl Connection {
    pub async fn handle_loop(self) -> anyhow::Result<()> {
        let mut stream = self.stream;
        let mut buf = BytesMut::new();

        loop {
            let read = stream.read_buf(&mut buf).await?;
            println!("[");

            println!("read {} bytes", read);

            // read packet frame
            let length = VarInt::decode(&mut buf)?;
            let id = VarInt::decode(&mut buf)?;

            println!("length: {}", length.0);
            println!("id: {}", id.0);

            println!("]")

        }
    }
}
