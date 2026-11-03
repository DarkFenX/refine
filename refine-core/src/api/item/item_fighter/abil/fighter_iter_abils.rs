use lender::{ExactSizeLender, Lender, Lending, check_covariance};

use crate::{
    ad::AAbilId,
    api::{Ability, Fighter},
    sol::SolarSystem,
    ud::UItemId,
};

/// Lending iterator for fighter abilities
pub struct AbilityIter<'iter> {
    sol: &'iter mut SolarSystem,
    fighter_uid: UItemId,
    abil_aids: Vec<AAbilId>,
    index: usize,
}
impl<'iter> AbilityIter<'iter> {
    fn new(sol: &'iter mut SolarSystem, fighter_uid: UItemId, abil_aids: Vec<AAbilId>) -> Self {
        Self {
            sol,
            fighter_uid,
            abil_aids,
            index: 0,
        }
    }
}
impl<'iter, 'lend> Lending<'lend> for AbilityIter<'iter> {
    type Lend = Ability<'lend>;
}
impl<'iter> Lender for AbilityIter<'iter> {
    check_covariance!();

    fn next(&mut self) -> Option<Ability<'_>> {
        let abil_aid = *self.abil_aids.get(self.index)?;
        self.index += 1;
        Some(Ability::new(self.sol, self.fighter_uid, abil_aid))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.abil_aids.len() - self.index;
        (remaining, Some(remaining))
    }
}
impl<'iter> ExactSizeLender for AbilityIter<'iter> {}

impl<'s> Fighter<'s> {
    /// Iterates over fighter's abilities.
    pub fn iter_abilities(&mut self) -> AbilityIter<'_> {
        let abil_aids = match self
            .sol
            .u_data
            .items
            .get(self.uid)
            .dc_fighter()
            .unwrap()
            .get_r_item_base()
        {
            Some(rib) => rib.abil_ids.clone(),
            None => Vec::new(),
        };
        AbilityIter::new(self.sol, self.uid, abil_aids)
    }
}
