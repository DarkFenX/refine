use crate::{
    consts::{attrs, units},
    defines::ReeInt,
    dh, util,
};

use super::{Fk, Pk};

impl Pk for dh::Attr {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}

impl Fk for dh::Attr {
    fn get_item_fks(&self) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        // When the attribute refers an item type, grab its default value
        self.push_defval_id(&mut vec, units::ITEM_ID);
        vec
    }
    fn get_item_group_fks(&self) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        // When the attribute refers a group, grab its default value
        self.push_defval_id(&mut vec, units::GROUP_ID);
        vec
    }
    fn get_attr_fks(&self) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        util::vec_push_opt(&mut vec, self.max_attr_id);
        // When the attribute refers another attribute, grab its default value
        self.push_defval_id(&mut vec, units::ATTR_ID);
        vec
    }
    fn get_effect_fks(&self) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_fighter_abil_fks(&self) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_buff_fks(&self) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        if let (true, Some(dv)) = (attrs::BUFF_ID_ATTRS.contains(&self.id), self.get_nonzero_defval()) {
            vec.push(dv.round() as ReeInt)
        }
        vec
    }
}
impl dh::Attr {
    fn push_defval_id(&self, vec: &mut Vec<ReeInt>, unit: ReeInt) {
        if let (Some(u), Some(dv)) = (self.unit_id, self.get_nonzero_defval()) {
            // Ignore default values of 0.0, since that's the placeholder value for the field
            if u == unit {
                vec.push(dv.round() as ReeInt)
            }
        }
    }
}
