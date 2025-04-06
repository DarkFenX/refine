use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, ItemTypeId, SolarSystem,
        info::FwEffectInfo,
        uad::item::{FwEffect, Item},
    },
};

impl SolarSystem {
    pub fn add_fw_effect(
        &mut self,
        fit_id: FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<FwEffectInfo, AddFwEffectError> {
        let item_id = self.uad.items.alloc_item_id();
        let fw_effect = FwEffect::new(&self.uad.src, item_id, type_id, fit_id, state);
        let info = FwEffectInfo::from(&fw_effect);
        let item = Item::FwEffect(fw_effect);
        let fit = self.uad.fits.get_fit_mut(&fit_id)?;
        fit.fw_effects.insert(item_id);
        self.uad.items.add(item);
        self.add_item_id_to_svc(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum AddFwEffectError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for AddFwEffectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddFwEffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddFwEffectError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
