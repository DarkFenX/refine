use crate::{
    defs::{Count, SolItemId},
    sol::svc::vast::SolVastFitData,
    util::StSet,
};

#[derive(Copy, Clone)]
pub struct SolValFighterCountFail {
    pub item_id: SolItemId,
    pub count: Count,
    pub max_count: Count,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_fighter_count_fast(&mut self, kfs: &StSet<SolItemId>) -> bool {
        match kfs.is_empty() {
            true => self.fighter_count.is_empty(),
            false => self.fighter_count.difference(kfs).nth(0).is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_fighter_count_verbose(
        &mut self,
        kfs: &StSet<SolItemId>,
    ) -> Vec<SolValFighterCountFail> {
        self.fighter_count
            .values()
            .filter(|v| !kfs.contains(&v.item_id))
            .copied()
            .collect()
    }
}
