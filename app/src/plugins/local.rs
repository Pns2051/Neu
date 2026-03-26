use plugin_sdk::{Capabilities, MusicPlugin, StreamInfo, TrackSource, UnifiedTrack};
use async_trait::async_trait;
use lofty::{Accessor, AudioFile, TaggedFileExt};
use lofty::read_from_path;
use walkdir::WalkDir;

pub struct LocalPlugin {
    base_path: String,
}

impl LocalPlugin {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.to_string(),
        }
    }
}

#[async_trait]
impl MusicPlugin for LocalPlugin {
    fn name(&self) -> &'static str {
        "Local Files"
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities {
            search: true,
            stream: true,
        }
    }

    async fn search(&self, query: &str) -> anyhow::Result<Vec<UnifiedTrack>> {
        let mut tracks = Vec::new();
        let query_lower = query.to_lowercase();

        for entry in WalkDir::new(&self.base_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if ["mp3", "flac", "wav", "m4a", "ogg"].contains(&ext) {
                        let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_lowercase();
                        if query.is_empty() || file_name.contains(&query_lower) {
                            
                            let mut title = file_name.to_string();
                            let mut artist = "Unknown Artist".to_string();
                            let mut duration = 0;

                            if let Ok(tagged_file) = read_from_path(path) {
                                duration = tagged_file.properties().duration().as_secs() as u32;
                                if let Some(tag) = tagged_file.primary_tag() {
                                    if let Some(t) = tag.title() {
                                        title = t.to_string();
                                    }
                                    if let Some(a) = tag.artist() {
                                        artist = a.to_string();
                                    }
                                }
                            }

                            tracks.push(UnifiedTrack {
                                id: path.to_string_lossy().to_string(),
                                title,
                                artist,
                                album: None,
                                duration,
                                artwork: String::new(),
                                source: TrackSource::Local { path: path.to_string_lossy().to_string() },
                                playable: true,
                            });

                            if tracks.len() >= 50 {
                                break;
                            }
                        }
                    }
                }
            }
        }
        Ok(tracks)
    }

    async fn stream(&self, track: &UnifiedTrack) -> anyhow::Result<StreamInfo> {
        match &track.source {
            TrackSource::Local { path } => {
                Ok(StreamInfo::External(path.clone()))
            }
            _ => Err(anyhow::anyhow!("Invalid track source for Local Plugin")),
        }
    }
}
