use qtbridge::qobject;

/// This is a container type for all channels used
/// in the application.
#[qobject(Base = QListModel, ConvertToCamelCase)]
pub mod channel_list {
    use crate::{catalog::ChannelItem, core::context};
    use libcore::services::CatalogQuery;
    use qtbridge::{QListModel, QListModelBase};

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
    }

    impl QListModel for ChannelList {
        type Item = ChannelItem;

        fn len(&self) -> usize {
            self.items.len()
        }
        fn get(&self, index: usize) -> Option<&ChannelItem> {
            self.items.get(index)
        }

        fn push_unnotified(&mut self, value: ChannelItem) {
            self.items.push(value);
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

        #[qsignal]
        fn channel_count_changed(&mut self);

        #[qsignal]
        fn loaded_count_changed(&mut self);

        #[qsignal]
        fn has_more_changed(&mut self);

        // Empty string = no constraint. QML owns the filter state and passes it in.
        #[qslot]
        fn apply_filters(
            &mut self,
            query: String,
            category_id: String,
            country_code: String,
            language_code: String,
        ) {
            let catalog = context::get().catalog();

            // One pass over the catalog for search + category + country +
            // language. Empty strings are ignored by `CatalogQuery`.
            let catalog_query = CatalogQuery::new()
                .search(query)
                .category(category_id)
                .country(country_code)
                .language(language_code);
            let channels = catalog.query(&catalog_query);
            let logos = catalog.get_logos();

            // Build the full filtered list once (cheap Arc clones), but only
            // expose the first page to the view.
            self.all_items = ChannelItem::from_channels(channels, &logos);
            self.pending = self.all_items.iter().take(PAGE_SIZE).cloned().collect();
            self.reset();
            self.channel_count_changed();
            self.loaded_count_changed();
            self.has_more_changed();
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
    }
}
