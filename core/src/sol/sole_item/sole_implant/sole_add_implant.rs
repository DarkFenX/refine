use crate::{
    defs::{EItemId, SolFitId},
    err::{FitFoundError, ItemAllocError},
    sol::{
        item::{SolImplant, SolItem},
        item_info::SolImplantInfo,
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn add_implant(
        &mut self,
        fit_id: SolFitId,
        a_item_id: EItemId,
        state: bool,
    ) -> Result<SolImplantInfo, AddImplantError> {
        let item_id = self.items.alloc_item_id()?;
        let implant = SolImplant::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SolImplantInfo::from(&implant);
        let item = SolItem::Implant(implant);
        let fit = self.fits.get_fit_mut(&fit_id)?;
        fit.implants.insert(item_id);
        self.items.add_item(item);
        self.add_item_id_to_svcs(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum AddImplantError {
    FitNotFound(FitFoundError),
    ItemIdAllocFailed(ItemAllocError),
}
impl From<FitFoundError> for AddImplantError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<ItemAllocError> for AddImplantError {
    fn from(error: ItemAllocError) -> Self {
        Self::ItemIdAllocFailed(error)
    }
}
impl std::error::Error for AddImplantError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::ItemIdAllocFailed(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddImplantError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::ItemIdAllocFailed(e) => e.fmt(f),
        }
    }
}
