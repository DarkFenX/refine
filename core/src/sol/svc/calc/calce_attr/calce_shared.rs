use ordered_float::{Float, OrderedFloat as OF};

use crate::{
    ac, ad,
    sol::{
        AttrVal,
        svc::calc::{Calc, Context, CtxModifier, ModifierKind},
        uad::Uad,
    },
};

pub(super) const LIMITED_PRECISION_A_ATTR_IDS: [ad::AAttrId; 4] = [
    ac::attrs::CPU,
    ac::attrs::POWER,
    ac::attrs::CPU_OUTPUT,
    ac::attrs::POWER_OUTPUT,
];

impl Calc {
    pub(super) fn calc_resist_mult(&mut self, uad: &Uad, modifier: &CtxModifier) -> Option<AttrVal> {
        // Only buffs and targeted modifiers can be resisted
        if !matches!(modifier.raw.kind, ModifierKind::Buff | ModifierKind::Targeted) {
            return None;
        }
        let resist_a_attr_id = modifier.raw.resist_a_attr_id?;
        let projectee_item_id = match modifier.ctx {
            Context::Item(projectee_item_id) => projectee_item_id,
            _ => return None,
        };
        let resist = self
            .get_item_attr_val_full(uad, &projectee_item_id, &resist_a_attr_id)
            .ok()?
            .dogma;
        Some(resist)
    }
    pub(super) fn calc_proj_mult(&mut self, uad: &Uad, modifier: &CtxModifier) -> Option<AttrVal> {
        let projectee_item_id = match modifier.ctx {
            Context::Item(projectee_item_id) => projectee_item_id,
            _ => return None,
        };
        let proj_range = self.projs.get_range(
            modifier.raw.affector_item_id,
            modifier.raw.a_effect_id,
            projectee_item_id,
        )?;
        match modifier.raw.kind {
            ModifierKind::Targeted => self.calc_proj_mult_targeted(uad, modifier, proj_range),
            ModifierKind::Buff => self.calc_proj_mult_buff(uad, modifier, proj_range),
            _ => None,
        }
    }
    // Private methods
    fn calc_proj_mult_targeted(&mut self, uad: &Uad, modifier: &CtxModifier, proj_range: AttrVal) -> Option<AttrVal> {
        // Assume optimal range is 0 if it's not available
        let affector_optimal = match modifier.raw.optimal_a_attr_id {
            Some(optimal_a_attr_id) => {
                match self.get_item_attr_val_full(uad, &modifier.raw.affector_item_id, &optimal_a_attr_id) {
                    Ok(val) => val.dogma,
                    _ => OF(0.0),
                }
            }
            None => OF(0.0),
        };
        // Assume falloff range is 0 if it's not available
        let affector_falloff = match modifier.raw.falloff_a_attr_id {
            Some(falloff_a_attr_id) => {
                match self.get_item_attr_val_full(uad, &modifier.raw.affector_item_id, &falloff_a_attr_id) {
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
    fn calc_proj_mult_buff(&mut self, uad: &Uad, modifier: &CtxModifier, proj_range: AttrVal) -> Option<AttrVal> {
        let affector_optimal = match modifier.raw.optimal_a_attr_id {
            Some(optimal_a_attr_id) => {
                match self.get_item_attr_val_full(uad, &modifier.raw.affector_item_id, &optimal_a_attr_id) {
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
