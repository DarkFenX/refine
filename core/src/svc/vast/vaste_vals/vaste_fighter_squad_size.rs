use std::collections::HashMap;

use crate::{
    def::{Count, ItemId, ItemKey},
    svc::{SvcCtx, vast::VastFitData},
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
    pub(in crate::svc::vast) fn validate_fighter_squad_size_fast(&mut self, kfs: &RSet<ItemKey>) -> bool {
        match kfs.is_empty() {
            true => self.fighter_squad_size.is_empty(),
            false => self.fighter_squad_size.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_fighter_squad_size_verbose(
        &mut self,
        kfs: &RSet<ItemKey>,
        ctx: SvcCtx,
    ) -> Option<ValFighterSquadSizeFail> {
        let fighters: HashMap<_, _> = self
            .fighter_squad_size
            .iter()
            .filter(|(fighter_key, _)| !kfs.contains(fighter_key))
            .map(|(fighter_key, fighter_info)| (ctx.uad.items.id_by_key(*fighter_key), *fighter_info))
            .collect();
        match fighters.is_empty() {
            true => None,
            false => Some(ValFighterSquadSizeFail { fighters }),
        }
    }
}
