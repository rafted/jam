use connection::Connection;
use tokio::net::TcpListener;

pub mod connection;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:25565").await?;

    loop {
        let (mut stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            let connection = Connection {
                stream
            };

            connection.handle_loop();
        });
    }
}