use crate::{
    defs::{SlotIndex, SolItemId},
    sol::svc::vast::SolVastFitData,
    util::StMapSetL1,
};

pub struct SolSlotIndexValFail {
    pub slot: SlotIndex,
    pub users: Vec<SolItemId>,
}
impl SolSlotIndexValFail {
    fn new(slot: SlotIndex, users: Vec<SolItemId>) -> Self {
        Self { slot, users }
    }
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_implant_slot_index_fast(&self) -> bool {
        self.slotted_implants.values().all(|v| v.len() < 2)
    }
    pub(in crate::sol::svc::vast) fn validate_booster_slot_index_fast(&self) -> bool {
        self.slotted_boosters.values().all(|v| v.len() < 2)
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_index_fast(&self) -> bool {
        self.slotted_subsystems.values().all(|v| v.len() < 2)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_implant_slot_index_verbose(&self) -> Vec<SolSlotIndexValFail> {
        validate_slot_index_verbose(&self.slotted_implants)
    }
    pub(in crate::sol::svc::vast) fn validate_booster_slot_index_verbose(&self) -> Vec<SolSlotIndexValFail> {
        validate_slot_index_verbose(&self.slotted_boosters)
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_index_verbose(&self) -> Vec<SolSlotIndexValFail> {
        validate_slot_index_verbose(&self.slotted_subsystems)
    }
}

fn validate_slot_index_verbose(data: &StMapSetL1<SlotIndex, SolItemId>) -> Vec<SolSlotIndexValFail> {
    let mut fails = Vec::new();
    for (slot, users) in data.iter() {
        if users.len() >= 2 {
            fails.push(SolSlotIndexValFail::new(*slot, users.map(|v| *v).collect()))
        }
    }
    fails
}
