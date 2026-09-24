use std::sync::Mutex;
use libcore::domain::Stream;
use libcore::ports::PlayerController;
use anyhow::Result as Res;

/// A GStreamer-based player controller implementation.
///
/// Uses `playbin` which handles demuxing, decoding and playback
/// automatically. The pipeline is stored behind a Mutex for interior
/// mutability, required for the `Send + Sync` trait bounds.
#[derive(Debug)]
pub struct GstPlayerController {
    pipeline: Mutex<Option<gstreamer::Element>>,
}

impl GstPlayerController {
    pub fn new() -> Self {
        GstPlayerController {
            pipeline: Mutex::new(None),
        }
    }
}

impl PlayerController for GstPlayerController {
    fn load(&self, stream: Stream) -> Res<()> {
        use gstreamer::prelude::*;

        // Create the playbin element
        let playbin = gstreamer::ElementFactory::make("playbin")
            .name("playbin")
            .build()?;

        // Set the URI
        playbin.set_property("uri", &stream.url);

        // Clone a reference for the bus watcher thread before storing the original
        let bus_playbin = playbin.clone();

        // Store the pipeline
        *self.pipeline.lock().unwrap() = Some(playbin);

        // Set up a bus watcher on a separate thread
        let bus = bus_playbin.bus().expect("playbin without bus");
        std::thread::spawn(move || {
            for msg in bus.iter() {
                use gstreamer::MessageView;
                match msg.view() {
                    MessageView::Eos(..) => {
                        eprintln!("[GstPlayer] EOS received");
                        let _ = bus_playbin.set_state(gstreamer::State::Null);
                        break;
                    }
                    MessageView::Error(err) => {
                        eprintln!(
                            "[GstPlayer] Error: {} ({})",
                            err.error(),
                            err.debug().unwrap_or_default()
                        );
                        let _ = bus_playbin.set_state(gstreamer::State::Null);
                        break;
                    }
                    MessageView::StateChanged(state_changed) => {
                        if let Some(src) = state_changed.src() {
                            if src.name() == "playbin" {
                                let old = state_changed.old();
                                let new = state_changed.current();
                                eprintln!("[GstPlayer] State changed: {:?} -> {:?}", old, new);
                            }
                        }
                    }
                    _ => {}
                }
            }

            let _ = bus_playbin.set_state(gstreamer::State::Null);
            eprintln!("[GstPlayer] Bus watch thread exiting");
        });

        Ok(())
    }

    fn play(&self) -> Res<()> {
        use gstreamer::prelude::*;
        let pipeline = self.pipeline.lock().unwrap();
        if let Some(ref playbin) = *pipeline {
            playbin.set_state(gstreamer::State::Playing)?;
        }
        Ok(())
    }

    fn pause(&self) -> Res<()> {
        use gstreamer::prelude::*;
        let pipeline = self.pipeline.lock().unwrap();
        if let Some(ref playbin) = *pipeline {
            playbin.set_state(gstreamer::State::Paused)?;
        }
        Ok(())
    }

    fn stop(&self) -> Res<()> {
        use gstreamer::prelude::*;
        let pipeline = self.pipeline.lock().unwrap();
        if let Some(ref playbin) = *pipeline {
            playbin.set_state(gstreamer::State::Null)?;
        }
        Ok(())
    }

    fn seek_position(&self, position: u32) -> Res<()> {
        use gstreamer::prelude::*;
        let pipeline = self.pipeline.lock().unwrap();
        if let Some(ref playbin) = *pipeline {
            let position_ns = gstreamer::ClockTime::from_seconds(position as u64);
            playbin.seek_simple(
                gstreamer::SeekFlags::FLUSH | gstreamer::SeekFlags::KEY_UNIT,
                position_ns,
            )?;
        }
        Ok(())
    }
}