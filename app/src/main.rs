slint::include_modules!();

#[path = "core/cache.rs"]
mod cache;
mod plugins;

use std::rc::Rc;
use std::sync::Arc;
use slint::{ModelRc, VecModel, SharedString, Model};
use tokio::runtime::Runtime;
use playback::PlaybackEngine;
use plugin_sdk::MusicPlugin;
use recommender::Recommender;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    // 1. Initialize Runtime
    let rt = Runtime::new()?;
    let _guard = rt.enter();

    // 2. Initialize Core Engines
    let (engine, _stream) = PlaybackEngine::new()?;
    let engine = Arc::new(engine);
    let _recommender = Recommender::new();
    
    // 3. Initialize Plugins
    let plugins: Vec<Box<dyn MusicPlugin>> = vec![
        Box::new(plugins::local::LocalPlugin::new("/home/ela/Music")),
        Box::new(plugins::youtube::YouTubePlugin::new("neu_demo_key")),
    ];
    let plugins = Arc::new(plugins);

    // 4. UI Init
    let app = AppWindow::new()?;
    let tracks_model: Rc<VecModel<TrackData>> = Rc::new(VecModel::default());
    app.set_tracks(ModelRc::from(tracks_model.clone()));

    let app_weak = app.as_weak();
    
    // --- CALLBACKS ---

    // Search Callback
    let plugins_search = Arc::clone(&plugins);
    app.on_search({
        let app_weak = app_weak.clone();
        move |query| {
            let app = app_weak.unwrap();
            let query = query.to_string();
            let plugins = Arc::clone(&plugins_search);
            let app_weak_inner = app_weak.clone();

            app.set_now_playing_title(SharedString::from(format!("Searching: {}", query)));
            
            tokio::spawn(async move {
                let mut all_results = Vec::new();
                for plugin in plugins.iter() {
                    if let Ok(results) = plugin.search(&query).await {
                        for t in results {
                            all_results.push(TrackData {
                                id: SharedString::from(&t.id),
                                title: SharedString::from(&t.title),
                                artist: SharedString::from(&t.artist),
                                album: SharedString::from(t.album.as_deref().unwrap_or("")),
                                duration: SharedString::from(format!("{}:{:02}", t.duration / 60, t.duration % 60)),
                                source: SharedString::from(match t.source {
                                    plugin_sdk::TrackSource::YouTube { .. } => "youtube",
                                    _ => "local",
                                }),
                            });
                        }
                    }
                }

                let app_weak = app_weak_inner.clone();
                slint::invoke_from_event_loop(move || {
                    if let Some(app) = app_weak.upgrade() {
                        let new_model = Rc::new(VecModel::from(all_results));
                        app.set_tracks(ModelRc::from(new_model));
                        app.set_now_playing_title(SharedString::from("Neu Music Platform"));
                    }
                }).unwrap();
            });
        }
    });

    // Play Track Callback
    let engine_play = Arc::clone(&engine);
    let plugins_play = Arc::clone(&plugins);
    app.on_play_track({
        let app_weak = app_weak.clone();
        let tracks_model = tracks_model.clone();
        move |index| {
            let app = app_weak.unwrap();
            let track_data = tracks_model.row_data(index as usize).unwrap();
            let engine = Arc::clone(&engine_play);
            let plugins = Arc::clone(&plugins_play);

            app.set_now_playing_title(track_data.title.clone());
            app.set_now_playing_artist(track_data.artist.clone());
            app.set_is_playing(true);

            tokio::spawn(async move {
                // Find matching track in plugins (simplification for demo)
                // Real app would have a unified cache or better plugin orchestration
                for plugin in plugins.iter() {
                    // This is a bit inefficient but works for the demo
                    if let Ok(search_results) = plugin.search(&track_data.title).await {
                        if let Some(track) = search_results.iter().find(|t| track_data.title == t.title) {
                            if let Ok(stream_info) = plugin.stream(track).await {
                                let _ = engine.play_stream(stream_info).await;
                                break;
                            }
                        }
                    }
                }
            });
        }
    });

    // View Switching
    app.on_change_view({
        let app_weak = app_weak.clone();
        move |view| {
            app_weak.unwrap().set_current_view(view);
        }
    });

    // Play/Pause
    let engine_pause = Arc::clone(&engine);
    app.on_toggle_play_pause({
        let app_weak = app_weak.clone();
        move || {
            let app = app_weak.unwrap();
            engine_pause.toggle();
            app.set_is_playing(!engine_pause.is_paused());
        }
    });

    app.run()?;

    Ok(())
}
