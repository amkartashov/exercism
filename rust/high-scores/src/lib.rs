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
        let mut topn: Vec<_> = self.scores.iter().copied().take(n).collect();

        // use sort_by to sort in descending order
        topn.sort_unstable_by(|a, b| b.cmp(a));

        for &score in self.scores.iter().skip(n) {
            topn.push(score);
            topn.sort_unstable_by(|a, b| b.cmp(a));
            topn.pop();
        }

        topn
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.topn(3)
    }
}
