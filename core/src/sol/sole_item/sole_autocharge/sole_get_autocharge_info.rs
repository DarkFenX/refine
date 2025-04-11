use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::AutochargeInfo},
};

impl SolarSystem {
    pub fn get_autocharge_info(&self, item_id: &ItemId) -> Result<AutochargeInfo, GetAutochargeInfoError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_autocharge_info_internal(item_key)?)
    }
    pub(in crate::sol) fn get_autocharge_info_internal(
        &self,
        item_key: ItemKey,
    ) -> Result<AutochargeInfo, ItemKindMatchError> {
        let autocharge = self.uad.items.get(item_key).get_autocharge()?;
        Ok(AutochargeInfo::from_autocharge(&self.uad, autocharge))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetAutochargeInfoError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotAutocharge(#[from] ItemKindMatchError),
}
