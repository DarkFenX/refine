use crate::{
    ChangedItemIdsResp, ItemChangeAutochargeCmd, ItemChangeBoosterCmd, ItemChangeCharacterCmd, ItemChangeChargeCmd,
    ItemChangeDroneCmd, ItemChangeFighterCmd, ItemChangeFwEffectCmd, ItemChangeImplantCmd, ItemChangeModuleCmd,
    ItemChangeProjEffectCmd, ItemChangeRigCmd, ItemChangeServiceCmd, ItemChangeShipCmd, ItemChangeSkillCmd,
    ItemChangeStanceCmd, ItemChangeSubsystemCmd, ItemChangeSwEffectCmd,
    err::{
        ItemChangeAutochargeError, ItemChangeBoosterError, ItemChangeCharacterError, ItemChangeChargeError,
        ItemChangeDroneError, ItemChangeFighterError, ItemChangeFwEffectError, ItemChangeImplantError,
        ItemChangeModuleError, ItemChangeProjEffectError, ItemChangeRigError, ItemChangeServiceError,
        ItemChangeShipError, ItemChangeSkillError, ItemChangeStanceError, ItemChangeSubsystemError,
        ItemChangeSwEffectError,
    },
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
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
    Skill(ItemChangeSkillCmd),
    Stance(ItemChangeStanceCmd),
    Subsystem(ItemChangeSubsystemCmd),
    SwEffect(ItemChangeSwEffectCmd),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChangeItemEnumCmd {
    pub(crate) fn execute(self, core_item: &mut rc::ItemMut) -> Result<ChangedItemIdsResp, ChangeItemEnumError> {
        match self {
            Self::Autocharge(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::Booster(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::Character(cmd) => Ok(cmd.inner.execute_via_item(core_item)?),
            Self::Charge(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::Drone(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::Fighter(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::FwEffect(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::Implant(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::Module(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::ProjEffect(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::Rig(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::Service(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::Ship(cmd) => Ok(cmd.inner.execute_via_item(core_item)?),
            Self::Skill(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::Stance(cmd) => Ok(cmd.inner.execute_via_item(core_item)?),
            Self::Subsystem(cmd) => Ok(cmd.inner.execute(core_item)?),
            Self::SwEffect(cmd) => Ok(cmd.inner.execute(core_item)?),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeItemEnumError {
    #[error("failed to change autocharge")]
    AutochargeFailed(#[from] ItemChangeAutochargeError),
    #[error("failed to change booster")]
    BoosterFailed(#[from] ItemChangeBoosterError),
    #[error("failed to change character")]
    CharacterFailed(#[from] ItemChangeCharacterError),
    #[error("failed to change charge")]
    ChargeFailed(#[from] ItemChangeChargeError),
    #[error("failed to change drone")]
    DroneFailed(#[from] ItemChangeDroneError),
    #[error("failed to change fighter")]
    FighterFailed(#[from] ItemChangeFighterError),
    #[error("failed to change fit-wide effect")]
    FwEffectFailed(#[from] ItemChangeFwEffectError),
    #[error("failed to change implant")]
    ImplantFailed(#[from] ItemChangeImplantError),
    #[error("failed to change module")]
    ModuleFailed(#[from] ItemChangeModuleError),
    #[error("failed to change projected effect")]
    ProjEffectFailed(#[from] ItemChangeProjEffectError),
    #[error("failed to change rig")]
    RigFailed(#[from] ItemChangeRigError),
    #[error("failed to change service")]
    ServiceFailed(#[from] ItemChangeServiceError),
    #[error("failed to change ship")]
    ShipFailed(#[from] ItemChangeShipError),
    #[error("failed to change skill")]
    SkillFailed(#[from] ItemChangeSkillError),
    #[error("failed to change stance")]
    StanceFailed(#[from] ItemChangeStanceError),
    #[error("failed to change subsystem")]
    SubsystemFailed(#[from] ItemChangeSubsystemError),
    #[error("failed to change system-wide effect")]
    SwEffectFailed(#[from] ItemChangeSwEffectError),
}
