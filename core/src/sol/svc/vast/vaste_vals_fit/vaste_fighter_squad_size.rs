use crate::{
    defs::{Count, SolItemId},
    sol::svc::vast::SolVastFitData,
    util::StSet,
};

#[derive(Copy, Clone)]
pub struct SolValFighterSquadSizeFail {
    pub item_id: SolItemId,
    pub size: Count,
    pub max_size: Count,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_fighter_squad_size_fast(&mut self, kfs: &StSet<SolItemId>) -> bool {
        match kfs.is_empty() {
            true => self.fighter_squad_size.is_empty(),
            false => self.fighter_squad_size.difference(kfs).nth(0).is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_fighter_squad_size_verbose(
        &mut self,
        kfs: &StSet<SolItemId>,
    ) -> Vec<SolValFighterSquadSizeFail> {
        self.fighter_squad_size
            .values()
            .filter(|v| !kfs.contains(&v.item_id))
            .copied()
            .collect()
    }
}
