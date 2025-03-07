use crate::{
    defs::{Count, SolItemId},
    sol::svc::vast::SolVastFitData,
};

#[derive(Copy, Clone)]
pub struct SolValFighterCountFail {
    pub item_id: SolItemId,
    pub count: Count,
    pub max_count: Count,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_fighter_count_fast(&mut self) -> bool {
        self.fighter_count.is_empty()
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_fighter_count_verbose(&mut self) -> Vec<SolValFighterCountFail> {
        self.fighter_count.values().copied().collect()
    }
}
