use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, ItemTypeId, SolarSystem,
        info::RigInfo,
        uad::item::{Item, Rig},
    },
};

impl SolarSystem {
    pub fn add_rig(&mut self, fit_id: FitId, type_id: ItemTypeId, state: bool) -> Result<RigInfo, AddRigError> {
        let item_id = self.uad.items.alloc_item_id();
        let rig = Rig::new(&self.uad.src, item_id, type_id, fit_id, state);
        let info = RigInfo::from(&rig);
        let item = Item::Rig(rig);
        let fit = self.uad.fits.get_fit_mut(&fit_id)?;
        fit.rigs.insert(item_id);
        self.uad.items.add_item(item);
        self.add_item_id_to_svc(&item_id);
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
