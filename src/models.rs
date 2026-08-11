use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Player {
    pub player_id: String,
    pub last_name: String,
    pub first_name: String,
    pub team: String,
    pub number: i32,
    pub position: String,
}

#[derive(Deserialize)]
pub struct WaitlistPlayer {
    pub player_id: String,
}
