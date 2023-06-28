use crate::{
    adg::{
        rels::{Fk, KeyPart, Pk},
        GSupport,
    },
    ed,
    util::vec_push_opt,
};

impl Pk for ed::EEffect {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.id]
    }
}

impl Fk for ed::EEffect {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        self.get_fks_from_mod_args("skillTypeID")
    }
    fn get_group_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        self.get_fks_from_mod_args("groupID")
    }
    fn get_attr_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        vec_push_opt(&mut vec, self.discharge_attr_id);
        vec_push_opt(&mut vec, self.duration_attr_id);
        vec_push_opt(&mut vec, self.range_attr_id);
        vec_push_opt(&mut vec, self.falloff_attr_id);
        vec_push_opt(&mut vec, self.tracking_attr_id);
        vec_push_opt(&mut vec, self.usage_chance_attr_id);
        vec_push_opt(&mut vec, self.resist_attr_id);
        vec.extend(self.get_fks_from_mod_args("modifyingAttributeID"));
        vec.extend(self.get_fks_from_mod_args("modifiedAttributeID"));
        vec
    }
    fn get_effect_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        // EffectStopper modifier type uses this argument
        self.get_fks_from_mod_args("effectID")
    }
}
impl ed::EEffect {
    fn get_fks_from_mod_args(&self, field: &'static str) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        for e_modifier in self.mods.iter() {
            for (k, v) in e_modifier.args.iter() {
                if let (true, &ed::EPrimitive::Int(fk)) = (k == field, v) {
                    vec.push(fk);
                }
            }
        }
        vec
    }
}
