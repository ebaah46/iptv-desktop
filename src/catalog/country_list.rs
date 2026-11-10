use qtbridge::qobject;

/// This is a singleton object that can be used anywhere withing
/// the application. It provides access to the Channel countries
/// list available in the applicaiton.
#[qobject(Singleton, Base = QListModel, ConvertToCamelCase)]
pub mod country_list {

    use crate::catalog::CountryItem;

    use crate::core::context;
    use qtbridge::QListModel;

    /// This is a QML singleton element that provides countries for channels
    /// available in the service. See CountryItem for more information on
    /// what each country provides.
    #[derive(Debug)]
    pub struct CountryList {
        items: Vec<CountryItem>,
    }

    impl Default for CountryList {
        fn default() -> Self {
            let mut items = vec![];
            items.extend(
                context::get()
                    .catalog()
                    .as_ref()
                    .get_countries()
                    .into_iter()
                    .map(|c| CountryItem {
                        code: c.code.clone(),
                        name: c.name.clone(),
                        languages: c.languages.clone(),
                        flag_url: c.flag_url.clone(),
                    }),
            );
            Self { items }
        }
    }

    impl QListModel for CountryList {
        type Item = CountryItem;
        fn len(&self) -> usize {
            self.items.len()
        }
        fn get(&self, i: usize) -> Option<&CountryItem> {
            self.items.get(i)
        }
    }
}
