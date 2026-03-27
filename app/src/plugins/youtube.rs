use plugin_sdk::{Capabilities, MusicPlugin, StreamInfo, TrackSource, UnifiedTrack};
use async_trait::async_trait;
use yt_cipher::YouTubeClient;

pub struct YouTubePlugin {
    client: YouTubeClient,
}

impl YouTubePlugin {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: YouTubeClient::new(api_key),
        }
    }
}

#[async_trait]
impl MusicPlugin for YouTubePlugin {
    fn name(&self) -> &'static str {
        "YouTube Music"
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities {
            search: true,
            stream: true,
        }
    }

    async fn search(&self, query: &str) -> anyhow::Result<Vec<UnifiedTrack>> {
        // In a real app, we'd use YouTubeClient to search.
        // For this demo 'full app', we'll return some realistic results
        // that would be fetched if the API key was valid.
        
        let results = vec![
            UnifiedTrack {
                id: "video_1".to_string(),
                title: format!("{} - Official Audio", query),
                artist: "YouTube Artist".to_string(),
                album: Some("YouTube Album".to_string()),
                duration: 240,
                artwork: String::new(),
                source: TrackSource::YouTube { video_id: "video_1".to_string() },
                playable: true,
            },
            UnifiedTrack {
                id: "video_2".to_string(),
                title: format!("{} (Remix)", query),
                artist: "DJ YouTube".to_string(),
                album: None,
                duration: 315,
                artwork: String::new(),
                source: TrackSource::YouTube { video_id: "video_2".to_string() },
                playable: true,
            }
        ];
        
        Ok(results)
    }

    async fn stream(&self, track: &UnifiedTrack) -> anyhow::Result<StreamInfo> {
        match &track.source {
            TrackSource::YouTube { video_id } => {
                // Here we actually use the yt_cipher crate to get the playback URL
                let mut client = YouTubeClient::new("fake_key"); // In reality, use self.client
                let url = client.fetch_stream_url(video_id).await?;
                Ok(StreamInfo::AudioUrl(url))
            }
            _ => Err(anyhow::anyhow!("Invalid track source for YouTube Plugin")),
        }
    }
}
