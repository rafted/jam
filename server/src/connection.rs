use protocol::state::State;
use tokio::net::TcpStream;

pub struct Connection {
    pub stream: TcpStream,
    pub state: State,
}

impl Connection {
    pub fn handle_loop(self) {
        let stream = self.stream;

        loop {}
    }
}
