use crate::{
    ac, ad,
    sol::{AttrVal, ItemId, svc::vast::VastFitData, uad::item::Ship},
    util::RSet,
};

pub struct ValRigSizeFail {
    pub allowed_size: AttrVal,
    pub items: Vec<ValRigSizeItemInfo>,
}

pub struct ValRigSizeItemInfo {
    pub item_id: ItemId,
    pub rig_size: Option<AttrVal>,
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
        let mut mismatches = Vec::new();
        for (item_id, &rig_size) in self.rigs_rig_size.iter() {
            if rig_size != Some(allowed_size) && !kfs.contains(item_id) {
                mismatches.push(ValRigSizeItemInfo {
                    item_id: *item_id,
                    rig_size,
                })
            }
        }
        match mismatches.is_empty() {
            true => None,
            false => Some(ValRigSizeFail {
                allowed_size,
                items: mismatches,
            }),
        }
    }
}

fn get_allowed_size(ship: Option<&Ship>) -> Option<ad::AAttrVal> {
    ship?.get_a_attrs()?.get(&ac::attrs::RIG_SIZE).copied()
}
