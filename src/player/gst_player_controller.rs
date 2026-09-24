use libcore::domain::Stream;
use libcore::ports::PlayerController;
use anyhow::Result as Res;


#[derive(Debug)]
pub struct GstPlayerController;

impl GstPlayerController {
    pub fn new() -> Self {
        Self
    }
}

impl PlayerController for GstPlayerController {
    fn load(&self, stream: Stream) -> Res<()> {
        todo!()
    }

    fn play(&self) -> Res<()> {
        Ok(())
    }

    fn pause(&self) -> Res<()> {

        Ok(())
    }

    fn stop(&self) -> Res<()> {

        Ok(())
    }

    fn seek_position(&self, position: u32) -> Res<()> {
        Ok(())
    }
}