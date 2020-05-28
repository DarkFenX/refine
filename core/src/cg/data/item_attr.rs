use crate::{
    consts::{attrs, units},
    defines::ReeInt,
    dh,
};

use super::{Fk, Pk, Support};

impl Pk for dh::ItemAttr {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.item_id, self.attr_id]
    }
}

impl Fk for dh::ItemAttr {
    fn get_item_fks(&self, support: &Support) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        vec.push(self.item_id);
        if let Some(v) = self.get_fk_from_val(units::ITEM_ID, &support) {
            vec.push(v);
        }
        vec
    }
    fn get_item_group_fks(&self, support: &Support) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        if let Some(v) = self.get_fk_from_val(units::GROUP_ID, &support) {
            vec.push(v);
        }
        vec
    }
    fn get_attr_fks(&self, support: &Support) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        vec.push(self.attr_id);
        if let Some(v) = self.get_fk_from_val(units::ATTR_ID, &support) {
            vec.push(v);
        }
        vec
    }
    fn get_effect_fks(&self, _: &Support) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_fighter_abil_fks(&self, _: &Support) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_buff_fks(&self, _: &Support) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        if attrs::BUFF_ID_ATTRS.contains(&self.attr_id) {
            vec.push(self.value.round() as ReeInt);
        }
        vec
    }
}
impl dh::ItemAttr {
    /// Receive unit ID, and if the attribute has such unit ID - return attribute value.
    fn get_fk_from_val(&self, unit: ReeInt, support: &Support) -> Option<ReeInt> {
        match support.attr_unit_map.get(&unit) {
            Some(&u) if u == unit => Some(self.value.round() as ReeInt),
            _ => None,
        }
    }
}
