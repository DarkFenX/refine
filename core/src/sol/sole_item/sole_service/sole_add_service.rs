use crate::{
    defs::{EItemId, SolFitId},
    err::basic::FitFoundError,
    sol::{
        SolarSystem,
        info::SolServiceInfo,
        uad::item::{SolItem, SolService},
    },
};

impl SolarSystem {
    pub fn add_service(
        &mut self,
        fit_id: SolFitId,
        type_id: EItemId,
        state: bool,
    ) -> Result<SolServiceInfo, AddServiceError> {
        let item_id = self.uad.items.alloc_item_id();
        let service = SolService::new(&self.uad.src, item_id, type_id, fit_id, state);
        let info = SolServiceInfo::from(&service);
        let item = SolItem::Service(service);
        let fit = self.uad.fits.get_fit_mut(&fit_id)?;
        fit.services.insert(item_id);
        self.uad.items.add_item(item);
        self.add_item_id_to_svc(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum AddServiceError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for AddServiceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddServiceError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
