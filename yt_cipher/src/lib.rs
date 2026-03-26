pub mod parser;

use anyhow::Result;
use parser::{decipher, Op};
use reqwest::Client;

pub struct YouTubeClient {
    client: Client,
    api_key: String,
    js_ops_cache: Vec<Op>,
}

impl YouTubeClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            js_ops_cache: vec![],
        }
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
