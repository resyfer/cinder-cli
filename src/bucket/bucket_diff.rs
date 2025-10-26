use std::fmt::Debug;

use log::debug;

#[derive(Clone, Copy)]
pub struct PlayerBucketDiff {
    total: u16,
    index: usize,
    bucket_edges: [u16; 15],
}

impl Debug for PlayerBucketDiff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Player Bucket Diff")
            .field("total", &self.total)
            .field("index", &self.index)
            .finish()
    }
}

impl PlayerBucketDiff {
    pub fn new(total: u16) -> Self {
        let base: [u16; 15] = std::array::from_fn(|i| (i + 1) as u16);
        let base_total_sum: u16 = base.iter().sum();

        let mut int_sum = 0u16;
        let mut bucket_edges = [0u16; 15];
        for i in 0..15 {
            bucket_edges[i] =
                ((total as f64) * (base[i] as f64) / (base_total_sum as f64)).round() as u16;
            int_sum += bucket_edges[i];
        }

        debug!("Bucket edges: {:?}", bucket_edges);

        // Step 3: Fix rounding error
        let mut diff = int_sum as i32 - total as i32;
        if diff != 0 {
            let last = &mut bucket_edges[14];
            if *last as i32 - diff >= 0 {
                *last = (*last as i32 - diff) as u16;
            } else {
                // Distribute error backwards
                diff -= *last as i32;
                *last = 0;

                for i in (0..14).rev() {
                    if diff == 0 {
                        break;
                    }
                    let take = std::cmp::min(diff, bucket_edges[i] as i32);
                    bucket_edges[i] -= take as u16;
                    diff -= take;
                }
            }
        }

        Self {
            total,
            index: 0,
            bucket_edges,
        }
    }
}

impl Iterator for PlayerBucketDiff {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 15 {
            return None;
        }

        let value = self.bucket_edges[self.index];
        self.index += 1;

        Some(value)
    }
}
