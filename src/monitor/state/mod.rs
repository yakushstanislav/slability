#[derive(Clone, Copy)]
pub struct State {
    is_online: Option<bool>,
}

impl State {
    pub fn new() -> Self {
        State { is_online: None }
    }

    pub fn set_is_online(&mut self, is_online: bool) {
        self.is_online = Some(is_online)
    }

    pub fn is_online(&self) -> Option<bool> {
        self.is_online
    }
}
