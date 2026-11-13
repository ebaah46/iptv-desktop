use qtbridge::qobject;

/// This is a container type for all channels used
/// in the application.
#[qobject(Base = QListModel, ConvertToCamelCase)]
pub mod channel_list {
    use crate::{catalog::ChannelItem, core::context};
    use libcore::services::CatalogQuery;
    use qtbridge::{QListModel, QListModelBase};
    use std::collections::{HashMap, HashSet};

    /// Number of channels revealed per page.
    const PAGE_SIZE: usize = 15;

    /// ChannelList QML element the provides channels available in the system.
    /// The internal channel element used here is ChannelItem.
    /// See ChannelItem for more information on the channel details.
    #[derive(Default)]
    pub struct ChannelList {
        /// Every item matching the current filter.
        all_items: Vec<ChannelItem>,
        /// The currently revealed window (first `items.len()` of `all_items`).
        items: Vec<ChannelItem>,
        /// Staging buffer for `reset_unnotified`.
        pending: Vec<ChannelItem>,
        /// Whether the current filter is restricted to favorites.
        favorites_only: bool,
        /// Number of favorite channels stored by the service.
        favorite_count: i32,
        /// Last filter arguments, kept so favorites changes can re-apply them.
        last_query: String,
        last_category_id: String,
        last_country_code: String,
        last_language_code: String,
    }

    impl QListModel for ChannelList {
        type Item = ChannelItem;

        fn len(&self) -> usize {
            self.items.len()
        }
        fn get(&self, index: usize) -> Option<&ChannelItem> {
            self.items.get(index)
        }

        fn set_unnotified(&mut self, index: usize, value: ChannelItem) -> bool {
            match self.items.get_mut(index) {
                Some(item) => {
                    *item = value;
                    true
                }
                None => false,
            }
        }

        fn push_unnotified(&mut self, value: ChannelItem) {
            self.items.push(value);
        }

        fn remove_unnotified(&mut self, index: usize) -> ChannelItem {
            self.items.remove(index)
        }

        fn reset_unnotified(&mut self) {
            self.items = std::mem::take(&mut self.pending);
        }
    }

    impl ChannelList {
        qproperty!(
            "channelCount",
            Read = channel_count,
            Notify = channel_count_changed
        );
        qproperty!(
            "loadedCount",
            Read = loaded_count,
            Notify = loaded_count_changed
        );
        qproperty!("hasMore", Read = has_more, Notify = has_more_changed);
        qproperty!(
            "favoriteCount",
            Read = favorite_count,
            Notify = favorite_count_changed
        );

        // Total number of channels matching the current filter.
        pub fn channel_count(&self) -> i32 {
            self.all_items.len() as i32
        }

        // Number of channels currently materialised in the view.
        pub fn loaded_count(&self) -> i32 {
            self.items.len() as i32
        }

        // Whether another page is available.
        pub fn has_more(&self) -> bool {
            self.items.len() < self.all_items.len()
        }

        // Number of channels currently marked as favorites.
        pub fn favorite_count(&self) -> i32 {
            self.favorite_count
        }

        #[qsignal]
        fn channel_count_changed(&mut self);

        #[qsignal]
        fn loaded_count_changed(&mut self);

        #[qsignal]
        fn has_more_changed(&mut self);

        #[qsignal]
        fn favorite_count_changed(&mut self);

        /// Emitted after a favorite is added or removed, so other views can
        /// refresh their own heart state.
        #[qsignal]
        fn favorites_changed(&mut self);

        // Empty string = no constraint. QML owns the filter state and passes it in.
        #[qslot]
        fn apply_filters(
            &mut self,
            query: String,
            category_id: String,
            country_code: String,
            language_code: String,
            favorites_only: bool,
        ) {
            self.last_query = query.clone();
            self.last_category_id = category_id.clone();
            self.last_country_code = country_code.clone();
            self.last_language_code = language_code.clone();
            self.favorites_only = favorites_only;

            let catalog = context::get().catalog();
            let favorites = context::get().favorites();
            let favorite_ids = favorites.get_favorite_ids();
            let favorite_set: HashSet<&str> = favorite_ids.iter().map(String::as_str).collect();
            self.favorite_count = favorite_ids.len() as i32;

            // One pass over the catalog for search + category + country +
            // language. Empty strings are ignored by `CatalogQuery`.
            let mut catalog_query = CatalogQuery::new()
                .search(query)
                .category(category_id)
                .country(country_code)
                .language(language_code);

            if favorites_only {
                // `CatalogQuery::channels` treats an empty selection as "no
                // filter", so handle the no-favorites case explicitly instead
                // of returning the whole catalog.
                if favorite_ids.is_empty() {
                    self.clear();
                    return;
                }
                catalog_query = catalog_query.channels(favorite_ids.clone());
            }

            let mut channels = catalog.query(&catalog_query);
            if favorites_only {
                // `CatalogQuery` preserves catalog order; re-order so the
                // most recently favorited channels come first.
                let favorite_order: HashMap<&str, usize> = favorite_ids
                    .iter()
                    .enumerate()
                    .map(|(index, id)| (id.as_str(), index))
                    .collect();
                channels.sort_by_key(|channel| {
                    favorite_order
                        .get(channel.id.as_str())
                        .copied()
                        .unwrap_or(usize::MAX)
                });
            }
            let logos = catalog.get_logos();

            // Build the full filtered list once (cheap Arc clones), but only
            // expose the first page to the view.
            self.all_items = ChannelItem::from_channels(channels, &logos, &favorite_set);
            self.pending = self.all_items.iter().take(PAGE_SIZE).cloned().collect();
            self.reset();
            self.channel_count_changed();
            self.loaded_count_changed();
            self.has_more_changed();
            self.favorite_count_changed();
        }

        /// Reveals the next page of the current filter.
        #[qslot]
        fn load_more(&mut self) {
            let start = self.items.len();
            let end = (start + PAGE_SIZE).min(self.all_items.len());
            if start >= end {
                return;
            }

            // Clone the page before mutating `items`; `push` borrows `self`.
            let next_page: Vec<ChannelItem> = self.all_items[start..end].to_vec();
            for item in next_page {
                self.push(item);
            }

            self.loaded_count_changed();
            self.has_more_changed();
        }

        /// Adds or removes a favorite and updates the visible model in place.
        #[qslot]
        fn toggle_favorite(&mut self, channel_id: String) {
            let favorites = context::get().favorites();
            match favorites.toggle(&channel_id) {
                Ok(is_favorite) => {
                    self.favorite_count = favorites.get_favorite_ids().len() as i32;
                    self.favorite_count_changed();

                    if self.favorites_only && !is_favorite {
                        // A channel removed from the favorites view should
                        // disappear immediately.
                        self.remove_channel(&channel_id);
                    } else {
                        self.update_favorite_flag(&channel_id, is_favorite);
                    }
                    self.favorites_changed();
                }
                Err(error) => log::error!("Failed to toggle favorite {channel_id}: {error}"),
            }
        }

        /// Re-reads the favorite set after another view changed it.
        ///
        /// In favorites-only mode the filter is re-applied so added favorites
        /// appear and removed ones disappear. Other views only need their
        /// heart state updated, which avoids resetting pagination.
        #[qslot]
        fn refresh_favorites(&mut self) {
            if self.favorites_only {
                self.apply_filters(
                    self.last_query.clone(),
                    self.last_category_id.clone(),
                    self.last_country_code.clone(),
                    self.last_language_code.clone(),
                    true,
                );
                return;
            }

            let favorites = context::get().favorites();
            let favorite_ids = favorites.get_favorite_ids();
            let favorite_set: HashSet<&str> = favorite_ids.iter().map(String::as_str).collect();
            self.favorite_count = favorite_ids.len() as i32;
            self.favorite_count_changed();

            for item in self.all_items.iter_mut() {
                let is_favorite = item.id().is_some_and(|id| favorite_set.contains(id));
                item.set_favorite(is_favorite);
            }

            for index in 0..self.items.len() {
                let is_favorite = self.items[index]
                    .id()
                    .is_some_and(|id| favorite_set.contains(id));
                if self.items[index].is_favorite() != is_favorite {
                    self.items[index].set_favorite(is_favorite);
                    let updated = self.items[index].clone();
                    self.set(index, updated);
                }
            }
        }

        /// Clears the model and refreshes every derived property.
        fn clear(&mut self) {
            self.all_items.clear();
            self.pending.clear();
            self.reset();
            self.channel_count_changed();
            self.loaded_count_changed();
            self.has_more_changed();
            self.favorite_count_changed();
        }

        /// Updates the cached favorite flag for a channel in place.
        fn update_favorite_flag(&mut self, channel_id: &str, is_favorite: bool) {
            for item in self.all_items.iter_mut() {
                if item.id() == Some(channel_id) {
                    item.set_favorite(is_favorite);
                }
            }

            for index in 0..self.items.len() {
                if self.items[index].id() == Some(channel_id) {
                    self.items[index].set_favorite(is_favorite);
                    let updated = self.items[index].clone();
                    self.set(index, updated);
                }
            }
        }

        /// Removes a channel from both the backing list and the visible window.
        fn remove_channel(&mut self, channel_id: &str) {
            self.all_items.retain(|item| item.id() != Some(channel_id));

            if let Some(index) = self
                .items
                .iter()
                .position(|item| item.id() == Some(channel_id))
            {
                self.remove(index);
            }

            self.channel_count_changed();
            self.loaded_count_changed();
            self.has_more_changed();
        }
    }
}
