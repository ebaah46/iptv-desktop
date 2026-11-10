use anyhow::Result as Res;
use libcore::domain::Stream;
use libcore::ports::PlayerController;
use qtbridge::QThreadHandle;

use crate::player::QmlPlayer;

/// This adapter struct implements the bidirectional communication between
/// the core layer and the QML VideoPlayer functionality to enable
/// the core to communicate with the video player.
pub struct PlayerAdapter {
    handle: QThreadHandle<QmlPlayer>,
}

impl PlayerAdapter {
    pub fn new(handle: QThreadHandle<QmlPlayer>) -> Self {
        Self { handle }
    }
}

impl PlayerController for PlayerAdapter {
    fn load(&self, stream: Stream) -> Res<()> {
        self.handle.invoke_mut(move |vm| vm.load(stream));
        Ok(())
    }

    fn play(&self) -> Res<()> {
        self.handle.invoke_mut(|vm| vm.request_play());
        Ok(())
    }

    fn pause(&self) -> Res<()> {
        self.handle.invoke_mut(|vm| vm.request_pause());
        Ok(())
    }

    fn stop(&self) -> Res<()> {
        self.handle.invoke_mut(|vm| vm.request_stop());
        Ok(())
    }

    fn seek_position(&self, position: u32) -> Res<()> {
        self.handle.invoke_mut(move |vm| vm.request_seek(position));
        Ok(())
    }
}

impl std::fmt::Debug for PlayerAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlayerAdapter")
            .field("handle", &"<QThreadHandle<QmlPlayer>>")
            .finish_non_exhaustive()
    }
}
