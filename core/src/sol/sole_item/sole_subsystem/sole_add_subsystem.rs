use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, ItemId, ItemTypeId, SolarSystem,
        info::SubsystemInfo,
        uad::item::{Item, Subsystem},
    },
};

impl SolarSystem {
    pub fn add_subsystem(
        &mut self,
        fit_id: FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<SubsystemInfo, AddSubsystemError> {
        let item_id = self.add_subsystem_internal(fit_id, type_id, state)?;
        Ok(self.get_subsystem(&item_id).unwrap())
    }
    pub(in crate::sol) fn add_subsystem_internal(
        &mut self,
        fit_id: FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<ItemId, AddSubsystemError> {
        let item_id = self.uad.items.alloc_item_id();
        let subsystem = Subsystem::new(&self.uad.src, item_id, type_id, fit_id, state);
        let item = Item::Subsystem(subsystem);
        let fit = self.uad.fits.get_fit_mut(&fit_id)?;
        fit.subsystems.insert(item_id);
        self.uad.items.add_item(item);
        self.add_item_id_to_svc(&item_id);
        Ok(item_id)
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
