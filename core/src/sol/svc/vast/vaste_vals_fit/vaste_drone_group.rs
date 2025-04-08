use std::collections::HashMap;

use crate::{
    sol::{ItemGrpId, ItemId, ItemKey, svc::vast::VastFitData, uad::Uad},
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
    pub(in crate::sol::svc::vast) fn validate_drone_group_fast(&mut self, kfs: &RSet<ItemKey>) -> bool {
        match kfs.is_empty() {
            true => self.drone_groups.is_empty(),
            false => self.drone_groups.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_drone_group_verbose(
        &mut self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
    ) -> Option<ValDroneGroupFail> {
        if self.drone_groups.is_empty() {
            return None;
        }
        let drone_groups: HashMap<_, _> = self
            .drone_groups
            .iter()
            .filter(|(drone_key, _)| !kfs.contains(drone_key))
            .map(|(drone_key, drone_a_group_id)| (uad.items.id_by_key(*drone_key), *drone_a_group_id))
            .collect();
        match drone_groups.is_empty() {
            true => None,
            false => Some(ValDroneGroupFail {
                allowed_group_ids: self.drone_group_limit.clone(),
                drone_groups,
            }),
        }
    }
}
