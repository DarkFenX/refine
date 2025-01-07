use crate::{
    defs::{EItemId, SolFitId},
    err::basic::FitFoundError,
    sol::{
        info::SolImplantInfo,
        uad::item::{SolImplant, SolItem},
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn add_implant(
        &mut self,
        fit_id: SolFitId,
        type_id: EItemId,
        state: bool,
    ) -> Result<SolImplantInfo, AddImplantError> {
        let item_id = self.uad.items.alloc_item_id();
        let implant = SolImplant::new(&self.uad.src, item_id, type_id, fit_id, state);
        let info = SolImplantInfo::from(&implant);
        let item = SolItem::Implant(implant);
        let fit = self.uad.fits.get_fit_mut(&fit_id)?;
        fit.implants.insert(item_id);
        self.uad.items.add_item(item);
        self.add_item_id_to_svcs(&item_id);
        Ok(info)
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
