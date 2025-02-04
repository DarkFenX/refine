use crate::{
    defs::{AttrVal, SolItemId},
    ec,
    sol::{svc::vast::SolVastFitData, uad::item::SolShip},
};

pub struct SolRigSizeValFail {
    pub allowed_size: AttrVal,
    pub mismatches: Vec<SolRigSizeMismatch>,
}
impl SolRigSizeValFail {
    fn new(allowed_size: AttrVal, mismatches: Vec<SolRigSizeMismatch>) -> Self {
        Self {
            allowed_size,
            mismatches,
        }
    }
}

pub struct SolRigSizeMismatch {
    pub item_id: SolItemId,
    pub rig_size: Option<AttrVal>,
}
impl SolRigSizeMismatch {
    fn new(item_id: SolItemId, rig_size: Option<AttrVal>) -> Self {
        Self { item_id, rig_size }
    }
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_rig_size_fast(&self, ship: Option<&SolShip>) -> bool {
        let allowed_size = match get_allowed_size(ship) {
            Some(allowed_size) => allowed_size,
            None => return true,
        };
        for &rig_size in self.rigs_rig_size.values() {
            if rig_size != Some(allowed_size) {
                return false;
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_rig_size_verbose(
        &self,
        ship: Option<&SolShip>,
    ) -> Option<SolRigSizeValFail> {
        let allowed_size = get_allowed_size(ship)?;
        let mut mismatches = Vec::new();
        for (&item_id, &rig_size) in self.rigs_rig_size.iter() {
            if rig_size != Some(allowed_size) {
                mismatches.push(SolRigSizeMismatch::new(item_id, rig_size))
            }
        }
        match mismatches.is_empty() {
            true => None,
            false => Some(SolRigSizeValFail::new(allowed_size, mismatches)),
        }
    }
}

fn get_allowed_size(ship: Option<&SolShip>) -> Option<AttrVal> {
    ship?.get_attrs()?.get(&ec::attrs::RIG_SIZE).copied()
}
