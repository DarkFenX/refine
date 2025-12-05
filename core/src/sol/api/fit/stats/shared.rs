use crate::{
    def::ItemId,
    err::basic::{
        FitHasCharacterError, FitHasShipError, ItemFoundError, ItemLoadedError, ItemReceiveProjError,
        SupportedStatError,
    },
    sol::api::{CharacterMut, FitMut, ItemStatError, ShipMut},
    ud::UItemKey,
};

impl<'a> FitMut<'a> {
    pub(super) fn get_character_for_stats(&mut self) -> Result<CharacterMut<'_>, FitHasCharacterError> {
        let char_key = match self.sol.u_data.fits.get(self.key).character {
            Some(char_key) => char_key,
            None => {
                return Err(FitHasCharacterError {
                    fit_id: self.sol.u_data.fits.id_by_key(self.key),
                });
            }
        };
        Ok(CharacterMut::new(self.sol, char_key))
    }
    pub(super) fn get_ship_for_stats(&mut self) -> Result<ShipMut<'_>, FitHasShipError> {
        let ship_key = match self.sol.u_data.fits.get(self.key).ship {
            Some(ship_key) => ship_key,
            None => {
                return Err(FitHasShipError {
                    fit_id: self.sol.u_data.fits.id_by_key(self.key),
                });
            }
        };
        Ok(ShipMut::new(self.sol, ship_key))
    }
    pub(super) fn get_stat_applied_projectee_key(
        &self,
        projectee_item_id: &ItemId,
    ) -> Result<UItemKey, FitStatAppliedError> {
        let projectee_key = self.sol.u_data.items.key_by_id_err(projectee_item_id)?;
        let projectee_u_item = self.sol.u_data.items.get(projectee_key);
        if projectee_u_item.get_direct_physics().is_none() {
            return Err(ItemReceiveProjError {
                item_id: projectee_u_item.get_item_id(),
                item_kind: projectee_u_item.get_name(),
            }
            .into());
        }
        Ok(projectee_key)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitShipStatError {
    #[error("{0}")]
    NoShip(#[from] FitHasShipError),
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error("{0}")]
    UnsupportedStat(#[from] SupportedStatError),
}
impl From<ItemStatError> for FitShipStatError {
    fn from(item_err: ItemStatError) -> Self {
        match item_err {
            ItemStatError::ItemNotLoaded(e) => e.into(),
            ItemStatError::UnsupportedStat(e) => e.into(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitCharacterStatError {
    #[error("{0}")]
    NoCharacter(#[from] FitHasCharacterError),
    #[error("{0}")]
    ItemNotLoaded(#[from] ItemLoadedError),
    #[error("{0}")]
    UnsupportedStat(#[from] SupportedStatError),
}
impl From<ItemStatError> for FitCharacterStatError {
    fn from(item_err: ItemStatError) -> Self {
        match item_err {
            ItemStatError::ItemNotLoaded(e) => e.into(),
            ItemStatError::UnsupportedStat(e) => e.into(),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitStatAppliedError {
    #[error("{0}")]
    ProjecteeNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ProjecteeCantTakeProjs(#[from] ItemReceiveProjError),
}
