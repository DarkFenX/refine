use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, FitKey, ItemKey, ItemTypeId, SolarSystem,
        info::RigInfo,
        uad::item::{UadItem, UadRig},
    },
};

impl SolarSystem {
    pub fn add_rig(&mut self, fit_id: &FitId, type_id: ItemTypeId, state: bool) -> Result<RigInfo, AddRigError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        let item_key = self.add_rig_internal(fit_key, type_id, state);
        Ok(self.get_rig_info_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn add_rig_internal(&mut self, fit_key: FitKey, type_id: ItemTypeId, state: bool) -> ItemKey {
        let item_id = self.uad.items.alloc_id();
        let rig = UadRig::new(&self.uad.src, item_id, type_id, fit_key, state);
        let item = UadItem::Rig(rig);
        let item_key = self.uad.items.add(item);
        let fit = self.uad.fits.get_mut(fit_key);
        fit.rigs.insert(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddRigError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
