use libcore::domain::Stream;
use libcore::ports::PlayerController;
use libcore::ports::PlaybackListener;

use anyhow::Result as Res;

#[derive(Debug)]
pub struct GstPlayerController {}

impl GstPlayerController{
    pub fn new() -> Self{
        GstPlayerController{}
    }
}
impl PlayerController for GstPlayerController {
    fn load(&self, stream: Stream) -> Res<()> {
        todo!()
    }

    fn play(&self) -> Res<()> {
        todo!()
    }

    fn pause(&self) -> Res<()> {
        todo!()
    }

    fn stop(&self) -> Res<()> {
        todo!()
    }

    fn seek_position(&self, position: u32) -> Res<()> {
        todo!()
    }
}

impl PlaybackListener for GstPlayerController {
    fn on_playback_started(&self) -> Res<()> {
        todo!()
    }

    fn on_playback_failed(&self, error: &str) -> Res<()> {
        todo!()
    }

    fn on_playback_stopped(&self) -> Res<()> {
        todo!()
    }
}

