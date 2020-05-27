use std::collections::HashMap;

use crate::ct::Effect;
use crate::defines::{ReeFloat, ReeInt};

pub struct Item<'a> {
    pub id: ReeInt,
    pub group_id: ReeInt,
    pub category_id: ReeInt,
    pub attrs: HashMap<ReeInt, ReeFloat>,
    pub effects: Vec<&'a Effect>,
    pub default_effect: Option<&'a Effect>,
    pub skillreqs: HashMap<ReeInt, ReeInt>,
}
impl<'a> Item<'a> {
    pub fn new(
        id: ReeInt,
        group_id: ReeInt,
        category_id: ReeInt,
        attrs: HashMap<ReeInt, ReeFloat>,
        effects: Vec<&'a Effect>,
        default_effect: Option<&'a Effect>,
        skillreqs: HashMap<ReeInt, ReeInt>,
    ) -> Item<'a> {
        Item {
            id,
            group_id,
            category_id,
            attrs,
            effects,
            default_effect,
            skillreqs,
        }
    }
}
