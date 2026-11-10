use qtbridge::QModelItem;

/// A category item as used across the UI
/// This object is used by the CategoryList.
#[derive(QModelItem, Debug, Clone, Default)]
pub struct CategoryItem {
    pub id: String,
    pub name: String,
    pub description: String,
}
