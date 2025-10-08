use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Diff {
    total: u16,
    index: usize,
    _base: [u16; 15],
    _scaled: [f64; 15],
    rounded: [u16; 15],
    done: bool,
}

impl Debug for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Diff")
            .field("total", &self.total)
            .field("index", &self.index)
            .field("done", &self.done)
            .finish()
    }
}

impl Diff {
    pub fn new(total: u16) -> Self {
        let base: [u16; 15] = std::array::from_fn(|i| (i + 1) as u16); // [1, 2, ..., 15]
        let base_sum: u16 = base.iter().sum();

        // Step 1: Compute scaled values
        let mut scaled = [0.0; 15];
        for i in 0..15 {
            scaled[i] = (total as f64) * (base[i] as f64) / (base_sum as f64);
        }

        // Step 2: Round and compute sum
        let mut rounded = [0u16; 15];
        let mut int_sum = 0u16;
        for i in 0..15 {
            rounded[i] = scaled[i].round() as u16;
            int_sum += rounded[i];
        }

        // Step 3: Fix rounding error
        let mut diff = int_sum as i32 - total as i32;
        if diff != 0 {
            let last = &mut rounded[14];
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
                    let take = std::cmp::min(diff, rounded[i] as i32);
                    rounded[i] -= take as u16;
                    diff -= take;
                }
            }
        }

        Self {
            total,
            index: 0,
            _base: base,
            _scaled: scaled,
            rounded,
            done: false,
        }
    }
}

impl Iterator for Diff {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= 15 {
            return None;
        }

        let value = self.rounded[self.index];
        self.index += 1;

        Some(value)
    }
}
