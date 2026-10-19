use std::sync::Arc;

use libcore::domain::{Channel, Logo};

/// A channel together with its associated logo, as consumed by the UI layer.
///
/// UI components interact only through the typed getters below and never touch
/// the underlying domain `Channel`/`Logo` types directly.
#[derive(Debug, Clone)]
pub struct ChannelItem {
    channel: Arc<Channel>,
    logo: Arc<Logo>,
    /// Decoded image handle for the channel logo (None until fetched/decoded).
    logo_handle: Option<iced::widget::image::Handle>,
}

/// A collection of channel items used across the UI.
pub type ChannelItems = Vec<Arc<ChannelItem>>;

impl ChannelItem {
    /// Builds a channel item from the domain channel and its logo.
    pub fn new(channel: Arc<Channel>, logo: Arc<Logo>) -> Self {
        Self {
            channel,
            logo,
            logo_handle: None,
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
    pub fn logo_handle(&self) -> &Option<iced::widget::image::Handle> {
        &self.logo_handle
    }

    /// Attaches a decoded image handle for this channel's logo.
    pub fn set_logo_handle(mut self, handle: iced::widget::image::Handle) {
        self.logo_handle = Some(handle);
    }

    /// Returns a new item identical to this one but carrying the given logo handle.
    pub fn with_logo_handle(&self, handle: iced::widget::image::Handle) -> Self {
        Self {
            channel: self.channel.clone(),
            logo: self.logo.clone(),
            logo_handle: Some(handle),
        }
    }
}
