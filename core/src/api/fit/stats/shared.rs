use crate::{
    api::{CharacterMut, FitMut, ItemStatError, ShipMut},
    err::basic::{
        FitHasCharacterError, FitHasShipError, ItemFoundError, ItemLoadedError, ItemReceiveProjError,
        SupportedStatError,
    },
    ud::{ItemId, UItemId},
};

impl<'a> FitMut<'a> {
    pub(super) fn get_character_for_stats(&mut self) -> Result<CharacterMut<'_>, FitHasCharacterError> {
        let char_uid = match self.sol.u_data.fits.get(self.uid).character {
            Some(char_uid) => char_uid,
            None => {
                return Err(FitHasCharacterError {
                    fit_id: self.sol.u_data.fits.xid_by_iid(self.uid),
                });
            }
        };
        Ok(CharacterMut::new(self.sol, char_uid))
    }
    pub(super) fn get_ship_for_stats(&mut self) -> Result<ShipMut<'_>, FitHasShipError> {
        let ship_uid = match self.sol.u_data.fits.get(self.uid).ship {
            Some(ship_uid) => ship_uid,
            None => {
                return Err(FitHasShipError {
                    fit_id: self.sol.u_data.fits.xid_by_iid(self.uid),
                });
            }
        };
        Ok(ShipMut::new(self.sol, ship_uid))
    }
    pub(super) fn get_stat_applied_projectee_uid(
        &self,
        projectee_item_id: &ItemId,
    ) -> Result<UItemId, FitStatAppliedError> {
        let projectee_uid = self.sol.u_data.items.iid_by_xid_err(projectee_item_id)?;
        let projectee_u_item = self.sol.u_data.items.get(projectee_uid);
        if projectee_u_item.get_direct_physics().is_none() {
            return Err(ItemReceiveProjError {
                item_id: projectee_u_item.get_item_id(),
                item_kind: projectee_u_item.lib_get_name(),
            }
            .into());
        }
        Ok(projectee_uid)
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
