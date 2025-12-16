use crate::{
    ad::{AAttr, ABuff, AEffectBuff},
    ed::{EAttrId, EAttrUnitId, EData, EEffectId, EItemCatId, EItemGrpId},
    nd::{N_ATTR_MAP, N_BUFF_MAP, N_EFFECT_MAP},
    util::RMap,
};

/// Container for auxiliary data.
pub(in crate::ad::generator) struct GSupport {
    pub(in crate::ad::generator) grp_cat_map: RMap<EItemGrpId, EItemCatId>,
    pub(in crate::ad::generator) attr_unit_map: RMap<EAttrId, EAttrUnitId>,
    pub(in crate::ad::generator) eff_buff_map: RMap<EEffectId, AEffectBuff>,
    // Standalone containers are for entities which do not exist in data yet, but will be put into
    // it later
    pub(in crate::ad::generator) standalone_attrs: Vec<AAttr>,
    pub(in crate::ad::generator) standalone_effect_buffs: Vec<AEffectBuff>,
    pub(in crate::ad::generator) standalone_buffs: Vec<ABuff>,
}
impl GSupport {
    pub(in crate::ad::generator) fn new() -> Self {
        Self {
            grp_cat_map: RMap::new(),
            attr_unit_map: RMap::new(),
            eff_buff_map: RMap::new(),
            standalone_attrs: Vec::new(),
            standalone_effect_buffs: Vec::new(),
            standalone_buffs: Vec::new(),
        }
    }
    pub(in crate::ad::generator) fn fill(&mut self, e_data: &EData) {
        self.fill_grp_cat_map(e_data);
        self.fill_attr_units(e_data);
        self.fill_effect_buff_data();
        self.fill_standalone_data();
    }
    fn fill_grp_cat_map(&mut self, e_data: &EData) {
        for grp in e_data.groups.data.iter() {
            self.grp_cat_map.insert(grp.id, grp.category_id);
        }
    }
    fn fill_attr_units(&mut self, e_data: &EData) {
        for attr in e_data.attrs.data.iter() {
            if let Some(unit) = attr.unit_id {
                self.attr_unit_map.insert(attr.id, unit);
            }
        }
    }
    fn fill_effect_buff_data(&mut self) {
        for n_effect in N_EFFECT_MAP.values() {
            if let Some(e_effect_id) = n_effect.eid
                && let Some(effect_buff) = &n_effect.adg_buff
            {
                self.eff_buff_map.insert(e_effect_id, effect_buff.clone());
            }
        }
    }
    fn fill_standalone_data(&mut self) {
        for n_attr in N_ATTR_MAP.values() {
            if n_attr.eid.is_none()
                && let Some(attr_maker) = n_attr.adg_make_attr_fn
            {
                self.standalone_attrs.push(attr_maker());
            }
        }
        for n_effect in N_EFFECT_MAP.values() {
            if n_effect.eid.is_none()
                && let Some(effect_maker) = n_effect.adg_make_effect_fn
                && let Some(effect_buff) = effect_maker().buff
            {
                self.standalone_effect_buffs.push(effect_buff);
            }
        }
        for n_buff in N_BUFF_MAP.values() {
            if n_buff.eid.is_none()
                && let Some(buff_maker) = n_buff.adg_make_buff_fn
            {
                self.standalone_buffs.push(buff_maker());
            }
        }
    }
}
