use crate::defines::{Id, AttrVal};

pub struct Attribute {
    pub id: Id,
    pub max_attr_id: Option<Id>,
    pub default_value: Option<AttrVal>,
    pub high_is_good: bool,
    pub stackable: bool,
}

impl Attribute {
    pub fn new(
        id: Id,
        max_attr_id: Option<Id>,
        default_value: Option<AttrVal>,
        high_is_good: bool,
        stackable: bool,
    ) -> Attribute {
        Attribute {
            id,
            max_attr_id,
            default_value,
            high_is_good,
            stackable,
        }
    }
}
