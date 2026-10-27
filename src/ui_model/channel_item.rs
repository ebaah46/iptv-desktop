use std::sync::{Arc, OnceLock};
use std::sync::atomic::{AtomicBool, Ordering};
use libcore::domain::{Channel, Logo};

/// A decoded channel logo ready to be rendered.
///
/// Keeps the two distinct handle types iced exposes — raster images (png, jpg,
/// gif, webp, …) and vector images (SVG) — so the UI can render whichever the
/// source served.
#[derive(Debug, Clone)]
pub enum LogoImage {
    /// A raster image decoded at render time from raw bytes.
    Raster(iced::widget::image::Handle),
    /// A vector (SVG) image decoded at render time from raw bytes.
    Svg(iced::widget::svg::Handle),
}

/// A channel together with its associated logo, as consumed by the UI layer.
///
/// UI components interact only through the typed getters below and never touch
/// the underlying domain `Channel`/`Logo` types directly.
#[derive(Debug,)]
pub struct ChannelItem {
    channel: Arc<Channel>,
    logo: Arc<Logo>,
    /// Decoded logo handle for the channel (None until fetched/decoded).
    logo_handle: Arc<OnceLock<LogoImage>>,
    /// This flag sets whether a logo has been requested/downloaded already.
    logo_requested: AtomicBool,
}

/// A collection of channel items used across the UI.
pub type ChannelItems = Vec<Arc<ChannelItem>>;

impl ChannelItem {
    /// Builds a channel item from the domain channel and its logo.
    pub fn new(channel: Arc<Channel>, logo: Arc<Logo>) -> Self {
        Self {
            channel,
            logo,
            logo_handle: Arc::new(OnceLock::new()),
            logo_requested: AtomicBool::new(false),
        }
    }

    /// The item's channel id.
    pub fn id(&self) -> &str {
        &self.channel.id
    }

    /// The item's channel name.
    pub fn name(&self) -> &str {
        &self.channel.name
    }

    /// Alternate names used when searching.
    pub fn alt_names(&self) -> &Vec<String> {
        &self.channel.alt_names
    }

    /// Category ids this channel belongs to.
    pub fn category_ids(&self) -> &Vec<String> {
        &self.channel.category_ids
    }

    /// Country code of the channel.
    pub fn country_code(&self) -> &str {
        &self.channel.country_code
    }

    /// Whether the channel is marked NSFW.
    pub fn is_nsfw(&self) -> bool {
        self.channel.is_nsfw
    }

    /// Channel website url.
    pub fn website(&self) -> &str {
        &self.channel.website
    }

    /// Channel network name.
    pub fn network(&self) -> &str {
        &self.channel.network
    }

    /// URL where the channel logo is hosted.
    pub fn logo_url(&self) -> &str {
        &self.logo.url
    }

    /// Logo image format (e.g. "png", "svg").
    pub fn logo_format(&self) -> &str {
        &self.logo.format
    }

    /// Logo width in pixels.
    pub fn logo_width(&self) -> u32 {
        self.logo.width
    }

    /// Logo height in pixels.
    pub fn logo_height(&self) -> u32 {
        self.logo.height
    }

    /// Whether the item carries a non-empty logo url.
    pub fn has_logo(&self) -> bool {
        !self.logo_url().is_empty()
    }

    /// The decoded image handle for this channel's logo, if loaded.
    pub fn logo_handle(&self) -> Option<&LogoImage> {
        self.logo_handle.get()
    }

    /// Attaches a decoded image handle for this channel's logo.
    pub fn set_logo_handle(&self, handle: LogoImage) -> bool {
        self.logo_handle.set(handle).is_ok()
    }

    pub fn is_logo_loaded(&self) -> bool{
        self.logo_handle.get().is_some()
    }

    /// Atomically claims the logo load for this channel. Returns `false` when
    /// the logo is already loaded or another request already claimed it.
    pub fn try_mark_logo_requested(&self) -> bool {
        !self.is_logo_loaded()
            && self
            .logo_requested
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    /// Releases a claim so a load that failed can be retried on a later page.
    pub fn release_logo_request(&self) {
        self.logo_requested.store(false, Ordering::Release);
    }
}
