use crate::{
    ad::generator::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::{EEffect, EPrimitive},
    nd::N_EFFECT_MAP,
    util::vec_push_opt,
};

impl Pk for EEffect {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.id.into()]
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
        let mut fks = Vec::new();
        if let Some(effect_buff) = g_supp.eff_buff_map.get(&self.id) {
            fks.extend(effect_buff.iter_item_list_eids().map(KeyPart::from));
        }
        fks
    }
    fn get_attr_fks(&self, g_supp: &GSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        vec_push_opt(&mut fks, self.discharge_attr_id.map(Into::into));
        vec_push_opt(&mut fks, self.duration_attr_id.map(Into::into));
        vec_push_opt(&mut fks, self.range_attr_id.map(Into::into));
        vec_push_opt(&mut fks, self.falloff_attr_id.map(Into::into));
        vec_push_opt(&mut fks, self.tracking_attr_id.map(Into::into));
        vec_push_opt(&mut fks, self.usage_chance_attr_id.map(Into::into));
        vec_push_opt(&mut fks, self.resist_attr_id.map(Into::into));
        fks.extend(self.get_fks_from_mod_args("modifyingAttributeID"));
        fks.extend(self.get_fks_from_mod_args("modifiedAttributeID"));
        if let Some(effect_buff) = g_supp.eff_buff_map.get(&self.id) {
            fks.extend(effect_buff.iter_attr_eids().map(KeyPart::from));
        }
        // Hardcoded charge info can reference attributes
        if let Some(n_effect) = N_EFFECT_MAP.get(&self.id.into()) {
            fks.extend(n_effect.iter_attr_eids().map(KeyPart::from));
        }
        fks
    }
    fn get_effect_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        // EffectStopper modifier type uses this argument
        self.get_fks_from_mod_args("effectID")
    }
    fn get_buff_fks(&self, g_supp: &GSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        if let Some(effect_buff) = g_supp.eff_buff_map.get(&self.id) {
            fks.extend(effect_buff.iter_buff_eids().map(KeyPart::from));
        }
        fks
    }
}
impl EEffect {
    fn get_fks_from_mod_args(&self, field: &'static str) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        for e_modifier in self.mods.iter() {
            for (k, v) in e_modifier.args.iter() {
                if let (true, &EPrimitive::Int(fk)) = (k == field, v) {
                    fks.push(KeyPart::from_i32(fk));
                }
            }
        }
        fks
    }
}
