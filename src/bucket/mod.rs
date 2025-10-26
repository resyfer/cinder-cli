pub mod bucket_diff;
pub mod bucket_list;

use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Bucket {
    low: u16,
    high: u16,
}

impl Debug for Bucket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bucket")
            .field("low", &self.low)
            .field("high", &self.high)
            .field("difference", &(&self.high - &self.low))
            .finish()
    }
}

impl Bucket {
    pub fn new(low: u16, high: u16) -> Bucket {
        Bucket { low, high }
    }

    pub fn high(&self) -> u16 {
        self.high
    }

    pub fn low(&self) -> u16 {
        self.low
    }
}
