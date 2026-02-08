#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub level: i64,
    pub guild: String,
    pub last_kill: Option<String>,
    pub kills_count:  Option<i32>,
    pub in_md3: bool,
    pub elo: Option<String>,
    pub division:  Option<i32>,
    pub player_rating: i64,
}

#[derive(Debug, Clone)]
pub struct BestOf {
    pub name: String,
    pub wins: i64,
    pub loses: i64,
}

#[derive(Debug, Clone)]
pub struct Kill {
    pub killed: String,
    pub player_rating_gain: i64,
}

#[derive(Debug, Clone)]
pub struct Death {
    pub kb: String,
    pub player_rating_lost: i64,
}