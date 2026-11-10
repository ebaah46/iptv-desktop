use qtbridge::qobject;

/// This is a singleton object that can be used anywhere withing
/// the application. It provides access to the Channel categories
/// list available in the applicaiton.
#[qobject(Singleton, Base = QListModel, ConvertToCamelCase)]
pub mod category_list {

    use crate::catalog::CategoryItem;

    use crate::core::context;
    use qtbridge::QListModel;

    /// This is a QML singleton element that provides categories for channels
    /// available in the service. See CategoryItem for more information on
    /// what each category provides.
    pub struct CategoryList {
        items: Vec<CategoryItem>,
    }

    impl Default for CategoryList {
        fn default() -> Self {
            let mut items = vec![];
            items.extend(
                context::get()
                    .catalog()
                    .as_ref()
                    .get_categories()
                    .into_iter()
                    .map(|c| CategoryItem {
                        id: c.id.clone(),
                        name: c.name.clone(),
                        description: c.description.clone(),
                    }),
            );
            Self { items }
        }
    }

    impl QListModel for CategoryList {
        type Item = CategoryItem;
        fn len(&self) -> usize {
            self.items.len()
        }
        fn get(&self, i: usize) -> Option<&CategoryItem> {
            self.items.get(i)
        }
    }
}
