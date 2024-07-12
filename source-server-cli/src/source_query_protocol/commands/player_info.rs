use chrono::TimeDelta;

pub struct PlayerInfo {
    name: String,
    score: u64,
    duration: chrono::TimeDelta,
}

pub struct PlayersInfo {
    players: Vec<PlayerInfo>,
}
