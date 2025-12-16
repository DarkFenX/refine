use crate::{
    ad::{
        ABuffId, AEffectBuff, AEffectBuffDuration, AEffectBuffScope, AEffectBuffStrength, AEffectId,
        generator::{
            GSupport,
            rels::{Fk, KeyPart, Pk},
        },
    },
    ed::{EAttrId, EBuffId, EEffect, EItemListId, EPrimitive},
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

impl AEffectBuff {
    pub(in crate::ad::generator::rels) fn iter_e_item_list_ids(&self) -> impl Iterator<Item = EItemListId> {
        self.iter_a_scopes().filter_map(|v| v.get_e_item_list_id())
    }
    pub(in crate::ad::generator::rels) fn iter_e_attr_ids(&self) -> impl Iterator<Item = EAttrId> {
        let attr_merges = self.attr_merge.and_then(|v| v.duration.get_e_attr_id()).into_iter();
        let full_str = self.full.iter().filter_map(|v| v.strength.get_e_attr_id());
        let full_dur = self.full.iter().filter_map(|v| v.duration.get_e_attr_id());
        attr_merges.chain(full_str).chain(full_dur)
    }
    pub(in crate::ad::generator::rels) fn iter_e_buff_ids(&self) -> impl Iterator<Item = EBuffId> {
        self.full.iter().filter_map(|v| match v.buff_id {
            ABuffId::Eve(buff_id) => Some(buff_id),
            ABuffId::Custom(_) => None,
        })
    }
    pub(in crate::ad::generator) fn iter_a_scopes(&self) -> impl Iterator<Item = AEffectBuffScope> {
        let attr_merges = self.attr_merge.map(|v| v.scope).into_iter();
        let full = self.full.iter().map(|v| v.scope);
        attr_merges.chain(full)
    }
}

impl AEffectBuffStrength {
    fn get_e_attr_id(&self) -> Option<EAttrId> {
        match self {
            Self::Attr(a_attr_id) => a_attr_id.dc_eve(),
            Self::Hardcoded(_) => None,
        }
    }
}

impl AEffectBuffDuration {
    fn get_e_attr_id(&self) -> Option<EAttrId> {
        match self {
            Self::None => None,
            Self::AttrMs(a_attr_id) => a_attr_id.dc_eve(),
        }
    }
}

impl AEffectBuffScope {
    fn get_e_item_list_id(&self) -> Option<EItemListId> {
        match self {
            Self::Carrier => None,
            Self::Projected(item_list_id) => item_list_id.dc_eve(),
            Self::Fleet(item_list_id) => item_list_id.dc_eve(),
        }
    }
}
