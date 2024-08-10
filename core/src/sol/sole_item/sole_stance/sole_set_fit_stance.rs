use crate::{
    defs::{EItemId, SolFitId},
    err::{FitFoundError, ItemAllocError},
    sol::{
        item::{SolItem, SolStance},
        item_info::SolStanceInfo,
        SolView, SolarSystem,
    },
};

impl SolarSystem {
    // TODO: rewrite so that when allocation fails, nothing changes
    pub fn set_fit_stance(
        &mut self,
        fit_id: SolFitId,
        a_item_id: EItemId,
        state: bool,
    ) -> Result<SolStanceInfo, SetFitStanceError> {
        let fit = self.fits.get_fit(&fit_id)?;
        // Remove old stance, if it was set
        if let Some(old_item_id) = fit.stance {
            let old_item = self.items.get_item(&old_item_id).unwrap();
            self.svcs.remove_item(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                old_item,
            );
            self.items.remove_item(&old_item_id);
        }
        // Add new stance
        let item_id = self.items.alloc_item_id()?;
        let stance = SolStance::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SolStanceInfo::from(&stance);
        let item = SolItem::Stance(stance);
        let fit = self.fits.get_fit_mut(&fit_id).unwrap();
        fit.stance = Some(item_id);
        self.items.add_item(item);
        self.add_item_id_to_svcs(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum SetFitStanceError {
    FitNotFound(FitFoundError),
    ItemIdAllocFailed(ItemAllocError),
}
impl From<FitFoundError> for SetFitStanceError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<ItemAllocError> for SetFitStanceError {
    fn from(error: ItemAllocError) -> Self {
        Self::ItemIdAllocFailed(error)
    }
}
impl std::error::Error for SetFitStanceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::ItemIdAllocFailed(e) => Some(e),
        }
    }
}
impl std::fmt::Display for SetFitStanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::ItemIdAllocFailed(e) => e.fmt(f),
        }
    }
}
