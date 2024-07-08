use crate::{
    defs::{AttrVal, EAttrId},
    sol::{
        item::SolItem,
        svc::{
            svce_calc::{SolContext, SolCtxModifier, SolModification, SolModificationKey, SolModifierKind},
            SolSvcs,
        },
        SolView,
    },
    util::StMap,
};

impl SolSvcs {
    pub(super) fn calc_iter_modifications(
        &mut self,
        sol_view: &SolView,
        item: &SolItem,
        attr_id: &EAttrId,
    ) -> impl Iterator<Item = SolModification> {
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
        mods.into_values()
    }
    // Private methods
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
}
