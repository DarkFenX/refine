use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, ItemKey, ItemTypeId, SolarSystem,
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
        let item_key = self.add_fighter_internal(fit_id, type_id, state)?;
        Ok(self.get_fighter_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn add_fighter_internal(
        &mut self,
        fit_id: FitId,
        type_id: ItemTypeId,
        state: MinionState,
    ) -> Result<ItemKey, FitFoundError> {
        let fit = self.uad.fits.get_fit_mut(&fit_id)?;
        let item_id = self.uad.items.alloc_item_id();
        let fighter = Fighter::new(&self.uad.src, item_id, type_id, fit_id, state);
        let item = Item::Fighter(fighter);
        let item_key = self.uad.items.add(item);
        fit.fighters.insert(item_key);
        self.add_item_autocharges(item_key);
        // Add fighter and autocharges to services
        let item = self.uad.items.get(item_key);
        self.svc.add_item(&self.uad, item_key, item);
        for &autocharge_key in item.get_fighter().unwrap().get_autocharges().values() {
            let autocharge_item = self.uad.items.get(autocharge_key);
            self.svc.add_item(&self.uad, autocharge_key, autocharge_item);
        }
        Ok(item_key)
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
