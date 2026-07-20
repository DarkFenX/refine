use itertools::Itertools;

use super::shared::get_max_resource;
use crate::{
    num::Value,
    svc::{SvcCtx, calc::Calc, vast::VastFitData},
    ud::{ItemId, UFit, UItemId},
    util::RSet,
};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde_tuple::Serialize_tuple)
)]
pub struct ValUnusableResFail {
    /// Max available resource (e.g. amount of CPU produced by ship).
    pub max: Option<Value>,
    /// Consumers and amount they consume.
    #[cfg_attr(feature = "serde", serde_as(as = "&serde_with::Map<_, _>"))]
    pub users: Vec<(ItemId, Value)>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_unlaunchable_drone_bandwidth_fast(
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
    pub(in crate::svc::vast) fn validate_unlaunchable_drone_bandwidth_verbose(
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
                    true => Some((ctx.u_data.items.ext_id_by_int_id(*item_uid), item_use)),
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
