use serde_derive::Deserialize;

#[derive(Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Place {
    Region,
    Town,
    Battle
}

pub struct CurrentPlace {
    pub place: Option<Place>,
}

impl Default for CurrentPlace {
    fn default() -> Self {
        Self {
            place: None
        }
    }
}
