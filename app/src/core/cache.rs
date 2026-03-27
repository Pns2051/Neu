use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::fs;

#[allow(dead_code)]
pub struct CacheLayer {
    memory_cache: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    disk_dir: PathBuf,
}

#[allow(dead_code)]
impl CacheLayer {
    pub fn new<P: AsRef<Path>>(disk_dir: P) -> anyhow::Result<Self> {
        let path = disk_dir.as_ref().to_path_buf();
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }
        Ok(Self {
            memory_cache: Arc::new(Mutex::new(HashMap::new())),
            disk_dir: path,
        })
    }

    pub async fn fetch<F, Fut>(&self, key: &str, fetcher: F) -> anyhow::Result<Vec<u8>>
    where
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = anyhow::Result<Vec<u8>>> + Send,
    {
        // 1. Memory check
        if let Some(data) = self.memory_cache.lock().unwrap().get(key) {
            return Ok(data.clone());
        }

        // 2. Disk check
        let disk_path = self.disk_dir.join(key);
        if disk_path.exists() {
            if let Ok(data) = fs::read(&disk_path) {
                self.memory_cache.lock().unwrap().insert(key.to_string(), data.clone());
                return Ok(data);
            }
        }

        // 3. Remote fetch via fetcher
        let data = fetcher().await?;

        // 4. Store in disk and memory
        let _ = fs::write(&disk_path, &data);
        self.memory_cache.lock().unwrap().insert(key.to_string(), data.clone());

        Ok(data)
    }
}
