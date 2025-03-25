use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, ItemTypeId, SolarSystem,
        info::StanceInfo,
        uad::item::{Item, Stance},
    },
};

impl SolarSystem {
    pub fn set_fit_stance(
        &mut self,
        fit_id: FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<StanceInfo, SetFitStanceError> {
        let fit = self.uad.fits.get_fit(&fit_id)?;
        // Remove old stance, if it was set
        if let Some(old_item_id) = fit.stance {
            // Update services
            self.remove_item_id_from_svc(&old_item_id);
            // Update user data - do not touch fit, since it will be changed later
            self.uad.items.remove_item(&old_item_id);
        }
        // Add new stance
        let item_id = self.uad.items.alloc_item_id();
        let stance = Stance::new(&self.uad.src, item_id, type_id, fit_id, state);
        let info = StanceInfo::from(&stance);
        let item = Item::Stance(stance);
        let fit = self.uad.fits.get_fit_mut(&fit_id).unwrap();
        fit.stance = Some(item_id);
        self.uad.items.add_item(item);
        self.add_item_id_to_svc(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum SetFitStanceError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for SetFitStanceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitStanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for SetFitStanceError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
