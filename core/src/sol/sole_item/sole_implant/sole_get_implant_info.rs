use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::ImplantInfo},
};

impl SolarSystem {
    pub fn get_implant_info(&self, item_id: &ItemId) -> Result<ImplantInfo, GetImplantInfoError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_implant_info_internal(item_key)?)
    }
    pub(in crate::sol) fn get_implant_info_internal(
        &self,
        item_key: ItemKey,
    ) -> Result<ImplantInfo, ItemKindMatchError> {
        let implant = self.uad.items.get(item_key).get_implant()?;
        Ok(ImplantInfo::from_implant(&self.uad, implant))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetImplantInfoError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotImplant(#[from] ItemKindMatchError),
}
