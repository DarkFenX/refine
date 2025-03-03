use crate::{
    defs::{EItemId, SolFitId},
    err::basic::FitFoundError,
    sol::{
        SolarSystem,
        info::SolBoosterInfo,
        uad::item::{SolBooster, SolItem},
    },
};

impl SolarSystem {
    pub fn add_booster(
        &mut self,
        fit_id: SolFitId,
        type_id: EItemId,
        state: bool,
    ) -> Result<SolBoosterInfo, AddBoosterError> {
        let item_id = self.uad.items.alloc_item_id();
        let booster = SolBooster::new(&self.uad.src, item_id, type_id, fit_id, state);
        let info = self.make_booster_info(&booster);
        let item = SolItem::Booster(booster);
        let fit = self.uad.fits.get_fit_mut(&fit_id)?;
        fit.boosters.insert(item_id);
        self.uad.items.add_item(item);
        self.add_item_id_to_svc(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum AddBoosterError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for AddBoosterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddBoosterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddBoosterError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
