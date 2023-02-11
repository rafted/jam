use bytes::BytesMut;
use protocol::{encoding::Encodable, state::State, varint::VarInt};
use tokio::{
    io::{AsyncReadExt, Interest},
    net::TcpStream,
};

pub struct Connection {
    pub state: State,
}

impl Connection {
    pub async fn handle_loop(self, mut stream: TcpStream) -> anyhow::Result<()> {
        let mut buf = BytesMut::new();

        loop {
            let ready = stream.ready(Interest::READABLE).await?;

            if !ready.is_readable() {
                continue;
            }

            let read = stream.read_buf(&mut buf).await?;
            println!("read {} bytes", read);

            // read packet frame
            let _length = VarInt::decode(&mut buf)?;
            let id = VarInt::decode(&mut buf)?;

            self.handle_packet(id.0, &mut buf);
        }
    }

    pub fn handle_packet(&self, _id: i32, _buf: &mut BytesMut) -> anyhow::Result<()> {
        todo!()
    }
}
