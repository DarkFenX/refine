use std::collections::HashMap;

use crate::{
    def::{AttrVal, ItemId, OF},
    misc::{EffectId, EffectSpec},
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    ud::UItemKey,
    util::{GetId, RSet},
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
                && stopped_reffs.contains(&stopped_espec.effect_key)
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
                && stopped_reffs.contains(&stopped_espec.effect_key)
                && is_any_in_effective_range(ctx, calc, stopper_especs.copied(), stopped_espec.item_key)
                && !kfs.contains(&stopped_espec.item_key)
            {
                let item_id = ctx.u_data.items.id_by_key(stopped_espec.item_key);
                let effect_id = ctx.u_data.src.get_effect(stopped_espec.effect_key).get_id();
                items.entry(item_id).or_insert_with(Vec::new).push(effect_id.into());
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
        match get_espec_proj_mult(ctx, calc, stopper_espec, stopped_item_key) {
            Some(OF(0.0)) => (),
            _ => return true,
        }
    }
    false
}

fn get_espec_proj_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_espec: EffectSpec,
    projectee_key: UItemKey,
) -> Option<AttrVal> {
    let projector_effect = ctx.u_data.src.get_effect(projector_espec.effect_key);
    let proj_mult_getter = projector_effect.modifier_proj_mult_getter?;
    let proj_data = ctx.eff_projs.get_proj_data(projector_espec, projectee_key)?;
    Some(proj_mult_getter(
        ctx,
        calc,
        projector_espec.item_key,
        projector_effect,
        projectee_key,
        proj_data,
    ))
}
