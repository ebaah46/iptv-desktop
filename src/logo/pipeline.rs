use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use futures::{Stream, StreamExt};
use tokio::sync::Notify;

use crate::logo::{LogoBatch, LogoLoader};
use crate::ui_model::{ChannelItem, ChannelItems};

/// Number of logo loads allowed to run in parallel (process-wide).
const CONCURRENCY: usize = 8;
/// Number of finished loads grouped into a single progressive update.
const BATCH_SIZE: usize = 15;

/// A demand-driven queue feeding the background logo loader.
///
/// Paging pushes only the channels it just revealed, so logos are fetched when
/// (and only when) the user asks to see them. A single consumer stream drains
/// the queue, which keeps the concurrency cap global instead of per page.
pub struct LogoPipeline {
    loader: Arc<LogoLoader>,
    queue: Mutex<VecDeque<Arc<ChannelItem>>>,
    notify: Notify,
}

impl LogoPipeline {
    pub fn new(loader: Arc<LogoLoader>) -> Self{
        Self{
            loader,
            queue: Mutex::new(Default::default()),
            notify: Notify::new(),
        }
    }

    /// Queues the logos of `items` that are not loaded and not already claimed.
    ///
    /// Safe to call with a whole window repeatedly: already-queued channels are
    /// filtered out by the per-item atomic claim.
    pub fn enqueue(&self, items: impl IntoIterator<Item = Arc<ChannelItem>>) {
        let mut queued = 0usize;
        {
            let mut queue = self
                .queue
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            for item in items {
                if !item.has_logo() || !item.try_mark_logo_requested() {
                    continue;
                }
                queue.push_back(item);
                queued += 1;
            }
        }
        if queued > 0 {
            self.notify.notify_one();
        }
    }

    /// Convenience helper for paging code paths.
    pub fn enqueue_all(&self, items: &ChannelItems) {
        self.enqueue(items.iter().cloned());
    }

    /// Waits for the next queued channel (used by [`Self::batch_stream`]).
    async fn pop(&self) -> Option<Arc<ChannelItem>> {
        loop {
            // Register interest *before* checking the queue to avoid the classic
            // Notify race where a push lands between the check and the await.
            let notified = self.notify.notified();
            if let Some(item) = self
                .queue
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .pop_front()
            {
                return Some(item);
            }
            notified.await;
        }
    }

    /// Streams decoded logos in progressive batches, bounded by [`CONCURRENCY`].
    ///
    /// The stream never ends on its own — it simply waits for more work, so it
    /// can be installed as a long-lived subscription.
    pub fn batch_stream(self: Arc<Self>) -> impl Stream<Item = LogoBatch> {
        let this = Arc::clone(&self);
        futures::stream::unfold(self, |pipeline| async move {
            let item = pipeline.pop().await?;
            Some((item, pipeline))
        })
            .map(move |item| {
                let loader = this.loader.clone();
                tokio::task::spawn_blocking(move || {
                    let image = loader.load(item.logo_url(), item.logo_format());
                    (item.id().to_string(), image)
                })
            })
            .buffer_unordered(CONCURRENCY)
            .map(|res| res.unwrap_or_else(|_| (String::new(), None)))
            .ready_chunks(BATCH_SIZE)
    }
}