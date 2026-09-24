use slint::ComponentHandle;
use std::sync::Arc;
use libcore::facade::{CoreFacade, IptvFacade};
use slint::Weak;

use crate::{MainWindow, NavigationState, PlayerState};

/**
 * Implements the player controller responsible for orchestrating
 * communication between the player UI screen and the core playback layer.
 * Handles channel playback lifecycle: play, pause, stop, and stream loading.
 */
pub struct PlayerController {
    window: Weak<MainWindow>,
    core: Arc<IptvFacade>,
}

impl PlayerController {

    pub fn new(window: Weak<MainWindow>, core: Arc<IptvFacade>) -> Self {
        PlayerController { window, core }
    }

    /// Register callbacks for player screen interactions
    pub fn register(&self) {
        let Some(window) = self.window.upgrade() else { return };
        let player_state = window.global::<PlayerState>();

        // Stream requested — called when a channel is selected from the home screen
        let core_play = self.core.clone();
        let weak_window_play = self.window.clone();
        player_state.on_stream_requested(move |channel_id| {
            // Load and start playback for the given channel
            let channel_id_str = channel_id.to_string();
            core_play.play(&channel_id_str);

            // Update UI state after stream loading
            if let Some(window) = weak_window_play.upgrade() {
                let player = window.global::<PlayerState>();
                player.set_is_loading(false);
                player.set_is_playing(true);
            }
        });

        // Play/Pause toggle
        let core_playback = self.core.clone();
        player_state.on_play_pause_toggled(move |is_playing| {
            if is_playing {
                let _ = core_playback.pause();
            } else {
                let _ = core_playback.pause();
            }
        });

        // Back requested — stop playback and return to home
        let core_stop = self.core.clone();
        let weak_window_stop = self.window.clone();
        player_state.on_back_requested(move || {
            core_stop.stop();
            if let Some(window) = weak_window_stop.upgrade() {
                let player = window.global::<PlayerState>();
                player.set_is_playing(false);
                player.set_is_loading(false);
                player.set_error_message("".into());
                let nav = window.global::<NavigationState>();
                nav.set_selected_channel_id("".into());
                nav.set_selected_channel_name("".into());
            }
        });

        dbg!("PlayerController registered successfully");
    }
}