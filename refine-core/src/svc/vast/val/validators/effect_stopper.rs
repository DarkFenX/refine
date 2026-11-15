use itertools::Itertools;

use crate::{
    EffectId, ItemId, PValue,
    misc::EffectSpec,
    stats::StatCritOptions,
    svc::{Calc, SvcCtx, funcs, vast::VastFitData},
    ud::UItemId,
    util::{RMap, RSet},
    val::{ValItemStatus, ValResult},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Result
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
#[derive(Clone)]
pub struct ValEffectStopperFail {
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::KeyValueMap<_>"))]
    pub items: Vec<ValEffectStopperItemInfo>,
}

/// Item and its running effects which should be stopped.
#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Clone)]
pub struct ValEffectStopperItemInfo {
    pub item_id: ItemId,
    pub status: ValItemStatus,
    pub effect_ids: Vec<EffectId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Validators
////////////////////////////////////////////////////////////////////////////////////////////////////
impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_effect_stopper_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        calc: &mut Calc,
    ) -> bool {
        for (&stopped_espec, stopper_especs) in self.stopped_effects.iter() {
            let stopped_u_item = ctx.u_data.items.get(stopped_espec.item_uid);
            if let Some(stopped_reffs) = stopped_u_item.get_reffs()
                && stopped_reffs.contains(stopped_espec.effect_rid)
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
        ctx: &SvcCtx,
        calc: &mut Calc,
    ) -> ValResult<ValEffectStopperFail> {
        let mut items = RMap::new();
        for (&stopped_espec, stopper_especs) in self.stopped_effects.iter() {
            let stopped_u_item = ctx.u_data.items.get(stopped_espec.item_uid);
            if let Some(stopped_reffs) = stopped_u_item.get_reffs()
                && stopped_reffs.contains(stopped_espec.effect_rid)
                && is_any_in_effective_range(ctx, calc, stopper_especs.copied(), stopped_espec.item_uid)
            {
                let effect_aid = ctx.u_data.r_data.get_effect_by_rid(stopped_espec.effect_rid).aid;
                items
                    .entry(stopped_espec.item_uid)
                    .or_insert_with(Vec::new)
                    .push(EffectId::from_aid(effect_aid));
            }
        }
        let items = items
            .into_iter()
            .map(|(item_uid, effect_ids)| ValEffectStopperItemInfo {
                item_id: ctx.u_data.items.ext_id_by_int_id(item_uid),
                status: ValItemStatus::from_kfs(kfs, item_uid),
                effect_ids,
            })
            .collect_vec();
        match items.is_empty() {
            true => ValResult::Passed,
            false => ValResult::from_fail(items.iter().all(|v| v.status.is_kf()), ValEffectStopperFail { items }),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Returns true if any of projectors is in range to block target effect
fn is_any_in_effective_range(
    ctx: &SvcCtx,
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
