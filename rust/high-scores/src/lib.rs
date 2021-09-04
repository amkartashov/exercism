use std::{cmp::Reverse, collections::BinaryHeap};

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
        let mut topn: BinaryHeap<_> = self.scores.iter().copied().take(n).map(Reverse).collect();

        for &score in self.scores.iter().skip(n) {
            topn.push(Reverse(score));
            topn.pop();
        }

        let mut topn = topn.into_iter().map(|r| r.0).collect::<Vec<_>>();

        topn.sort_unstable();
        topn.reverse();

        topn
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.topn(3)
    }
}
