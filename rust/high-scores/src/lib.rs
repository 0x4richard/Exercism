#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        if let Some(&score) = self.scores.last() {
            return Some(score);
        }

        None
    }

    pub fn personal_best(&self) -> Option<u32> {
        if let Some(&score) = self.scores.iter().max() {
            return Some(score);
        }

        None
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.clone();
        scores.sort();
        scores.into_iter().rev().take(3).collect()
    }
}
