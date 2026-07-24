use itertools::Itertools;

use super::shared::get_max_resource;
use crate::{
    ItemId, Value,
    svc::{Calc, SvcCtx, vast::VastFitData},
    ud::{UFit, UItemId},
    util::RSet,
};

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
pub struct ValUnusableResFail {
    /// Max available resource.
    pub max: Option<Value>,
    #[cfg_attr(feature = "serde", serde(serialize_with = "custom_serde::as_map"))]
    pub users: Vec<ValUnusableResItemInfo>,
}

pub struct ValUnusableResItemInfo {
    /// Item which consumes the resource.
    pub item_id: ItemId,
    /// How much resource is used by the item.
    pub used: Value,
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::ser::{SerializeMap, Serializer};

    use super::*;

    pub(super) fn as_map<S>(items: &[ValUnusableResItemInfo], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(items.len()))?;
        for item in items {
            map.serialize_entry(&item.item_id, &item.used)?;
        }
        map.end()
    }
}
