use crate::{
    ad::{
        AEffectId,
        generator::{
            GSupport,
            rels::{Fk, KeyPart, Pk},
        },
    },
    ed::{EEffect, EPrimitive},
    nd::N_EFFECT_MAP,
    util::vec_push_opt,
};

impl Pk for EEffect {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.id]
    }
}

impl Fk for EEffect {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        self.get_fks_from_mod_args("skillTypeID")
    }
    fn get_group_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        self.get_fks_from_mod_args("groupID")
    }
    fn get_item_list_fks(&self, g_supp: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        if let Some(effect_buff) = g_supp.eff_buff_map.get(&self.id) {
            vec.extend(effect_buff.iter_e_item_list_ids());
        }
        vec
    }
    fn get_attr_fks(&self, g_supp: &GSupport) -> Vec<KeyPart> {
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
        if let Some(effect_buff) = g_supp.eff_buff_map.get(&self.id) {
            vec.extend(effect_buff.iter_e_attr_ids());
        }
        // Hardcoded charge info can reference attributes
        // TODO: check if this logic should be moved elsewhere
        if let Some(n_effect) = N_EFFECT_MAP.get(&AEffectId::Dogma(self.id)) {
            vec.extend(n_effect.extract_e_attr_ids());
        }
        vec
    }
    fn get_effect_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        // EffectStopper modifier type uses this argument
        self.get_fks_from_mod_args("effectID")
    }
    fn get_buff_fks(&self, g_supp: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        if let Some(effect_buff) = g_supp.eff_buff_map.get(&self.id) {
            vec.extend(effect_buff.iter_e_buff_ids());
        }
        vec
    }
}
impl EEffect {
    fn get_fks_from_mod_args(&self, field: &'static str) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        for e_modifier in self.mods.iter() {
            for (k, v) in e_modifier.args.iter() {
                if let (true, &EPrimitive::Int(fk)) = (k == field, v) {
                    vec.push(fk);
                }
            }
        }
        vec
    }
}
