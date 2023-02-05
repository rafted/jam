#[derive(Debug, Clone, Copy, Default)]
pub enum State {
    #[default]
    Handshake,
    Login,
    Play,
    Closed,
}
