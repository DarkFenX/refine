use std::collections::HashMap;

use crate::{
    def::{ItemId, OF},
    misc::{EffectId, EffectSpec},
    svc::{SvcCtx, calc::Calc, efuncs, vast::VastFitData},
    ud::UItemKey,
    util::RSet,
};

pub struct ValEffectStopperFail {
    /// Items and their running effects which should be stopped.
    pub items: HashMap<ItemId, Vec<EffectId>>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_effect_stopper_fast(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        for (stopped_espec, stopper_especs) in self.stopped_effects.iter() {
            let stopped_u_item = ctx.u_data.items.get(stopped_espec.item_key);
            if let Some(stopped_reffs) = stopped_u_item.get_reffs()
                && stopped_reffs.contains(&stopped_espec.a_effect_id)
                && is_any_in_effective_range(ctx, calc, stopper_especs.copied(), stopped_espec.item_key)
                && !kfs.contains(&stopped_espec.item_key)
            {
                return false;
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_effect_stopper_verbose(
        &self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValEffectStopperFail> {
        let mut items = HashMap::new();
        for (stopped_espec, stopper_especs) in self.stopped_effects.iter() {
            let stopped_u_item = ctx.u_data.items.get(stopped_espec.item_key);
            if let Some(stopped_reffs) = stopped_u_item.get_reffs()
                && stopped_reffs.contains(&stopped_espec.a_effect_id)
                && is_any_in_effective_range(ctx, calc, stopper_especs.copied(), stopped_espec.item_key)
                && !kfs.contains(&stopped_espec.item_key)
            {
                items
                    .entry(ctx.u_data.items.id_by_key(stopped_espec.item_key))
                    .or_insert_with(Vec::new)
                    .push(stopped_espec.a_effect_id.into());
            }
        }
        match items.is_empty() {
            true => None,
            false => Some(ValEffectStopperFail { items }),
        }
    }
}

// Returns true if any of projectors is in range to block target effect
fn is_any_in_effective_range(
    ctx: SvcCtx,
    calc: &mut Calc,
    stopper_especs: impl Iterator<Item = EffectSpec>,
    stopped_item_key: UItemKey,
) -> bool {
    for stopper_espec in stopper_especs {
        match efuncs::get_espec_proj_mult(ctx, calc, stopper_espec, stopped_item_key) {
            Some(OF(0.0)) => (),
            _ => return true,
        }
    }
    false
}
