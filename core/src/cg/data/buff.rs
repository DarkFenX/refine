use crate::{defines::ReeInt, dh};

use super::{Fk, Pk};

impl Pk for dh::Buff {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}

impl Fk for dh::Buff {
    fn get_item_fks(&self) -> Vec<ReeInt> {
        self.locsrq_mods.iter().map(|v| v.skill_id).collect()
    }
    fn get_item_group_fks(&self) -> Vec<ReeInt> {
        self.locgroup_mods.iter().map(|v| v.group_id).collect()
    }
    fn get_attr_fks(&self) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        vec.extend(self.item_mods.iter().map(|v| v.attr_id));
        vec.extend(self.loc_mods.iter().map(|v| v.attr_id));
        vec.extend(self.locgroup_mods.iter().map(|v| v.attr_id));
        vec.extend(self.locsrq_mods.iter().map(|v| v.attr_id));
        vec
    }
    fn get_effect_fks(&self) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_fighter_abil_fks(&self) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_buff_fks(&self) -> Vec<ReeInt> {
        Vec::new()
    }
}
