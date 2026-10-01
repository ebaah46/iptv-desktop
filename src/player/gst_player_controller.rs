use std::sync::{Arc, OnceLock};
use libcore::domain::Stream;
use libcore::ports::{PlaybackListener, PlayerController};

use anyhow::Result as Res;
use log::{warn};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use crate::player::commands::{PlayerCommand, PlayerEvent};
use crate::player::commands::PlayerCommand::Seek;


#[derive(Debug)]
pub struct GstPlayerController {
    command_tx: UnboundedSender<PlayerCommand>,
    // This OnceLock hack can be removed when we migrate to event driven architecture
    controller: Arc<OnceLock<Arc<dyn PlaybackListener>>>,
}

impl GstPlayerController {
    pub fn new(command_tx: UnboundedSender<PlayerCommand>, mut event_rx: UnboundedReceiver<PlayerEvent>) -> Self {

        let controller:Arc<OnceLock<Arc<dyn PlaybackListener>>> = Arc::new(OnceLock::new());
        let controller_clone = controller.clone();
        tokio::task::spawn_blocking( move || {
            while let Some(event) = event_rx.blocking_recv() {
                if let Some(listener) = controller_clone.get(){
                    let callback_result = match event {
                        PlayerEvent::Started => listener.on_playback_started(),
                        PlayerEvent::Failed(err) => listener.on_playback_failed(&err),
                        PlayerEvent::Stopped => listener.on_playback_stopped(),
                    };
                    if let Err(e) = callback_result {
                        warn!("[core] callback execution failed for current playback: {:?}", e);
                    }
                }
            }
        });
        Self { command_tx, controller }
    }

    pub fn set_listener(&self, listener: Arc<dyn PlaybackListener>) {
        let _ =  self.controller.set(listener);
    }
}

/// Failure in sending command messages are internal errors that affect the application.
/// They are not to be handled/displayed in the UI in any way.
impl PlayerController for GstPlayerController {
    fn load(&self, stream: Stream) -> Res<()> {
        if let Err(e) = self.command_tx.send(PlayerCommand::Load(stream)){
            warn!("[player_controller] error sending Load command: {:?}", e);
        }
        Ok(())
    }

    fn play(&self) -> Res<()> {
        if let Err(e) = self.command_tx.send(PlayerCommand::Play) {
            warn!("[player_controller] error sending play command: {:?}", e);
        }
        Ok(())
    }

    fn pause(&self) -> Res<()> {
        if let Err(e) = self.command_tx.send(PlayerCommand::Pause) {
            warn!("[player_controller] error sending pause command: {:?}", e);
        }
        Ok(())
    }

    fn stop(&self) -> Res<()> {
        if let Err(e) = self.command_tx.send(PlayerCommand::Stop) {
            warn!("[player_controller] error sending stop command: {:?}", e);
        }
        Ok(())
    }

    fn seek_position(&self, position: u32) -> Res<()> {
        if let Err(e) = self.command_tx.send(Seek(position)) {
            warn!("[player_controller] error sending seek command: {:?}", e);
        }
        Ok(())
    }

}
