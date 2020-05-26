use std::collections::HashMap;

use crate::ct::Effect;
use crate::defines::{ReeFloat, ReeInt};

pub struct Item {
    pub id: ReeInt,
    pub group_id: ReeInt,
    pub category_id: ReeInt,
    pub attrs: HashMap<ReeInt, ReeFloat>,
    pub effects: HashMap<ReeInt, ReeFloat>,
    pub default_effect: Option<Effect>,
}

impl Item {
    pub fn new(
        id: ReeInt,
        group_id: ReeInt,
        category_id: ReeInt,
        attrs: HashMap<ReeInt, ReeFloat>,
        effects: HashMap<ReeInt, ReeFloat>,
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
