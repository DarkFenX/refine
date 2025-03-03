use crate::{
    defs::{EItemId, SolFitId},
    err::basic::FitFoundError,
    sol::{
        SolarSystem,
        info::SolSubsystemInfo,
        uad::item::{SolItem, SolSubsystem},
    },
};

impl SolarSystem {
    pub fn add_subsystem(
        &mut self,
        fit_id: SolFitId,
        type_id: EItemId,
        state: bool,
    ) -> Result<SolSubsystemInfo, AddSubsystemError> {
        let item_id = self.uad.items.alloc_item_id();
        let subsystem = SolSubsystem::new(&self.uad.src, item_id, type_id, fit_id, state);
        let info = SolSubsystemInfo::from(&subsystem);
        let item = SolItem::Subsystem(subsystem);
        let fit = self.uad.fits.get_fit_mut(&fit_id)?;
        fit.subsystems.insert(item_id);
        self.uad.items.add_item(item);
        self.add_item_id_to_svc(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum AddSubsystemError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for AddSubsystemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddSubsystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for AddSubsystemError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
