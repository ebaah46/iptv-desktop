use std::sync::Arc;

use libcore::domain::Stream;
use libcore::ports::PlaybackListener;
use qtbridge::qobject;

use crate::core::context;

#[derive(Debug)]
pub struct QmlPlayer {
    source: String,
    state: i32,
    listener: Option<Arc<dyn PlaybackListener>>,
}

impl Default for QmlPlayer {
    fn default() -> Self {
        Self {
            source: String::new(),
            state: 0,
            listener: None,
        }
    }
}

impl QmlPlayer {
    pub fn set_listener(&mut self, listener: Arc<dyn PlaybackListener>) {
        self.listener = Some(listener);
    }

    pub fn load(&mut self, stream: Stream) {
        self.source = stream.url;
        self.source_changed();
    }

    pub fn request_play(&mut self) {
        self.play_requested();
    }
    pub fn request_pause(&mut self) {
        self.pause_requested();
    }
    pub fn request_stop(&mut self) {
        self.stop_requested();
    }
    pub fn request_seek(&mut self, position: u32) {
        self.seek_requested(position);
    }
}

#[qobject(ConvertToCamelCase)]
impl QmlPlayer {
    qproperty!("source", Member = source, Notify = source_changed);
    qproperty!("state", Member = state, Notify = state_changed);

    #[qsignal]
    fn source_changed(&mut self);

    #[qsignal]
    fn state_changed(&mut self);

    #[qsignal(qml_name = "playRequested")]
    fn play_requested(&mut self);

    #[qsignal]
    fn pause_requested(&mut self);

    #[qsignal]
    fn stop_requested(&mut self);

    #[qsignal]
    fn seek_requested(&mut self, position: u32);

    /// Resolves and starts playback for `channel_id` through the core.
    ///
    /// Stream resolution may hit the network, so it runs on the blocking pool
    /// instead of stalling the Qt thread.
    #[qslot]
    fn play_channel(&mut self, channel_id: String) {
        let playback = context::get().playback();
        let _ = context::get().spawn_blocking(move || playback.play(&channel_id));
    }

    /// Clears the current source so a freshly created MediaPlayer does not
    /// start loading the previous channel before the new one is resolved.
    #[qslot]
    fn clear_source(&mut self) {
        if !self.source.is_empty() {
            self.source.clear();
            self.source_changed();
        }
    }

    #[qslot]
    fn on_playback_state_changed(&mut self, state: i32) {
        self.state = state;
        self.state_changed();

        let Some(listener) = self.listener.clone() else {
            return;
        };
        match state {
            1 => {
                context::get().spawn_blocking(move || {
                    let _ = listener.on_playback_started();
                });
            }
            0 => {
                context::get().spawn_blocking(move || {
                    let _ = listener.on_playback_stopped();
                });
            }
            _ => {}
        }
    }

    #[qslot]
    fn on_error(&mut self, message: String) {
        let Some(listener) = self.listener.clone() else {
            return;
        };
        context::get().spawn_blocking(move || {
            let _ = listener.on_playback_failed(&message);
        });
    }
}
