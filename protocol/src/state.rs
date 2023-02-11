#[derive(Debug, Clone, Copy, Default)]
pub enum State {
    #[default]
    Handshaking,
    Status,
    Login,
    Play,
    Closed,
}
