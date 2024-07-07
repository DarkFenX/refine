use crate::{
    defs::{AttrVal, EAttrId, SolItemId},
    ec,
    sol::{
        item::SolItem,
        svc::{
            svce_calc::{
                SolAttrVal, SolAttrValues, SolContext, SolCtxModifier, SolModification, SolModificationKey,
                SolModifierKind,
            },
            SolSvcs,
        },
        SolView,
    },
    util::{Error, ErrorKind, Result, StMap},
};

const LIMITED_PRECISION_ATTR_IDS: [EAttrId; 4] = [
    ec::attrs::CPU,
    ec::attrs::POWER,
    ec::attrs::CPU_OUTPUT,
    ec::attrs::POWER_OUTPUT,
];

impl SolSvcs {
    // Query methods
    pub(in crate::sol) fn calc_get_item_attr_val(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) -> Result<SolAttrVal> {
        // Try accessing cached value
        match self.calc_data.attrs.get_item_attrs(item_id)?.get(attr_id) {
            Some(v) => return Ok(*v),
            _ => (),
        };
        // If it is not cached, calculate and cache it
        let val = self.calc_calc_item_attr_val(sol_view, item_id, attr_id)?;
        self.calc_data.attrs.get_item_attrs_mut(item_id)?.insert(*attr_id, val);
        Ok(val)
    }
    pub(in crate::sol) fn calc_iter_item_attr_vals(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EAttrId, SolAttrVal)>> {
        // SolItem can have attributes which are not defined on the original EVE item. This happens
        // when something requested an attr value, and it was calculated using base attribute value.
        // Here, we get already calculated attributes, which includes attributes absent on the EVE
        // item
        let mut vals = self.calc_data.attrs.get_item_attrs_mut(item_id)?.clone();
        // Calculate & store attributes which are not calculated yet, but are defined on the EVE
        // item
        for attr_id in sol_view.items.get_item(item_id)?.get_orig_attrs()?.keys() {
            match self.calc_get_item_attr_val(sol_view, item_id, attr_id) {
                Ok(v) => vals.entry(*attr_id).or_insert(v),
                _ => continue,
            };
        }
        Ok(vals.into_iter())
    }
    // Private methods
    fn calc_get_modifications(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        attr_id: &EAttrId,
    ) -> StMap<SolModificationKey, SolModification> {
        let mut mods = StMap::new();
        for modifier in self
            .calc_data
            .std
            .get_mods_for_affectee(item, attr_id, sol_view.fits)
            .iter()
        {
            let val = match modifier.raw.get_mod_val(self, sol_view) {
                Ok(v) => v,
                _ => continue,
            };
            let affector_item = match sol_view.items.get_item(&modifier.raw.affector_item_id) {
                Ok(i) => i,
                _ => continue,
            };
            let affector_item_cat_id = match affector_item.get_category_id() {
                Ok(affector_item_cat_id) => affector_item_cat_id,
                _ => continue,
            };
            let mod_key = SolModificationKey::from(modifier);
            let modification = SolModification::new(
                modifier.raw.op,
                val,
                self.calc_resist_mult(sol_view, modifier),
                self.calc_proj_mult(sol_view, modifier),
                modifier.raw.aggr_mode,
                affector_item_cat_id,
            );
            mods.insert(mod_key, modification);
        }
        mods
    }
    fn calc_resist_mult(&mut self, sol_view: &SolView, modifier: &SolCtxModifier) -> Option<AttrVal> {
        // Only buffs and targeted modifiers can be resisted
        if !matches!(modifier.raw.kind, SolModifierKind::Buff | SolModifierKind::Targeted) {
            return None;
        }
        let resist_attr_id = match modifier.raw.resist_attr_id {
            Some(resist_attr_id) => resist_attr_id,
            None => return None,
        };
        let projectee_item_id = match modifier.ctx {
            SolContext::Item(projectee_item_id) => projectee_item_id,
            _ => return None,
        };
        let resist = match self.calc_get_item_attr_val(sol_view, &projectee_item_id, &resist_attr_id) {
            Ok(val) => val.dogma,
            _ => return None,
        };
        Some(resist)
    }
    fn calc_proj_mult(&mut self, sol_view: &SolView, modifier: &SolCtxModifier) -> Option<AttrVal> {
        let projectee_item_id = match modifier.ctx {
            SolContext::Item(projectee_item_id) => projectee_item_id,
            _ => return None,
        };
        let proj_range = match self.calc_data.projs.get_range(
            modifier.raw.affector_item_id,
            modifier.raw.effect_id,
            projectee_item_id,
        ) {
            Some(range) => range,
            None => return None,
        };
        match modifier.raw.kind {
            SolModifierKind::Targeted => self.calc_proj_mult_targeted(sol_view, modifier, proj_range),
            SolModifierKind::Buff => self.calc_proj_mult_buff(sol_view, modifier, proj_range),
            _ => None,
        }
    }
    fn calc_proj_mult_targeted(
        &mut self,
        sol_view: &SolView,
        modifier: &SolCtxModifier,
        proj_range: AttrVal,
    ) -> Option<AttrVal> {
        // Assume optimal range is 0 if it's not available
        let affector_optimal = match modifier.raw.optimal_attr_id {
            Some(optimal_attr_id) => {
                match self.calc_get_item_attr_val(sol_view, &modifier.raw.affector_item_id, &optimal_attr_id) {
                    Ok(val) => val.dogma,
                    _ => 0.0,
                }
            }
            None => 0.0,
        };
        // Assume falloff range is 0 if it's not available
        let affector_falloff = match modifier.raw.falloff_attr_id {
            Some(falloff_attr_id) => {
                match self.calc_get_item_attr_val(sol_view, &modifier.raw.affector_item_id, &falloff_attr_id) {
                    Ok(val) => val.dogma,
                    _ => 0.0,
                }
            }
            None => 0.0,
        };
        // TODO: do not hardcode it here, define on a per-effect basis
        let restricted_range = false;
        // Calculate actual range multiplier after collecting all the data
        if affector_falloff > 0.0 {
            if restricted_range && proj_range > affector_optimal + 3.0 * affector_falloff {
                Some(0.0)
            } else {
                let val = AttrVal::powf(
                    0.5,
                    (AttrVal::max(0.0, proj_range - affector_optimal) / affector_falloff).powi(2),
                );
                Some(val)
            }
        } else if proj_range <= affector_optimal {
            Some(1.0)
        } else {
            Some(0.0)
        }
    }
    fn calc_proj_mult_buff(
        &mut self,
        sol_view: &SolView,
        modifier: &SolCtxModifier,
        proj_range: AttrVal,
    ) -> Option<AttrVal> {
        let affector_optimal = match modifier.raw.optimal_attr_id {
            Some(optimal_attr_id) => {
                match self.calc_get_item_attr_val(sol_view, &modifier.raw.affector_item_id, &optimal_attr_id) {
                    Ok(val) => val.dogma,
                    // If optimal range attribute ID is defined but value is not available, assume
                    // optimal range of 0
                    _ => 0.0,
                }
            }
            // If optimal range attribute ID not defined, assume buff is not restricted by range
            None => return None,
        };
        if proj_range <= affector_optimal {
            Some(1.0)
        } else {
            Some(0.0)
        }
    }
    fn calc_calc_item_attr_val(
        &mut self,
        sol_view: &SolView,
        item_id: &SolItemId,
        attr_id: &EAttrId,
    ) -> Result<SolAttrVal> {
        let item = sol_view.items.get_item(item_id)?;
        let attr = match sol_view.src.get_a_attr(attr_id) {
            Some(attr) => attr,
            None => return Err(Error::new(ErrorKind::AAttrNotFound(*attr_id))),
        };
        // Get base value; use on-item original attributes, or, if not specified, default attribute value.
        // If both can't be fetched, consider it a failure
        let base_val = match item.get_orig_attrs()?.get(attr_id) {
            Some(orig_val) => *orig_val,
            None => attr.def_val,
        };
        match (attr_id, item) {
            (&ec::attrs::SKILL_LEVEL, SolItem::Skill(s)) => {
                return Ok(SolAttrVal::new(base_val, s.level as AttrVal, s.level as AttrVal))
            }
            _ => (),
        }
        let mut vals = SolAttrValues::new();
        for modification in self.calc_get_modifications(sol_view, item, attr_id).values() {
            vals.add_val(
                modification.val,
                modification.res_mult,
                modification.proj_mult,
                &modification.op,
                attr.penalizable,
                &modification.affector_item_cat_id,
                &modification.aggr_mode,
            );
        }
        let dogma_val = vals.apply_dogma_mods(base_val, attr.hig);
        // Upper cap for the attribute value being calculated
        let mut dogma_val = match attr.max_attr_id {
            Some(capping_attr_id) => match self.calc_get_item_attr_val(sol_view, item_id, &capping_attr_id) {
                Ok(capping_vals) => {
                    self.calc_data
                        .deps
                        .add_direct_local(*item_id, capping_attr_id, *attr_id);
                    AttrVal::min(dogma_val, capping_vals.dogma)
                }
                Err(_) => dogma_val,
            },
            None => dogma_val,
        };
        if LIMITED_PRECISION_ATTR_IDS.contains(attr_id) {
            dogma_val = (dogma_val * 100.0).round() / 100.0
        }
        // Post-dogma calculations
        let extra_val = vals.apply_extra_mods(dogma_val, attr.hig);
        Ok(SolAttrVal::new(base_val, dogma_val, extra_val))
    }
}
