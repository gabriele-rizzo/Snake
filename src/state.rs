#[derive(PartialEq, Clone, Copy, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    Finished(bool),
    Terminated,
}
