use crate::{
    sol::{Count, ItemId, svc::vast::VastFitData},
    util::StSet,
};

#[derive(Copy, Clone)]
pub struct ValFighterSquadSizeFail {
    pub item_id: ItemId,
    pub size: Count,
    pub max_size: Count,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_fighter_squad_size_fast(&mut self, kfs: &StSet<ItemId>) -> bool {
        match kfs.is_empty() {
            true => self.fighter_squad_size.is_empty(),
            false => self.fighter_squad_size.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_fighter_squad_size_verbose(
        &mut self,
        kfs: &StSet<ItemId>,
    ) -> Vec<ValFighterSquadSizeFail> {
        self.fighter_squad_size
            .values()
            .filter(|v| !kfs.contains(&v.item_id))
            .copied()
            .collect()
    }
}
