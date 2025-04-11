use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::BoosterInfo},
};

impl SolarSystem {
    pub fn get_booster_info(&self, item_id: &ItemId) -> Result<BoosterInfo, GetBoosterInfoError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_booster_info_internal(item_key)?)
    }
    pub(in crate::sol) fn get_booster_info_internal(
        &self,
        item_key: ItemKey,
    ) -> Result<BoosterInfo, ItemKindMatchError> {
        let booster = self.uad.items.get(item_key).get_booster()?;
        Ok(BoosterInfo::from_booster(&self.uad, booster))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetBoosterInfoError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotBooster(#[from] ItemKindMatchError),
}
