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