use libcore::domain::Channel;
use slint::{ModelRc, SharedString, ToSharedString, VecModel};
use crate::{Channel as ChannelItem};

pub fn to_channel_info(channel: Channel) -> ChannelItem {
    let to_model = |vec: Vec<String>| -> ModelRc<SharedString> {
        let items: Vec<SharedString> = vec.into_iter().map(SharedString::from).collect();
        ModelRc::new(VecModel::from(items))
    };
    ChannelItem{
        alt_names: to_model(channel.alt_names),
        category_ids: to_model(channel.category_ids),
        closed: channel.closed.map(|d| d.to_shared_string()).unwrap_or_default(),
        country_code: channel.country_code.into(),
        current_program: Default::default(),
        id: channel.id.into(),
        is_nsfw: channel.is_nsfw,
        launched: channel.launched.map(|l| l.to_shared_string()).unwrap_or_default(),
        logo_color: Default::default(),
        logo_text: Default::default(),
        name: channel.name.into(),
        network: channel.network.into(),
        progress: 0.0,
        website: channel.website.into(),
    }
}