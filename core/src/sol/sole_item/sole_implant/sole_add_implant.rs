use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, ItemKey, ItemTypeId, SolarSystem,
        info::ImplantInfo,
        uad::item::{Implant, Item},
    },
};

impl SolarSystem {
    pub fn add_implant(
        &mut self,
        fit_id: FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<ImplantInfo, AddImplantError> {
        let item_key = self.add_implant_internal(fit_id, type_id, state)?;
        Ok(self.get_implant_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn add_implant_internal(
        &mut self,
        fit_id: FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<ItemKey, FitFoundError> {
        let fit = self.uad.fits.get_fit_mut(&fit_id)?;
        let item_id = self.uad.items.alloc_item_id();
        let implant = Implant::new(&self.uad.src, item_id, type_id, fit_id, state);
        let item = Item::Implant(implant);
        let item_key = self.uad.items.add(item);
        fit.implants.insert(item_key);
        self.add_item_key_to_svc(item_key);
        Ok(item_key)
    }
}

#[derive(Debug)]
pub enum AddImplantError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for AddImplantError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddImplantError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddImplantError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
