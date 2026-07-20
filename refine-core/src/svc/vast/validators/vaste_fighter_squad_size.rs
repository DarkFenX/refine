use itertools::Itertools;

use crate::{
    num::CountNz,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UItemId},
    util::RSet,
};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
pub struct ValFighterSquadSizeFail {
    /// Fighters and info about failed validation.
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::Map<_, _>"))]
    pub fighters: Vec<(ItemId, ValFighterSquadSizeFighterInfo)>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ValFighterSquadSizeFighterInfo {
    /// Current squad size.
    pub size: CountNz,
    /// Max allowed squad size.
    pub max_size: CountNz,
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
        let fighters = self
            .fighter_squad_size
            .iter()
            .filter(|(fighter_uid, _)| !kfs.contains(fighter_uid))
            .map(|(fighter_uid, fighter_info)| (ctx.u_data.items.ext_id_by_int_id(*fighter_uid), *fighter_info))
            .collect_vec();
        match fighters.is_empty() {
            true => None,
            false => Some(ValFighterSquadSizeFail { fighters }),
        }
    }
}
