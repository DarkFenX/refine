use std::collections::HashMap;

use crate::defines::{Id, AttrVal};
use crate::eve_type::Effect;

pub struct Item {
    pub id: Id,
    pub group_id: Id,
    pub category_id: Id,
    pub attrs: HashMap<Id, AttrVal>,
    pub effects: HashMap<Id, AttrVal>,
    pub default_effect: Option<Effect>,
}

impl Item {
    pub fn new(
        id: Id,
        group_id: Id,
        category_id: Id,
        attrs: HashMap<Id, AttrVal>,
        effects: HashMap<Id, AttrVal>,
        default_effect: Option<Effect>,
    ) -> Item {
        Item {
            id,
            group_id,
            category_id,
            attrs,
            effects,
            default_effect,
        }
    }
}
