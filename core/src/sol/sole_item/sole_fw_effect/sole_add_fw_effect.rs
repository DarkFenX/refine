use crate::{
    defs::{EItemId, SolFitId},
    sol::{
        err::basic::{FitFoundError, ItemAllocError},
        item::{SolFwEffect, SolItem},
        item_info::SolFwEffectInfo,
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn add_fw_effect(
        &mut self,
        fit_id: SolFitId,
        a_item_id: EItemId,
        state: bool,
    ) -> Result<SolFwEffectInfo, AddFwEffectError> {
        let item_id = self.items.alloc_item_id()?;
        let fw_effect = SolFwEffect::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SolFwEffectInfo::from(&fw_effect);
        let item = SolItem::FwEffect(fw_effect);
        let fit = self.fits.get_fit_mut(&fit_id)?;
        fit.fw_effects.insert(item_id);
        self.items.add_item(item);
        self.add_item_id_to_svcs(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum AddFwEffectError {
    FitNotFound(FitFoundError),
    ItemIdAllocFailed(ItemAllocError),
}
impl From<FitFoundError> for AddFwEffectError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<ItemAllocError> for AddFwEffectError {
    fn from(error: ItemAllocError) -> Self {
        Self::ItemIdAllocFailed(error)
    }
}
impl std::error::Error for AddFwEffectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::ItemIdAllocFailed(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddFwEffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::ItemIdAllocFailed(e) => e.fmt(f),
        }
    }
}
