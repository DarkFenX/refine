use itertools::Itertools;

use crate::{
    ItemGrpId, ItemId,
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemId,
    util::RSet,
};

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Clone)]
pub struct ValDroneGroupFail {
    /// Drone item groups allowed by the ship.
    pub allowed_group_ids: Vec<ItemGrpId>,
    /// Drones breaking the validation and their groups.
    #[cfg_attr(feature = "serde", serde(serialize_with = "custom_serde::as_map"))]
    pub drone_groups: Vec<ValDroneGroupInfo>,
}

/// Drones which break the validation and their group.
#[derive(Copy, Clone)]
pub struct ValDroneGroupInfo {
    pub drone_id: ItemId,
    pub group_id: ItemGrpId,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_drone_group_fast(&self, kfs: &RSet<UItemId>) -> bool {
        match kfs.is_empty() {
            true => self.drone_groups.is_empty(),
            false => self.drone_groups.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_drone_group_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValDroneGroupFail> {
        if self.drone_groups.is_empty() {
            return None;
        }
        let drone_groups = self
            .drone_groups
            .iter()
            .filter_map(|(drone_uid, drone_group_aid)| match kfs.contains(drone_uid) {
                true => None,
                false => Some(ValDroneGroupInfo {
                    drone_id: ctx.u_data.items.ext_id_by_int_id(*drone_uid),
                    group_id: ItemGrpId::from_aid(*drone_group_aid),
                }),
            })
            .collect_vec();
        match drone_groups.is_empty() {
            true => None,
            false => Some(ValDroneGroupFail {
                allowed_group_ids: self
                    .drone_group_limit
                    .iter()
                    .map(|grp_aid| ItemGrpId::from_aid(*grp_aid))
                    .collect(),
                drone_groups,
            }),
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

    pub(super) fn as_map<S>(items: &[ValDroneGroupInfo], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(items.len()))?;
        for item in items {
            map.serialize_entry(&item.drone_id, &item.group_id)?;
        }
        map.end()
    }
}
