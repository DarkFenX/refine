use crate::{
    ad::AEffectBuff,
    ed::{EAttrId, EAttrUnitId, EData, EEffectId, EItemCatId, EItemGrpId},
    nd::N_EFFECT_MAP,
    util::RMap,
};

/// Container for auxiliary data.
pub(in crate::ad::generator) struct GSupport {
    pub(in crate::ad::generator) grp_cat_map: RMap<EItemGrpId, EItemCatId>,
    pub(in crate::ad::generator) attr_unit_map: RMap<EAttrId, EAttrUnitId>,
    pub(in crate::ad::generator) eff_buff_map: RMap<EEffectId, AEffectBuff>,
    // Buffs which can be used, but are not attached to any effect yet
    pub(in crate::ad::generator) standalone_buffs: Vec<AEffectBuff>,
}
impl GSupport {
    pub(in crate::ad::generator) fn new() -> Self {
        Self {
            grp_cat_map: RMap::new(),
            attr_unit_map: RMap::new(),
            eff_buff_map: RMap::new(),
            standalone_buffs: Vec::new(),
        }
    }
    pub(in crate::ad::generator) fn fill(&mut self, e_data: &EData) {
        self.fill_grp_cat_map(e_data);
        self.fill_attr_units(e_data);
        self.fill_buff_data();
    }
    fn fill_grp_cat_map(&mut self, e_data: &EData) {
        for grp in e_data.groups.data.iter() {
            self.grp_cat_map.insert(grp.id, grp.category_id);
        }
    }
    fn fill_buff_data(&mut self) {
        for n_effect in N_EFFECT_MAP.values() {
            if let Some(effect_buff) = &n_effect.adg_buff
                && let Some(e_effect_id) = n_effect.eid
            {
                self.eff_buff_map.insert(e_effect_id, effect_buff.clone());
            }
            if let Some(effect_maker) = n_effect.adg_make_effect_fn
                && let Some(effect_buff) = effect_maker().buff
            {
                self.standalone_buffs.push(effect_buff);
            }
        }
    }
    fn fill_attr_units(&mut self, e_data: &EData) {
        for attr in e_data.attrs.data.iter() {
            if let Some(unit) = attr.unit_id {
                self.attr_unit_map.insert(attr.id, unit);
            }
        }
    }
}
