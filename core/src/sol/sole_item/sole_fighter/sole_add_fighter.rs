use itertools::Itertools;

use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, ItemTypeId, SolarSystem,
        info::FighterInfo,
        uad::item::{Fighter, Item, MinionState},
    },
};

impl SolarSystem {
    pub fn add_fighter(
        &mut self,
        fit_id: FitId,
        type_id: ItemTypeId,
        state: MinionState,
    ) -> Result<FighterInfo, AddFighterError> {
        // Do everything needed to reserve ID for fighter itself
        let item_id = self.uad.items.alloc_item_id();
        let fighter = Fighter::new(&self.uad.src, item_id, type_id, fit_id, state);
        let item = Item::Fighter(fighter);
        self.uad.items.add_item(item);
        // Reserve IDs for autocharges
        self.add_item_autocharges(&item_id);
        // Finalize updates of user data
        let fit = self.uad.fits.get_fit_mut(&fit_id)?;
        fit.fighters.insert(item_id);
        // Add fighter and autocharges to services
        self.add_item_id_to_svc(&item_id);
        let fighter = self.uad.items.get_item(&item_id).unwrap().get_fighter().unwrap();
        for autocharge_id in fighter.get_autocharges().values().copied().collect_vec() {
            self.add_item_id_to_svc(&autocharge_id);
        }
        // Make info
        let info = self.get_fighter(&item_id).unwrap();
        Ok(info)
    }
}

#[derive(Debug)]
pub enum AddFighterError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for AddFighterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddFighterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddFighterError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
