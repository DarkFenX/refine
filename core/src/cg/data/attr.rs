use crate::{
    consts::{attrs, units},
    defines::ReeInt,
    dh, util,
};

use super::{Fk, Pk, Support};

impl Pk for dh::Attr {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}

impl Fk for dh::Attr {
    fn get_item_fks(&self, _: &Support) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        if let Some(v) = self.get_fk_from_defval(units::ITEM_ID) {
            vec.push(v);
        }
        vec
    }
    fn get_item_group_fks(&self, _: &Support) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        if let Some(v) = self.get_fk_from_defval(units::GROUP_ID) {
            vec.push(v);
        }
        vec
    }
    fn get_attr_fks(&self, _: &Support) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        util::vec_push_opt(&mut vec, self.max_attr_id);
        if let Some(v) = self.get_fk_from_defval(units::ATTR_ID) {
            vec.push(v);
        }
        vec
    }
    fn get_buff_fks(&self, _: &Support) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        if let (true, Some(dv)) = (attrs::BUFF_ID_ATTRS.contains(&self.id), self.get_nonzero_defval()) {
            vec.push(dv.round() as ReeInt);
        }
        vec
    }
}
impl dh::Attr {
    /// Receive unit ID, and if the attribute has such unit ID - push its default value to the
    /// vector.
    fn get_fk_from_defval(&self, unit: ReeInt) -> Option<ReeInt> {
        match (self.unit_id, self.get_nonzero_defval()) {
            (Some(u), Some(dv)) if u == unit => Some(dv.round() as ReeInt),
            _ => None,
        }
    }
}
