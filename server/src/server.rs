use protocol::{state::State, chat::ChatComponent};
use tokio::net::TcpListener;
use typed_builder::TypedBuilder;

use crate::connection::Connection;

#[derive(TypedBuilder)]
pub struct Server<'a> {
    #[builder(default = "0.0.0.0")]
    pub host: &'a str,

    #[builder(default = 25565)]
    pub port: u16,

    #[builder(default = 20)]
    pub max_players: i32,

    #[builder]
    pub motd: ChatComponent<'a>,
}

impl<'a> Server<'a> {
    pub async fn start(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;

        loop {
            let (stream, _) = listener.accept().await?;

            tokio::spawn(async move {
                let connection = Connection {
                    state: State::default(),
                    stream,
                };

                connection.handle_loop().await.expect("error handling");
            });
        }
    }
}
