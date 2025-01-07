use crate::{
    defs::{EItemId, SolFitId},
    err::basic::FitFoundError,
    sol::{
        info::SolRigInfo,
        uad::item::{SolItem, SolRig},
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn add_rig(&mut self, fit_id: SolFitId, type_id: EItemId, state: bool) -> Result<SolRigInfo, AddRigError> {
        let item_id = self.uad.items.alloc_item_id();
        let rig = SolRig::new(&self.uad.src, item_id, type_id, fit_id, state);
        let info = SolRigInfo::from(&rig);
        let item = SolItem::Rig(rig);
        let fit = self.uad.fits.get_fit_mut(&fit_id)?;
        fit.rigs.insert(item_id);
        self.uad.items.add_item(item);
        self.add_item_id_to_svcs(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum AddRigError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for AddRigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddRigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddRigError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
