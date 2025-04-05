use std::collections::HashMap;

use crate::{
    ac, ad,
    sol::{AttrVal, ItemId, svc::vast::VastFitData, uad::item::Ship},
    util::RSet,
};

pub struct ValRigSizeFail {
    /// Rig size compatible with the ship.
    pub allowed_size: AttrVal,
    /// Sizes of incompatible rigs.
    pub rig_sizes: HashMap<ItemId, Option<AttrVal>>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_rig_size_fast(&self, kfs: &RSet<ItemId>, ship: Option<&Ship>) -> bool {
        let allowed_size = match get_allowed_size(ship) {
            Some(allowed_size) => allowed_size,
            None => return true,
        };
        for (item_id, &rig_size) in self.rigs_rig_size.iter() {
            if rig_size != Some(allowed_size) && !kfs.contains(item_id) {
                return false;
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_rig_size_verbose(
        &self,
        kfs: &RSet<ItemId>,
        ship: Option<&Ship>,
    ) -> Option<ValRigSizeFail> {
        let allowed_size = get_allowed_size(ship)?;
        let mut rig_sizes = HashMap::new();
        for (item_id, &rig_size) in self.rigs_rig_size.iter() {
            if rig_size != Some(allowed_size) && !kfs.contains(item_id) {
                rig_sizes.insert(*item_id, rig_size);
            }
        }
        match rig_sizes.is_empty() {
            true => None,
            false => Some(ValRigSizeFail {
                allowed_size,
                rig_sizes,
            }),
        }
    }
}

fn get_allowed_size(ship: Option<&Ship>) -> Option<ad::AAttrVal> {
    ship?.get_a_attrs()?.get(&ac::attrs::RIG_SIZE).copied()
}
