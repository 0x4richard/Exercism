#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
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
        let mut scores = self.scores.clone().to_vec();
        scores.sort();
        scores.into_iter().rev().take(3).collect()
    }
}
