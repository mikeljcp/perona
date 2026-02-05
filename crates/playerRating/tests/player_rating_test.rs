use player_rating::PlayerRating;

#[cfg(test)]
mod player_rating_test {
    use super::*;

    fn get_player_rating() -> (i32, i32) {
        PlayerRating::new(1000, 1000).get_pr()
    }

    #[test]
    fn test_killer_pr() {
        assert_eq!(get_player_rating().0, 1008)
    }

    #[test]
    fn test_dead_pr() {
        assert_eq!(get_player_rating().1, 992)
    }
}