use std::collections::HashMap;

use crate::{
    num::FighterCount,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UItemId},
    util::RSet,
};

pub struct ValFighterSquadSizeFail {
    /// Map between fighter squad item IDs and info about failed validation.
    pub fighters: HashMap<ItemId, ValFighterSquadSizeFighterInfo>,
}

#[derive(Copy, Clone)]
pub struct ValFighterSquadSizeFighterInfo {
    /// Current squad size.
    pub size: FighterCount,
    /// Max allowed squad size.
    pub max_size: FighterCount,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_fighter_squad_size_fast(&mut self, kfs: &RSet<UItemId>) -> bool {
        match kfs.is_empty() {
            true => self.fighter_squad_size.is_empty(),
            false => self.fighter_squad_size.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_fighter_squad_size_verbose(
        &mut self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValFighterSquadSizeFail> {
        let fighters: HashMap<_, _> = self
            .fighter_squad_size
            .iter()
            .filter(|(fighter_uid, _)| !kfs.contains(fighter_uid))
            .map(|(fighter_uid, fighter_info)| (ctx.u_data.items.xid_by_iid(*fighter_uid), *fighter_info))
            .collect();
        match fighters.is_empty() {
            true => None,
            false => Some(ValFighterSquadSizeFail { fighters }),
        }
    }
}
