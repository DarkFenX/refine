use itertools::Itertools;

use crate::{
    CountNz, ItemId,
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemId,
    util::RSet,
};

#[derive(Copy, Clone)]
pub(in crate::svc::vast) struct ValFighterSquadSizeFighterStored {
    pub(in crate::svc::vast) size: CountNz,
    pub(in crate::svc::vast) max_size: CountNz,
}

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
#[derive(Clone)]
pub struct ValFighterSquadSizeFail {
    /// Fighters and info about failed validation.
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::KeyValueMap<_>"))]
    pub fighters: Vec<ValFighterSquadSizeFighterInfo>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ValFighterSquadSizeFighterInfo {
    /// Current squad size.
    pub fighter_id: ItemId,
    /// Current squad size.
    pub size: CountNz,
    /// Max allowed squad size.
    pub max_size: CountNz,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_fighter_squad_size_fast(&self, kfs: &RSet<UItemId>) -> bool {
        match kfs.is_empty() {
            true => self.fighter_squad_size.is_empty(),
            false => self.fighter_squad_size.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_fighter_squad_size_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValFighterSquadSizeFail> {
        let fighters = self
            .fighter_squad_size
            .iter()
            .filter_map(|(fighter_uid, fighter_info)| match kfs.contains(fighter_uid) {
                true => None,
                false => Some(ValFighterSquadSizeFighterInfo {
                    fighter_id: ctx.u_data.items.ext_id_by_int_id(*fighter_uid),
                    size: fighter_info.size,
                    max_size: fighter_info.max_size,
                }),
            })
            .collect_vec();
        match fighters.is_empty() {
            true => None,
            false => Some(ValFighterSquadSizeFail { fighters }),
        }
    }
}
