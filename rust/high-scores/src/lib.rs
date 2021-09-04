#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    fn topn(&self, n: usize) -> Vec<u32> {
        if n == 0 || self.scores.len() == 0 {
            return Vec::new();
        }

        let mut topn: Vec<_> = self.scores.iter().copied().take(n).collect();
        // use sort_by to sort in descending order
        topn.sort_unstable_by(|a, b| b.cmp(a));

        // link to the lowest of the top
        let last: *mut u32 = topn.last_mut().unwrap();

        for &score in self.scores.iter().skip(n) {
            // this is safe to unreference the pointer
            // because sort_unstable_by does in-place sorting, without allocation
            // so topn underlying memory is not moved.
            if unsafe { score > *last } {
                unsafe { *last = score };
                topn.sort_unstable_by(|a, b| b.cmp(a));
            }
        }

        topn
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.topn(3)
    }
}
