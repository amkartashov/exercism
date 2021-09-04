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
        topn.sort_unstable();

        for &score in self.scores.iter().skip(n) {
            // compare with the lowest score from the top and replace it
            if score > topn[0] {
                topn[0] = score;
                topn.sort_unstable();
            }
        }

        topn.reverse();
        topn
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.topn(3)
    }
}
