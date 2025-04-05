use std::collections::HashMap;

use crate::{
    sol::{Count, ItemId, svc::vast::VastFitData},
    util::RSet,
};

pub struct ValFighterSquadSizeFail {
    /// Map between fighter squad item IDs and info about failed validation.
    pub fighters: HashMap<ItemId, ValFighterSquadSizeFighterInfo>,
}

#[derive(Copy, Clone)]
pub struct ValFighterSquadSizeFighterInfo {
    /// Current squad size.
    pub size: Count,
    /// Max allowed squad size.
    pub max_size: Count,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_fighter_squad_size_fast(&mut self, kfs: &RSet<ItemId>) -> bool {
        match kfs.is_empty() {
            true => self.fighter_squad_size.is_empty(),
            false => self.fighter_squad_size.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_fighter_squad_size_verbose(
        &mut self,
        kfs: &RSet<ItemId>,
    ) -> Option<ValFighterSquadSizeFail> {
        let fighters: HashMap<_, _> = self
            .fighter_squad_size
            .iter()
            .filter(|(k, v)| !kfs.contains(k))
            .map(|(k, v)| (*k, *v))
            .collect();
        if fighters.is_empty() {
            return None;
        }
        Some(ValFighterSquadSizeFail { fighters })
    }
}
