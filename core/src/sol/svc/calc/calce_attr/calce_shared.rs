use ordered_float::Float;

use crate::{
    defs::{AttrVal, OF},
    sol::{
        svc::calc::{SolCalc, SolContext, SolCtxModifier, SolModifierKind},
        uad::SolUad,
    },
};

impl SolCalc {
    pub(super) fn calc_resist_mult(&mut self, uad: &SolUad, modifier: &SolCtxModifier) -> Option<AttrVal> {
        // Only buffs and targeted modifiers can be resisted
        if !matches!(modifier.raw.kind, SolModifierKind::Buff | SolModifierKind::Targeted) {
            return None;
        }
        let resist_attr_id = modifier.raw.resist_attr_id?;
        let projectee_item_id = match modifier.ctx {
            SolContext::Item(projectee_item_id) => projectee_item_id,
            _ => return None,
        };
        let resist = self
            .get_item_attr_val_full(uad, &projectee_item_id, &resist_attr_id)
            .ok()?
            .dogma;
        Some(resist)
    }
    pub(super) fn calc_proj_mult(&mut self, uad: &SolUad, modifier: &SolCtxModifier) -> Option<AttrVal> {
        let projectee_item_id = match modifier.ctx {
            SolContext::Item(projectee_item_id) => projectee_item_id,
            _ => return None,
        };
        let proj_range =
            self.projs
                .get_range(modifier.raw.affector_item_id, modifier.raw.effect_id, projectee_item_id)?;
        match modifier.raw.kind {
            SolModifierKind::Targeted => self.calc_proj_mult_targeted(uad, modifier, proj_range),
            SolModifierKind::Buff => self.calc_proj_mult_buff(uad, modifier, proj_range),
            _ => None,
        }
    }
    // Private methods
    fn calc_proj_mult_targeted(
        &mut self,
        uad: &SolUad,
        modifier: &SolCtxModifier,
        proj_range: AttrVal,
    ) -> Option<AttrVal> {
        // Assume optimal range is 0 if it's not available
        let affector_optimal = match modifier.raw.optimal_attr_id {
            Some(optimal_attr_id) => {
                match self.get_item_attr_val_full(uad, &modifier.raw.affector_item_id, &optimal_attr_id) {
                    Ok(val) => val.dogma,
                    _ => OF(0.0),
                }
            }
            None => OF(0.0),
        };
        // Assume falloff range is 0 if it's not available
        let affector_falloff = match modifier.raw.falloff_attr_id {
            Some(falloff_attr_id) => {
                match self.get_item_attr_val_full(uad, &modifier.raw.affector_item_id, &falloff_attr_id) {
                    Ok(val) => val.dogma,
                    _ => OF(0.0),
                }
            }
            None => OF(0.0),
        };
        // TODO: do not hardcode it here, define on a per-effect basis
        let restricted_range = false;
        // Calculate actual range multiplier after collecting all the data
        if affector_falloff > OF(0.0) {
            if restricted_range && proj_range > affector_optimal + OF(3.0) * affector_falloff {
                Some(OF(0.0))
            } else {
                let val = Float::powf(
                    OF(0.5),
                    (Float::max(OF(0.0), proj_range - affector_optimal) / affector_falloff).powi(2),
                );
                Some(val)
            }
        } else if proj_range <= affector_optimal {
            Some(OF(1.0))
        } else {
            Some(OF(0.0))
        }
    }
    fn calc_proj_mult_buff(&mut self, uad: &SolUad, modifier: &SolCtxModifier, proj_range: AttrVal) -> Option<AttrVal> {
        let affector_optimal = match modifier.raw.optimal_attr_id {
            Some(optimal_attr_id) => {
                match self.get_item_attr_val_full(uad, &modifier.raw.affector_item_id, &optimal_attr_id) {
                    Ok(val) => val.dogma,
                    // If optimal range attribute ID is defined but value is not available, assume
                    // optimal range of 0
                    _ => OF(0.0),
                }
            }
            // If optimal range attribute ID not defined, assume buff is not restricted by range
            None => return None,
        };
        if proj_range <= affector_optimal {
            Some(OF(1.0))
        } else {
            Some(OF(0.0))
        }
    }
}
