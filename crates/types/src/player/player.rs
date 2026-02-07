#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub level: i64,
    pub guild: Option<String>,
    pub last_kill: Option<i32>,
    pub killst_count:  Option<i32>,
    pub in_md3: bool,
    pub elo: Option<String>,
    pub division:  Option<i32>,
    pub player_rating: i64,
}

#[derive(Debug)]
pub struct BestOf {
    pub name: String,
    pub wins: i64,
    pub loses: i64,
}

#[derive(Debug)]
pub struct Kill {
    pub killed: String,
    pub player_rating_gain: i64,
}

#[derive(Debug)]
pub struct Death {
    pub kb: String,
    pub player_rating_lost: i64,
}
