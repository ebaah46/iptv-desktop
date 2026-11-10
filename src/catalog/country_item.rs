use qtbridge::QModelItem;

/// A country item as used across the UI
/// This object is used by the CountryList.
#[derive(QModelItem, Debug, Clone, Default)]
pub struct CountryItem {
    pub code: String,
    pub name: String,
    pub languages: Vec<String>,
    pub flag_url: String,
}
