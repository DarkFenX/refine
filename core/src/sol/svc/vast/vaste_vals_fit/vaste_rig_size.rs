use crate::{
    consts,
    defs::{AttrVal, SolItemId},
    sol::{svc::vast::SolVastFitData, uad::item::SolShip},
    util::StSet,
};

pub struct SolValRigSizeFail {
    pub allowed_size: AttrVal,
    pub items: Vec<SolValRigSizeItemInfo>,
}

pub struct SolValRigSizeItemInfo {
    pub item_id: SolItemId,
    pub rig_size: Option<AttrVal>,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_rig_size_fast(
        &self,
        kfs: &StSet<SolItemId>,
        ship: Option<&SolShip>,
    ) -> bool {
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
        kfs: &StSet<SolItemId>,
        ship: Option<&SolShip>,
    ) -> Option<SolValRigSizeFail> {
        let allowed_size = get_allowed_size(ship)?;
        let mut mismatches = Vec::new();
        for (item_id, &rig_size) in self.rigs_rig_size.iter() {
            if rig_size != Some(allowed_size) && !kfs.contains(item_id) {
                mismatches.push(SolValRigSizeItemInfo {
                    item_id: *item_id,
                    rig_size,
                })
            }
        }
        match mismatches.is_empty() {
            true => None,
            false => Some(SolValRigSizeFail {
                allowed_size,
                items: mismatches,
            }),
        }
    }
}

fn get_allowed_size(ship: Option<&SolShip>) -> Option<AttrVal> {
    ship?.get_attrs()?.get(&consts::attrs::RIG_SIZE).copied()
}
