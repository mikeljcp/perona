use perona_types::constants::{POINT_DIFFERENCE, POINT_MULTIPLIER};



pub struct PlayerRating {
    pub killer: i32,
    pub dead: i32,
}

impl PlayerRating {
    pub fn new(killer: i32, dead: i32) -> Self {
        Self { killer, dead }
    }

    fn expected(&self) -> f64 {
        1.0 / (1.0 + 10.0_f64.powf((self.dead - self.killer) as f64 / POINT_DIFFERENCE as f64))
    }

    fn delta(&self) -> i32 {
        ((POINT_MULTIPLIER * (1.0 - self.expected())).floor()) as i32
    }

    pub fn get_pr(&self) -> (i32, i32) {
        let delta = self.delta();
        return ((self.killer + delta), (self.dead - delta));
    }
}
