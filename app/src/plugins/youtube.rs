use tokio::sync::Mutex;

pub struct YouTubePlugin {
    client: Mutex<YouTubeClient>,
}

impl YouTubePlugin {
    pub fn new(api_key: &str) -> Self {
        Self {
            client: Mutex::new(YouTubeClient::new(api_key)),
        }
    }
}

use plugin_sdk::{Capabilities, MusicPlugin, StreamInfo, TrackSource, UnifiedTrack};
use async_trait::async_trait;
use yt_cipher::YouTubeClient;

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
        let client = self.client.lock().await;
        let results = client.search(query).await?;
        
        let unified = results.into_iter().map(|t| UnifiedTrack {
            id: t.id.clone(),
            title: t.title,
            artist: t.artist,
            album: t.album,
            duration: t.duration,
            artwork: String::new(),
            source: TrackSource::YouTube { video_id: t.id },
            playable: true,
        }).collect();
        
        Ok(unified)
    }

    async fn stream(&self, track: &UnifiedTrack) -> anyhow::Result<StreamInfo> {
        match &track.source {
            TrackSource::YouTube { video_id } => {
                let mut client = self.client.lock().await;
                let url = client.fetch_stream_url(video_id).await?;
                Ok(StreamInfo::AudioUrl(url))
            }
            _ => Err(anyhow::anyhow!("Invalid track source for YouTube Plugin")),
        }
    }
}
