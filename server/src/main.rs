use connection::Connection;
use tokio::net::TcpListener;

pub mod connection;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

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