use libcore::domain::Country;
use slint::{ModelRc, SharedString, VecModel};
use crate::{Country as CountryItem, };

pub fn to_country(country: Country) -> CountryItem{
    let to_model = |vec: Vec<String>| -> ModelRc<SharedString> {
        let items: Vec<SharedString> = vec.into_iter().map(SharedString::from).collect();
        ModelRc::new(VecModel::from(items))
    };
    CountryItem{
        flag_url: country.flag_url.into(),
        code: country.code.into(),
        languages: to_model(country.languages),
        name: country.name.into(),
    }
}