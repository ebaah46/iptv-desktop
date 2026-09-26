use std::sync::{Arc, RwLock};
use std::time::Duration;
use libcore::domain::Stream;
use libcore::ports::PlayerController;

use anyhow::Result as Res;
use iced_video_player::Video;
use log::info;

#[derive(Debug)]
pub struct GstPlayerController {
    video: Arc<RwLock<Option<Video>>>
}

impl GstPlayerController {
    pub fn new(video: Arc<RwLock<Option<Video>>>) -> Self {
        Self { video }
    }
}

impl PlayerController for GstPlayerController {
    fn load(&self, stream: Stream) -> Res<()> {
        let url = url::Url::parse(stream.url.as_str())?;
        let video = Video::new(&url).map_err(|err| anyhow::anyhow!("Player error could not load video, error: {}",err))?;
        if let Ok(mut write_guard) = self.video.write(){
            *write_guard = Some(video);
        } else {
            return Err(anyhow::anyhow!("Player could not load video, lock poisoned"));
        }
        Ok(())
    }

    fn play(&self) -> Res<()> {
        let mut write_guard = self.video.write().map_err(|err| anyhow::anyhow!("Player could not play video, lock poisoned"))?;
        let video = write_guard.as_mut().ok_or(anyhow::anyhow!("Player could not play video, no video loaded"))?;
        video.set_paused(false);
        Ok(())
    }

    fn pause(&self) -> Res<()> {
        let mut write_guard = self.video.write().map_err(|err| anyhow::anyhow!("Player could not play video, lock poisoned"))?;
        let video = write_guard.as_mut().ok_or(anyhow::anyhow!("Player could not play video, no video loaded"))?;
        video.set_paused(true);
        Ok(())
    }

    fn stop(&self) -> Res<()> {
        let mut write_guard = self.video.write().map_err(|err| anyhow::anyhow!("Player could not play video, lock poisoned"))?;
        let video = write_guard.as_mut().ok_or(anyhow::anyhow!("Player could not play video, no video loaded"))?;
        video.set_paused(true);
        video.seek(Duration::ZERO, false)?;
        Ok(())
    }

    // for streaming seek can only move to a previous time and not future time
    fn seek_position(&self, position: u32) -> Res<()> {
        let mut write_guard = self.video.write().map_err(|err| anyhow::anyhow!("Player could not play video, lock poisoned"))?;
        let video = write_guard.as_mut().ok_or(anyhow::anyhow!("Player could not play video, no video loaded"))?;
        let target_time = Duration::from_secs(position as u64);
        let current_time = video.position();
        if target_time > current_time {
            info!("Seeking to future time not possible. Current time: {}, Target time: {}",current_time.as_secs(), target_time.as_secs());
            return Ok(())
        }
        video.seek(target_time, false)?; // seek
        Ok(())
    }
}