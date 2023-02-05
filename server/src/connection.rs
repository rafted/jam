use tokio::net::TcpStream;

pub struct Connection {
    pub stream: TcpStream
}

impl Connection {
    pub fn handle_loop(self) {
        let stream = self.stream;

        loop {
            // pray to the lord!!
        }
    }
}