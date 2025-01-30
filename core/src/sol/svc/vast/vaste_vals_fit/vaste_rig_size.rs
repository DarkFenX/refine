use crate::{
    defs::{AttrVal, SolItemId},
    ec,
    sol::{
        svc::vast::SolVastFitData,
        uad::{fit::SolFit, SolUad},
    },
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
    pub(in crate::sol::svc::vast) fn validate_rig_size_fast(&self, uad: &SolUad, fit: &SolFit) -> bool {
        let allowed_size = match get_allowed_size(uad, fit) {
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
        uad: &SolUad,
        fit: &SolFit,
    ) -> Option<SolRigSizeValFail> {
        let allowed_size = match get_allowed_size(uad, fit) {
            Some(allowed_size) => allowed_size,
            None => return None,
        };
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

fn get_allowed_size(uad: &SolUad, fit: &SolFit) -> Option<AttrVal> {
    let ship = match fit.ship {
        Some(ship_id) => uad.items.get_item(&ship_id).unwrap(),
        None => return None,
    };
    let ship_attrs = match ship.get_attrs() {
        Some(attrs) => attrs,
        None => return None,
    };
    match ship_attrs.get(&ec::attrs::RIG_SIZE) {
        Some(val) => Some(*val),
        None => None,
    }
}
