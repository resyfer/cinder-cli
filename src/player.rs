use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct Player {
    _id: Uuid,
    rating: u16,
}

impl Player {
    pub fn new(id: Uuid, rating: u16) -> Player {
        Player { _id: id, rating }
    }

    pub fn rating(&self) -> u16 {
        self.rating
    }

    pub fn _id(&self) -> Uuid {
        self._id
    }
}
