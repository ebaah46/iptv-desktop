use std::sync::Arc;
use libcore::domain::Country;


/// A country item as used across the UI
///
#[derive(Debug)]
pub struct CountryItem {
    country: Arc<Country>
}

impl CountryItem {
    pub fn new(country: Arc<Country>) -> Self {
        Self{ country }
    }

    pub fn code(&self) -> &str{
        self.country.code.as_str()
    }

    pub fn name(&self) -> &str {
        self.country.name.as_str()
    }

    pub fn languages(&self) -> &[String] {
        self.country.languages.as_slice()
    }

    pub fn flag(&self) -> &str {
        self.country.flag_url.as_str()
    }
}

/// A collection of country items used across the UI
pub type CountryItems = Vec<Arc<CountryItem>>;