use connection::Connection;
use protocol::state::State;
use server::Server;
use tokio::net::TcpListener;

pub mod connection;
pub mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Server::builder().build().start().await
}
