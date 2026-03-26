pub use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq)]
pub enum TrackSource {
    YouTube { video_id: String },
    Spotify { track_id: String },
    AppleMusic { song_id: String },
    Local { path: String },
}

#[derive(Debug, Clone)]
pub struct UnifiedTrack {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub duration: u32,
    pub artwork: String,
    pub source: TrackSource,
    pub playable: bool,
}

#[derive(Debug, Clone)]
pub struct Capabilities {
    pub search: bool,
    pub stream: bool,
}

#[derive(Debug, Clone)]
pub enum StreamInfo {
    AudioUrl(String),
    VideoUrl(String),
    External(String),
    None,
}

#[async_trait]
pub trait MusicPlugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn capabilities(&self) -> Capabilities;
    async fn search(&self, query: &str) -> anyhow::Result<Vec<UnifiedTrack>>;
    async fn stream(&self, track: &UnifiedTrack) -> anyhow::Result<StreamInfo>;
}

pub unsafe fn load_plugin(path: &str) -> anyhow::Result<Box<dyn MusicPlugin>> {
    let lib_box = Box::new(libloading::Library::new(path)?);
    // Leak the library to prevent unloading when the Box is dropped,
    // otherwise the plugin's vtable will point to unmapped memory!
    let lib = Box::leak(lib_box);
    
    let func: libloading::Symbol<fn() -> Box<dyn MusicPlugin>> = lib.get(b"create_plugin")?;
    Ok(func())
}
