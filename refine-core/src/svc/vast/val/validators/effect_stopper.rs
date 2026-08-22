use crate::{
    EffectId, ItemId, PValue,
    misc::EffectSpec,
    stats::StatCritOptions,
    svc::{Calc, SvcCtx, funcs, vast::VastFitData},
    ud::UItemId,
    util::{RMap, RSet},
};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
#[derive(Clone)]
pub struct ValEffectStopperFail {
    #[cfg_attr(feature = "serde", serde_as(as = "refine_serde::VecAsMap"))]
    pub items: Vec<ValEffectStopperItemInfo>,
}

/// Item and its running effects which should be stopped.
#[cfg_attr(feature = "serde", derive(refine_serde::VecAsMapEntry))]
#[derive(Clone)]
pub struct ValEffectStopperItemInfo {
    #[cfg_attr(feature = "serde", vec_map(key))]
    pub item_id: ItemId,
    #[cfg_attr(feature = "serde", vec_map(value))]
    pub effect_ids: Vec<EffectId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_effect_stopper_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        for (stopped_espec, stopper_especs) in self.stopped_effects.iter() {
            let stopped_u_item = ctx.u_data.items.get(stopped_espec.item_uid);
            if let Some(stopped_reffs) = stopped_u_item.get_reffs()
                && stopped_reffs.contains(&stopped_espec.effect_rid)
                && is_any_in_effective_range(ctx, calc, stopper_especs.copied(), stopped_espec.item_uid)
                && !kfs.contains(&stopped_espec.item_uid)
            {
                return false;
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast::val) fn validate_effect_stopper_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
    ) -> Option<ValEffectStopperFail> {
        let mut items = RMap::new();
        for (stopped_espec, stopper_especs) in self.stopped_effects.iter() {
            let stopped_u_item = ctx.u_data.items.get(stopped_espec.item_uid);
            if let Some(stopped_reffs) = stopped_u_item.get_reffs()
                && stopped_reffs.contains(&stopped_espec.effect_rid)
                && is_any_in_effective_range(ctx, calc, stopper_especs.copied(), stopped_espec.item_uid)
                && !kfs.contains(&stopped_espec.item_uid)
            {
                let item_id = ctx.u_data.items.ext_id_by_int_id(stopped_espec.item_uid);
                let effect_aid = ctx.u_data.r_data.get_effect_by_rid(stopped_espec.effect_rid).aid;
                items
                    .entry(item_id)
                    .or_insert_with(Vec::new)
                    .push(EffectId::from_aid(effect_aid));
            }
        }
        match items.is_empty() {
            true => None,
            false => Some(ValEffectStopperFail {
                items: items
                    .into_iter()
                    .map(|(item_id, effect_ids)| ValEffectStopperItemInfo { item_id, effect_ids })
                    .collect(),
            }),
        }
    }
}

// Returns true if any of projectors is in range to block target effect
fn is_any_in_effective_range(
    ctx: SvcCtx,
    calc: &mut Calc,
    stopper_especs: impl Iterator<Item = EffectSpec>,
    stopped_item_uid: UItemId,
) -> bool {
    for stopper_espec in stopper_especs {
        match funcs::get_espec_proj_mult(ctx, calc, stopper_espec, stopped_item_uid, StatCritOptions::default()) {
            Some(PValue::ZERO) => (),
            _ => return true,
        }
    }
    false
}
