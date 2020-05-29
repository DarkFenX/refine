use crate::{defines::ReeInt, dh, util};

use super::{Fk, Pk, Support};

impl Pk for dh::Effect {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}

impl Fk for dh::Effect {
    fn get_item_fks(&self, _: &Support) -> Vec<ReeInt> {
        self.get_fks_from_mod_args("skillTypeID")
    }
    fn get_item_group_fks(&self, _: &Support) -> Vec<ReeInt> {
        self.get_fks_from_mod_args("groupID")
    }
    fn get_attr_fks(&self, _: &Support) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        util::vec_push_opt(&mut vec, self.discharge_attr_id);
        util::vec_push_opt(&mut vec, self.duration_attr_id);
        util::vec_push_opt(&mut vec, self.range_attr_id);
        util::vec_push_opt(&mut vec, self.falloff_attr_id);
        util::vec_push_opt(&mut vec, self.tracking_attr_id);
        util::vec_push_opt(&mut vec, self.usage_chance_attr_id);
        util::vec_push_opt(&mut vec, self.resist_attr_id);
        vec.extend(self.get_fks_from_mod_args("modifyingAttributeID"));
        vec.extend(self.get_fks_from_mod_args("modifiedAttributeID"));
        vec
    }
    fn get_effect_fks(&self, _: &Support) -> Vec<ReeInt> {
        // EffectStopper modifier type uses this argument
        self.get_fks_from_mod_args("effectID")
    }
}
impl dh::Effect {
    fn get_fks_from_mod_args(&self, field: &'static str) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        for modifier in self.mods.iter() {
            for (k, v) in modifier.args.iter() {
                if let (true, &dh::Primitive::Int(fk)) = (k == field, v) {
                    vec.push(fk);
                }
            }
        }
        vec
    }
}
