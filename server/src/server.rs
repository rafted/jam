use protocol::state::State;
use tokio::net::TcpListener;
use typed_builder::TypedBuilder;

use crate::connection::Connection;

#[derive(TypedBuilder, PartialEq)]
pub struct Server<'a> {
    #[builder(default = "0.0.0.0")]
    host: &'a str,
    #[builder(default = 25565)]
    port: u16,
    #[builder(default = 20)]
    max_players: i32,
}

impl<'a> Server<'a> {
    pub async fn start(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind("0.0.0.0:25565").await?;

        loop {
            let (stream, _) = listener.accept().await?;

            tokio::spawn(async move {
                let connection = Connection {
                    stream,
                    state: State::default(),
                };

                connection.handle_loop();
            });
        }
    }
}
