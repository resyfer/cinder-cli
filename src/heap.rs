use std::cmp::Ordering;

use ordered_float::OrderedFloat;

use crate::player::Player;

#[derive(Debug, Clone, Copy)]
pub struct HeapItem {
    sanction: OrderedFloat<f64>,
    _lobby: [Player; 5],
}

impl PartialEq for HeapItem {
    fn eq(&self, other: &Self) -> bool {
        self.sanction == other.sanction
    }
}

impl Eq for HeapItem {}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sanction.cmp(&other.sanction)
    }
}

impl HeapItem {
    pub fn new(score: f64, lobby: [Player; 5]) -> HeapItem {
        HeapItem {
            sanction: OrderedFloat(score),
            _lobby: lobby,
        }
    }

    pub fn score(&self) -> f64 {
        self.sanction.into_inner()
    }

    pub fn _lobby(&self) -> &[Player; 5] {
        &self._lobby
    }
}
