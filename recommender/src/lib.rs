use std::collections::HashMap;

#[derive(Default)]
pub struct Recommender {
    embeddings: HashMap<String, Vec<f32>>,
    user_likes: Vec<String>,
}

impl Recommender {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_track(&mut self, track_id: &str, embedding: Vec<f32>) {
        self.embeddings.insert(track_id.to_string(), embedding);
    }

    pub fn like_track(&mut self, track_id: &str) {
        if !self.user_likes.contains(&track_id.to_string()) {
            self.user_likes.push(track_id.to_string());
        }
    }

    fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }
        let mut dot = 0.0;
        let mut norm_a = 0.0;
        let mut norm_b = 0.0;
        for (a_val, b_val) in a.iter().zip(b) {
            dot += a_val * b_val;
            norm_a += a_val * a_val;
            norm_b += b_val * b_val;
        }
        if norm_a == 0.0 || norm_b == 0.0 {
            return 0.0;
        }
        dot / (norm_a.sqrt() * norm_b.sqrt())
    }

    pub fn recommend(&self, top_n: usize) -> Vec<String> {
        if self.user_likes.is_empty() || self.embeddings.is_empty() {
            return vec![];
        }

        let dim = self.embeddings.values().next().unwrap().len();
        let mut user_profile = vec![0.0; dim];
        let mut count = 0;

        for id in &self.user_likes {
            if let Some(emb) = self.embeddings.get(id) {
                for (profile_val, emb_val) in user_profile.iter_mut().zip(emb) {
                    *profile_val += emb_val;
                }
                count += 1;
            }
        }

        if count == 0 {
            return vec![];
        }

        for val in &mut user_profile {
            *val /= count as f32;
        }

        let mut scores: Vec<(String, f32)> = self.embeddings
            .iter()
            .filter(|(id, _)| !self.user_likes.contains(id))
            .map(|(id, emb)| (id.clone(), Self::cosine_similarity(&user_profile, emb)))
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores.into_iter().take(top_n).map(|(id, _)| id).collect()
    }
}
