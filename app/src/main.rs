slint::include_modules!();

mod cache;
mod plugins;

use std::rc::Rc;
use slint::{ModelRc, VecModel};
use tokio::runtime::Runtime;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    // UI Init
    let app = AppWindow::new()?;
    let tracks_model: Rc<VecModel<TrackData>> = Rc::new(VecModel::default());
    app.set_tracks(ModelRc::from(tracks_model.clone()));

    let app_weak = app.as_weak();
    let _rt = Runtime::new()?;

    // Initialize Local Plugin
    let _local_plugin = plugins::local::LocalPlugin::new("/home/ela/Music");

    app.on_search(move |query| {
        let app = app_weak.unwrap();
        app.set_now_playing(slint::SharedString::from(format!("Neu — Searching: {}", query)));
        
        // Mocking async plugin loading logic to populate UI
        tracks_model.set_vec(vec![
            TrackData {
                title: slint::SharedString::from(format!("{} (Local)", query)),
                artist: slint::SharedString::from("Artist A"),
                duration: slint::SharedString::from("3:45"),
            },
            TrackData {
                title: slint::SharedString::from(format!("{} (YouTube)", query)),
                artist: slint::SharedString::from("Artist B"),
                duration: slint::SharedString::from("4:20"),
            }
        ]);
        
        app.set_now_playing(slint::SharedString::from("Search complete."));
    });

    let app_weak_play = app.as_weak();
    app.on_play_track(move |index| {
        let app = app_weak_play.unwrap();
        app.set_now_playing(slint::SharedString::from(format!("Playing track {}", index)));
    });

    app.run()?;

    Ok(())
}
