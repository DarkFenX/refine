use crate::defines::{ReeFloat, ReeInt};

pub struct Attribute {
    pub id: ReeInt,
    pub max_attr_id: Option<ReeInt>,
    pub default_value: Option<ReeFloat>,
    pub high_is_good: bool,
    pub stackable: bool,
}

impl Attribute {
    pub fn new(
        id: ReeInt,
        max_attr_id: Option<ReeInt>,
        default_value: Option<ReeFloat>,
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
