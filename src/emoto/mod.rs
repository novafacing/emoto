use self::state::EmotoState;

pub mod state;
pub mod ui;

pub struct Emoto {
    pub state: EmotoState,
}

impl Emoto {
    pub fn new() -> Self {
        let state = EmotoState::default();
        Self { state }
    }

    pub fn state(&self) -> &EmotoState {
        &self.state
    }
}
