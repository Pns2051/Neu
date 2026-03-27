pub mod parser;

use anyhow::Result;
use parser::{decipher, Op};
use reqwest::Client;

pub struct YouTubeClient {
    client: Client,
    api_key: String,
    js_ops_cache: Vec<Op>,
}

#[derive(Debug, Clone)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub duration: u32,
}

impl YouTubeClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            js_ops_cache: vec![],
        }
    }

    pub async fn search(&self, query: &str) -> Result<Vec<Track>> {
        // Real search logic would go here:
        // https://music.youtube.com/youtubei/v1/search?key=...
        // For now, return a more formal "mock" that simulates the API structure
        Ok(vec![
            Track {
                id: "vid_123".into(),
                title: format!("{} - Mastered", query),
                artist: "Universal Artist".into(),
                album: Some("Legacy Album".into()),
                duration: 210,
            },
            Track {
                id: "vid_456".into(),
                title: format!("{} (Live at Wembley)", query),
                artist: "Universal Artist".into(),
                album: None,
                duration: 450,
            }
        ])
    }

    pub async fn fetch_stream_url(&mut self, video_id: &str) -> Result<String> {
        let payload = serde_json::json!({
            "context": {
                "client": {
                    "clientName": "WEB",
                    "clientVersion": "2.20210721.00.00",
                }
            },
            "videoId": video_id,
        });

        let url = format!("https://music.youtube.com/youtubei/v1/player?key={}", self.api_key);
        // We'd send the request, parsing streamingData -> adaptiveFormats
        let _res = self.client.post(&url).json(&payload).send().await?;

        // Mocks for prototype:
        let mock_sig = "mock_encoded_signature";
        if self.js_ops_cache.is_empty() {
            self.js_ops_cache = parser::parse_js("mock js").unwrap_or_default();
        }

        let descrambled = decipher(mock_sig, &self.js_ops_cache);
        Ok(format!("https://playback.youtube.com/videoplayback?sig={}", descrambled))
    }
}
