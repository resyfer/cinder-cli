use std::fmt::Debug;

use crate::{bucket::Bucket, diff::Diff, player::Player};

#[derive(Debug)]
pub struct BucketList {
    diff: Diff,
    mean: u16,
    buckets: Option<[Bucket; 30]>,
}

impl BucketList {
    pub fn new(diff: Diff, mean: u16) -> BucketList {
        BucketList {
            diff,
            mean,
            buckets: None,
        }
    }

    pub fn buckets(&mut self) -> &[Bucket] {
        if let None = self.buckets {
            self.prepare_buckets();
        }

        self.buckets.as_ref().unwrap()
    }

    fn prepare_buckets(&mut self) {
        let mut buckets = [Bucket::new(0, 0); 30];

        let mut dist_from_centre: u16 = 0;
        let mut prev_low: u16 = self.mean;
        let mut prev_high: u16 = self.mean;

        for val in &mut self.diff {
            buckets[(14 - dist_from_centre) as usize] = Bucket::new(prev_low - val, prev_low);
            prev_low -= val;

            buckets[(15 + dist_from_centre) as usize] = Bucket::new(prev_high, prev_high + val);
            prev_high += val;

            dist_from_centre += 1;
        }

        self.buckets = Some(buckets);
    }

    pub fn get_index(&mut self, rating: u16) -> usize {
        if let None = self.buckets {
            self.prepare_buckets();
        }

        assert!(self.buckets().len() != 0);

        for (i, b) in self.buckets().iter().enumerate() {
            if b.low() <= rating && rating <= b.high() {
                return i;
            }
        }

        self.buckets().len()
    }

    pub fn bucket_diff(&mut self, lobby1: [Player; 5], lobby2: [Player; 5]) -> f64 {
        let lobby1_bucket_idxs: Vec<usize> = lobby1
            .iter()
            .map(|&player| self.get_index(player.rating()))
            .collect();

        let lobby2_bucket_idxs: Vec<usize> = lobby2
            .iter()
            .map(|&player| self.get_index(player.rating()))
            .collect();

        // println!("Lobby 1 Bucket Index: {:#?}", lobby1_bucket_idxs);
        // println!("Lobby 2 Bucket Index: {:#?}", lobby2_bucket_idxs);

        let mut available: Vec<usize> = (0..lobby1_bucket_idxs.len()).collect();
        let mut pair_bucket_diffs: Vec<f64> = Vec::new();

        for (i, &lobby1_bucket_idx) in lobby1_bucket_idxs.iter().enumerate() {
            // Finding minimum bucket difference
            let mut min_bucket_diff = usize::MAX;
            for &idx in &available {
                let diff = if lobby2_bucket_idxs[idx] > lobby1_bucket_idx {
                    lobby2_bucket_idxs[idx] - lobby1_bucket_idx
                } else {
                    lobby1_bucket_idx - lobby2_bucket_idxs[idx]
                };
                if diff < min_bucket_diff {
                    min_bucket_diff = diff;
                }
            }

            // Candidates are indices in available with min bucket difference

            let candidates: Vec<usize> = available
                .iter()
                .copied()
                .filter(|&idx| {
                    let diff = if lobby2_bucket_idxs[idx] > lobby1_bucket_idx {
                        lobby2_bucket_idxs[idx] - lobby1_bucket_idx
                    } else {
                        lobby1_bucket_idx - lobby2_bucket_idxs[idx]
                    };
                    diff == min_bucket_diff
                })
                .collect();

            // Select best candidate by numeric distance if multiple
            let best_j = if candidates.len() == 1 {
                candidates[0]
            } else {
                candidates
                    .into_iter()
                    .min_by(|&a, &b| {
                        let dist_a = if lobby2_bucket_idxs[a] > lobby1_bucket_idxs[i] {
                            lobby2_bucket_idxs[a] - lobby1_bucket_idxs[i]
                        } else {
                            lobby1_bucket_idxs[i] - lobby2_bucket_idxs[a]
                        };

                        let dist_b = if lobby2_bucket_idxs[b] > lobby1_bucket_idxs[i] {
                            lobby2_bucket_idxs[b] - lobby1_bucket_idxs[i]
                        } else {
                            lobby1_bucket_idxs[i] - lobby2_bucket_idxs[b]
                        };

                        dist_a.partial_cmp(&dist_b).unwrap()
                    })
                    .unwrap()
            };

            // println!(
            //     "Pair: ({:?}, {:?}), Bucket Difference: {}",
            //     lobby1[i], lobby2[best_j], min_bucket_diff
            // );

            pair_bucket_diffs.push((min_bucket_diff * min_bucket_diff) as f64);
            available.retain(|&x| x != best_j);
        }

        // println!("Bucket diffs: {:?}", pair_bucket_diffs);

        // Calculate geometric mean of (score + 1)^2, then subtract 1
        let arr: Vec<f64> = pair_bucket_diffs
            .iter()
            .map(|&x| (x + 1.0).powi(2))
            .collect();

        arr.iter()
            .fold(1.0, |acc, e| acc * e.powf(1.0 / arr.len() as f64))
            - 1.0
    }
}
