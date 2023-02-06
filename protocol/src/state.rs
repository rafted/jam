#[derive(Debug, Clone, Copy, Default)]
pub enum State {
    #[default]
    Handshaking,
    Login,
    Play,
    Closed,
}
