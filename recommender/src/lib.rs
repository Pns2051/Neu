use plugin_sdk::UnifiedTrack;
use std::collections::HashMap;

pub struct Recommender {
    embeddings: HashMap<String, Vec<f32>>,
    user_likes: Vec<String>,
}

impl Recommender {
    pub fn new() -> Self {
        Self {
            embeddings: HashMap::new(),
            user_likes: Vec::new(),
        }
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
        for i in 0..a.len() {
            dot += a[i] * b[i];
            norm_a += a[i] * a[i];
            norm_b += b[i] * b[i];
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
                for i in 0..dim {
                    user_profile[i] += emb[i];
                }
                count += 1;
            }
        }

        if count == 0 {
            return vec![];
        }

        for i in 0..dim {
            user_profile[i] /= count as f32;
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
