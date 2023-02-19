use protocol::chat::ChatComponent;
use server::Server;

pub mod connection;
pub mod server;
pub mod status;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let motd = ChatComponent::builder()
        .text("A Lightweight and High-Performance Minecraft Server".to_string())
        .build();

    Server::builder()
        .motd(motd)
        .build()
        .start().await
}
