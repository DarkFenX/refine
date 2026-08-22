use itertools::Itertools;

use super::shared::get_max_resource;
use crate::{
    ItemId, Value,
    svc::{Calc, SvcCtx, vast::VastFitData},
    ud::{UFit, UItemId},
    util::RSet,
};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde_tuple::Serialize_tuple)
)]
#[derive(Clone)]
pub struct ValUnusableResFail {
    /// Max available resource.
    pub max: Option<Value>,
    #[cfg_attr(feature = "serde", serde_as(as = "refine_serde::VecAsMap"))]
    pub users: Vec<ValUnusableResItemInfo>,
}

#[cfg_attr(feature = "serde", derive(refine_serde::VecAsMapEntry))]
#[derive(Copy, Clone)]
pub struct ValUnusableResItemInfo {
    /// Item which consumes the resource.
    #[cfg_attr(feature = "serde", vec_map(key))]
    pub item_id: ItemId,
    /// How much resource is used by the item.
    #[cfg_attr(feature = "serde", vec_map(value))]
    pub used: Value,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_unlaunchable_drone_bandwidth_fast(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> bool {
        if self.drones_bandwidth.is_empty() {
            return true;
        }
        let max = get_max_resource(ctx, calc, fit.ship, ctx.ac().drone_bandwidth).unwrap_or(Value::ZERO);
        for (item_uid, &item_use) in self.drones_bandwidth.iter() {
            if item_use > max && !kfs.contains(item_uid) {
                return false;
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast::val) fn validate_unlaunchable_drone_bandwidth_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> Option<ValUnusableResFail> {
        if self.drones_bandwidth.is_empty() {
            return None;
        }
        let max = get_max_resource(ctx, calc, fit.ship, ctx.ac().drone_bandwidth);
        let effective_max = max.unwrap_or(Value::ZERO);
        let users = self
            .drones_bandwidth
            .iter()
            .filter_map(
                |(item_uid, &item_use)| match item_use > effective_max && !kfs.contains(item_uid) {
                    true => Some(ValUnusableResItemInfo {
                        item_id: ctx.u_data.items.ext_id_by_int_id(*item_uid),
                        used: item_use,
                    }),
                    false => None,
                },
            )
            .collect_vec();
        match users.is_empty() {
            true => None,
            false => Some(ValUnusableResFail { max, users }),
        }
    }
}
