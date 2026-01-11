use std::collections::HashMap;

use crate::{
    api::ItemGrpId,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UItemId},
    util::RSet,
};

pub struct ValDroneGroupFail {
    /// Drone item groups allowed by the ship.
    pub allowed_group_ids: Vec<ItemGrpId>,
    /// Drones breaking the validation and their groups.
    pub drone_groups: HashMap<ItemId, ItemGrpId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_drone_group_fast(&mut self, kfs: &RSet<UItemId>) -> bool {
        match kfs.is_empty() {
            true => self.drone_groups.is_empty(),
            false => self.drone_groups.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_drone_group_verbose(
        &mut self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValDroneGroupFail> {
        if self.drone_groups.is_empty() {
            return None;
        }
        let drone_groups: HashMap<_, _> = self
            .drone_groups
            .iter()
            .filter(|(drone_uid, _)| !kfs.contains(drone_uid))
            .map(|(drone_uid, drone_group_aid)| {
                (
                    ctx.u_data.items.xid_by_iid(*drone_uid),
                    ItemGrpId::from_aid(*drone_group_aid),
                )
            })
            .collect();
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
