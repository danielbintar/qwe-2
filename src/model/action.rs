pub enum PlayerAction {
    LeaveTown
}

pub struct Action {
    pub action: Option<PlayerAction>,
}

impl Default for Action {
    fn default() -> Self {
        Self {
            action: None
        }
    }
}
