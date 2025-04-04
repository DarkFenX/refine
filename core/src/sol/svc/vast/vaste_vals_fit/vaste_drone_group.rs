use itertools::Itertools;

use crate::{
    sol::{ItemGrpId, ItemId, svc::vast::VastFitData},
    util::RSet,
};

pub struct ValDroneGroupFail {
    pub allowed_group_ids: Vec<ItemGrpId>,
    pub items: Vec<ValDroneGroupItemInfo>,
}

pub struct ValDroneGroupItemInfo {
    pub item_id: ItemId,
    pub group_id: ItemGrpId,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_drone_group_fast(&mut self, kfs: &RSet<ItemId>) -> bool {
        match kfs.is_empty() {
            true => self.drone_groups.is_empty(),
            false => self.drone_groups.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_drone_group_verbose(
        &mut self,
        kfs: &RSet<ItemId>,
    ) -> Option<ValDroneGroupFail> {
        if self.drone_groups.is_empty() {
            return None;
        }
        let items = self
            .drone_groups
            .iter()
            .filter(|(k, _)| !kfs.contains(k))
            .map(|(k, v)| ValDroneGroupItemInfo {
                item_id: *k,
                group_id: *v,
            })
            .collect_vec();
        if items.is_empty() {
            return None;
        }
        Some(ValDroneGroupFail {
            allowed_group_ids: self.drone_group_limit.clone(),
            items,
        })
    }
}
