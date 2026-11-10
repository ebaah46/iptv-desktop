use libcore::domain::{Channel, Logo};
use qtbridge::QModelItem;
use std::collections::HashMap;
use std::sync::Arc;

/// A channel together with its associated logo, as consumed by the UI layer.
/// This object is used by the ChannelList.
/// We prefer this approach of creating ModelItem elements because, Channel data
/// could exist beyond tens of thousands. So having multiple copies of channellists
/// is not cheap.
#[derive(Debug, Default, Clone)]
pub struct ChannelItem {
    channel: Option<Arc<Channel>>,
    logo: Option<Arc<Logo>>,
    has_logo: bool,
}

impl ChannelItem {
    /// Builds a channel item from the domain channel and its logo.
    pub fn new(channel: Arc<Channel>, logo: Arc<Logo>) -> Self {
        Self::from_parts(channel, Some(logo))
    }

    /// Pairs every channel with its logo (when one exists) in `O(channels + logos)`.
    ///
    /// Logos are indexed by channel id once, so each channel costs a single
    /// hash lookup instead of a nested scan. Channels without a matching logo
    /// are still included, with `logo: None` and `has_logo == false`.
    pub fn from_channels(
        channels: impl IntoIterator<Item = Arc<Channel>>,
        logos: &[Arc<Logo>],
    ) -> Vec<Self> {
        let mut logo_by_channel: HashMap<&str, &Arc<Logo>> = HashMap::with_capacity(logos.len());
        for logo in logos {
            // Last logo wins if a channel has duplicates, matching the old
            // `HashMap::insert` behaviour in the Iced frontend.
            logo_by_channel.insert(logo.channel.as_str(), logo);
        }

        let channels = channels.into_iter();
        let (lower_bound, _) = channels.size_hint();
        let mut items = Vec::with_capacity(lower_bound);

        for channel in channels {
            let logo = logo_by_channel.get(channel.id.as_str()).copied().cloned();
            let item = match logo {
                Some(logo) => Self::new(channel, logo),
                None => Self::from_parts(channel, None),
            };
            items.push(item);
        }

        items
    }

    fn from_parts(channel: Arc<Channel>, logo: Option<Arc<Logo>>) -> Self {
        let has_logo = logo.as_ref().is_some_and(|logo| !logo.url.is_empty());

        Self {
            channel: Some(channel),
            logo,
            has_logo,
        }
    }
}

impl QModelItem for ChannelItem {
    // 1. Map slot types to match the exact types stored in your Arc fields
    // so we can return references directly without cloning.
    type T0 = String; // id
    type T1 = String; // name
    type T2 = Vec<String>; // alt_names
    type T3 = Vec<String>; // category_ids
    type T4 = String; // country_code
    type T5 = bool; // is_nsfw
    type T6 = String; // website
    type T7 = String; // network
    type T8 = String; // logo_url
    type T9 = String; // logo_format
    type T10 = u32; // logo_width
    type T11 = u32; // logo_height
    type T12 = bool; // has_logo (cached)
    type T13 = ();
    type T14 = ();

    // 2. Zero-allocation getters: return references straight into the Arc<Channel> / Arc<Logo>
    fn get0(&self) -> Option<&Self::T0> {
        self.channel.as_ref().map(|c| &c.id)
    }
    fn get1(&self) -> Option<&Self::T1> {
        self.channel.as_ref().map(|c| &c.name)
    }
    fn get2(&self) -> Option<&Self::T2> {
        self.channel.as_ref().map(|c| &c.alt_names)
    }
    fn get3(&self) -> Option<&Self::T3> {
        self.channel.as_ref().map(|c| &c.category_ids)
    }
    fn get4(&self) -> Option<&Self::T4> {
        self.channel.as_ref().map(|c| &c.country_code)
    }
    fn get5(&self) -> Option<&Self::T5> {
        self.channel.as_ref().map(|c| &c.is_nsfw)
    }
    fn get6(&self) -> Option<&Self::T6> {
        self.channel.as_ref().map(|c| &c.website)
    }
    fn get7(&self) -> Option<&Self::T7> {
        self.channel.as_ref().map(|c| &c.network)
    }
    fn get8(&self) -> Option<&Self::T8> {
        self.logo.as_ref().map(|l| &l.url)
    }
    fn get9(&self) -> Option<&Self::T9> {
        self.logo.as_ref().map(|l| &l.format)
    }
    fn get10(&self) -> Option<&Self::T10> {
        self.logo.as_ref().map(|l| &l.width)
    }
    fn get11(&self) -> Option<&Self::T11> {
        self.logo.as_ref().map(|l| &l.height)
    }

    fn get12(&self) -> Option<&Self::T12> {
        Some(&self.has_logo)
    }

    fn get13(&self) -> Option<&Self::T13> {
        None
    }
    fn get14(&self) -> Option<&Self::T14> {
        None
    }

    // The role id MUST match the `getN` slot index: `QModelItem::get_role`
    // dispatches on 0..=14.
    fn role_names() -> HashMap<i32, String> {
        let mut map = HashMap::new();
        map.insert(0, "id".to_string());
        map.insert(1, "name".to_string());
        map.insert(2, "altNames".to_string());
        map.insert(3, "categoryIds".to_string());
        map.insert(4, "countryCode".to_string());
        map.insert(5, "isNsfw".to_string());
        map.insert(6, "website".to_string());
        map.insert(7, "network".to_string());
        map.insert(8, "logoUrl".to_string());
        map.insert(9, "logoFormat".to_string());
        map.insert(10, "logoWidth".to_string());
        map.insert(11, "logoHeight".to_string());
        map.insert(12, "hasLogo".to_string());
        map
    }
}
