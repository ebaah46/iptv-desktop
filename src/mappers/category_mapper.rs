use libcore::domain::Category;
use crate::{Category as CategoryItem};
pub fn to_category(category: Category) -> CategoryItem {
    CategoryItem{
        description: category.description.into(),
        id: category.id.into(),
        name: category.name.into(),
    }
}