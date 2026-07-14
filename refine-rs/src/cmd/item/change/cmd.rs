use crate::cmd::{
    ItemChangeAutochargeCmd, ItemChangeAutochargeError, ItemChangeBoosterCmd, ItemChangeBoosterError,
    ItemChangeCharacterCmd, ItemChangeCharacterError, ItemChangeChargeCmd, ItemChangeChargeError, ItemChangeDroneCmd,
    ItemChangeDroneError, shared::ChangedItemIdsResp,
};

pub enum ChangeItemEnumCmd {
    Autocharge(ItemChangeAutochargeCmd),
    Booster(ItemChangeBoosterCmd),
    Character(ItemChangeCharacterCmd),
    Charge(ItemChangeChargeCmd),
    Drone(ItemChangeDroneCmd),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChangeItemEnumCmd {
    pub(crate) fn execute(&self, core_item: &mut rc::ItemMut) -> Result<ChangedItemIdsResp, ChangeItemEnumError> {
        match self {
            Self::Autocharge(cmd) => Ok(cmd.inner.execute(core_item)?.into()),
            Self::Booster(cmd) => Ok(cmd.inner.execute(core_item)?.into()),
            Self::Character(cmd) => Ok(cmd.inner.execute_via_item(core_item)?.into()),
            Self::Charge(cmd) => Ok(cmd.inner.execute(core_item)?.into()),
            Self::Drone(cmd) => Ok(cmd.inner.execute(core_item)?.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeItemEnumError {
    #[error("failed to change autocharge: {0}")]
    AutochargeFailed(#[from] ItemChangeAutochargeError),
    #[error("failed to change booster: {0}")]
    BoosterFailed(#[from] ItemChangeBoosterError),
    #[error("failed to change character: {0}")]
    CharacterFailed(#[from] ItemChangeCharacterError),
    #[error("failed to change charge: {0}")]
    ChargeFailed(#[from] ItemChangeChargeError),
    #[error("failed to change drone: {0}")]
    DroneFailed(#[from] ItemChangeDroneError),
}
