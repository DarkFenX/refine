use crate::{defines::ReeInt, dh};

use super::{Fk, Pk, Support};

impl Pk for dh::Buff {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}

impl Fk for dh::Buff {
    fn get_item_fks(&self, _: &Support) -> Vec<ReeInt> {
        self.locsrq_mods.iter().map(|v| v.skill_id).collect()
    }
    fn get_group_fks(&self, _: &Support) -> Vec<ReeInt> {
        self.locgroup_mods.iter().map(|v| v.group_id).collect()
    }
    fn get_attr_fks(&self, _: &Support) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        vec.extend(self.item_mods.iter().map(|v| v.attr_id));
        vec.extend(self.loc_mods.iter().map(|v| v.attr_id));
        vec.extend(self.locgroup_mods.iter().map(|v| v.attr_id));
        vec.extend(self.locsrq_mods.iter().map(|v| v.attr_id));
        vec
    }
}
