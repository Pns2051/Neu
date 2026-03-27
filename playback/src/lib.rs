use anyhow::Result;
use plugin_sdk::{StreamInfo, UnifiedTrack};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::collections::VecDeque;
use std::io::{Read, Seek, SeekFrom};
use std::sync::{Arc, Condvar, Mutex};

/// Custom streaming buffer to allow Rodio to read from streaming HTTP responses
pub struct StreamBuffer {
    shared: Arc<(Mutex<VecDeque<u8>>, Condvar)>,
    position: u64,
    eof: Arc<Mutex<bool>>,
}

impl StreamBuffer {
    pub fn new(shared: Arc<(Mutex<VecDeque<u8>>, Condvar)>, eof: Arc<Mutex<bool>>) -> Self {
        Self { shared, position: 0, eof }
    }
}

impl Read for StreamBuffer {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let (lock, cvar) = &*self.shared;
        let mut inner = lock.lock().unwrap();
        
        // Wait for data or EOF
        while inner.is_empty() && !*self.eof.lock().unwrap() {
            inner = cvar.wait(inner).unwrap();
        }

        if inner.is_empty() {
            return Ok(0); // EOF
        }
        
        let mut read = 0;
        for (_i, b) in buf.iter_mut().enumerate() {
            if let Some(byte) = inner.pop_front() {
                *b = byte;
                read += 1;
            } else {
                break;
            }
        }
        
        self.position += read as u64;
        Ok(read)
    }
}

impl Seek for StreamBuffer {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        // Note: For a real seek, we'd clear the buffer and restart the HTTP request with a Range header.
        match pos {
            SeekFrom::Start(offset) => self.position = offset,
            SeekFrom::Current(offset) => self.position = (self.position as i64 + offset) as u64,
            SeekFrom::End(_) => {}
        }
        Ok(self.position)
    }
}

pub struct PlaybackEngine {
    _stream_handle: OutputStreamHandle,
    sink: Sink,
}

impl PlaybackEngine {
    pub fn new() -> Result<(Self, OutputStream)> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        Ok((Self { _stream_handle: stream_handle, sink }, stream))
    }

    pub async fn play_stream(&self, stream_info: StreamInfo) -> Result<()> {
        let url = match stream_info {
            StreamInfo::AudioUrl(u) | StreamInfo::VideoUrl(u) => u,
            _ => return Err(anyhow::anyhow!("Unsupported stream type")),
        };

        let shared = Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));
        let eof = Arc::new(Mutex::new(false));
        
        let shared_clone = shared.clone();
        let eof_clone = eof.clone();

        tokio::spawn(async move {
            let req = reqwest::Client::new().get(&url).header("Range", "bytes=0-");
            if let Ok(mut resp) = req.send().await {
                while let Ok(Some(chunk)) = resp.chunk().await {
                    let (lock, cvar) = &*shared_clone;
                    let mut inner = lock.lock().unwrap();
                    inner.extend(chunk);
                    cvar.notify_one();
                }
            }
            *eof_clone.lock().unwrap() = true;
            shared_clone.1.notify_all();
        });

        let reader = StreamBuffer::new(shared, eof);

        // BUG FIX: rodio's Decoder::new() performs blocking I/O to read the first few frames
        // to determine the generic audio format. Running this directly inside an async block
        // causes tokio worker thread starvation. We offload it to a blocking pool.
        let decoder_result = tokio::task::spawn_blocking(move || {
            Decoder::new(reader)
        }).await?;

        if let Ok(decoder) = decoder_result {
            self.sink.append(decoder);
            self.sink.play();
        }
        
        Ok(())
    }

    pub fn preload_next(&self, _track: &UnifiedTrack) {
        // Gapless playback
        // Implementation would check remaining duration, fetch stream URL, and call append on the sink
    }

    pub fn toggle(&self) {
        if self.sink.is_paused() {
            self.sink.play();
        } else {
            self.sink.pause();
        }
    }

    pub fn is_paused(&self) -> bool {
        self.sink.is_paused()
    }
}
