use crate::cmd::{
    ItemChangeAutochargeCmd, ItemChangeAutochargeError, ItemChangeBoosterCmd, ItemChangeBoosterError,
    ItemChangeCharacterCmd, ItemChangeCharacterError, ItemChangeChargeCmd, ItemChangeChargeError, ItemChangeDroneCmd,
    ItemChangeDroneError, ItemChangeFighterCmd, ItemChangeFighterError, ItemChangeFwEffectCmd, ItemChangeFwEffectError,
    ItemChangeImplantCmd, ItemChangeImplantError, ItemChangeModuleCmd, ItemChangeModuleError, ItemChangeProjEffectCmd,
    ItemChangeProjEffectError, ItemChangeRigCmd, ItemChangeRigError, ItemChangeServiceCmd, ItemChangeServiceError,
    ItemChangeShipCmd, ItemChangeShipError, shared::ChangedItemIdsResp,
};

pub enum ChangeItemEnumCmd {
    Autocharge(ItemChangeAutochargeCmd),
    Booster(ItemChangeBoosterCmd),
    Character(ItemChangeCharacterCmd),
    Charge(ItemChangeChargeCmd),
    Drone(ItemChangeDroneCmd),
    Fighter(ItemChangeFighterCmd),
    FwEffect(ItemChangeFwEffectCmd),
    Implant(ItemChangeImplantCmd),
    Module(ItemChangeModuleCmd),
    ProjEffect(ItemChangeProjEffectCmd),
    Rig(ItemChangeRigCmd),
    Service(ItemChangeServiceCmd),
    Ship(ItemChangeShipCmd),
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
            Self::Fighter(cmd) => Ok(cmd.inner.execute(core_item)?.into()),
            Self::FwEffect(cmd) => Ok(cmd.inner.execute(core_item)?.into()),
            Self::Implant(cmd) => Ok(cmd.inner.execute(core_item)?.into()),
            Self::Module(cmd) => Ok(cmd.inner.execute(core_item)?.into()),
            Self::ProjEffect(cmd) => Ok(cmd.inner.execute(core_item)?.into()),
            Self::Rig(cmd) => Ok(cmd.inner.execute(core_item)?.into()),
            Self::Service(cmd) => Ok(cmd.inner.execute(core_item)?.into()),
            Self::Ship(cmd) => Ok(cmd.inner.execute_via_item(core_item)?.into()),
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
    #[error("failed to change fighter: {0}")]
    FighterFailed(#[from] ItemChangeFighterError),
    #[error("failed to change fit-wide effect: {0}")]
    FwEffectFailed(#[from] ItemChangeFwEffectError),
    #[error("failed to change implant: {0}")]
    ImplantFailed(#[from] ItemChangeImplantError),
    #[error("failed to change module: {0}")]
    ModuleFailed(#[from] ItemChangeModuleError),
    #[error("failed to change projected effect: {0}")]
    ProjEffectFailed(#[from] ItemChangeProjEffectError),
    #[error("failed to change rig: {0}")]
    RigFailed(#[from] ItemChangeRigError),
    #[error("failed to change service: {0}")]
    ServiceFailed(#[from] ItemChangeServiceError),
    #[error("failed to change ship: {0}")]
    ShipFailed(#[from] ItemChangeShipError),
}
