use crate::states::PlayerAction;

#[derive(Clone)]
pub struct Game {
    pub player_action: Option<PlayerAction>
}

impl Default for Game {
    fn default() -> Self {
        Self {
            player_action: None
        }
    }
}
