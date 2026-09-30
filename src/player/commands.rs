use libcore::domain::Stream;

/**
* This enumeration defines commands that can be sent from the player to the
* UI state about an operation that is to be executed.
*/
#[derive(Debug, Clone)]
pub enum PlayerCommand {
    Load(Stream),
    Play,
    Pause,
    Stop,
    Seek(u32)
}


/**
* This enumeration defines events that can be received from the player UI
* state about an ongoing video/audio playback.
*/

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    Started,
    Failed(String),
    Stopped
}