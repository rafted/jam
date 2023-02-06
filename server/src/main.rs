use server::Server;

pub mod connection;
pub mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Server::builder().build().start().await
}
