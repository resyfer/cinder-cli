use uuid::Uuid;

use crate::player::Player;

pub fn players_from_ratings(ratings: &[u16; 5]) -> [Player; 5] {
    ratings.map(|rating| Player::new(Uuid::new_v4(), rating))
}
