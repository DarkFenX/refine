use crate::{
    AutochargeChangeCmd, ChangedItemIdsResp, ItemChangeBoosterCmd, ItemChangeCharacterCmd, ItemChangeChargeCmd,
    ItemChangeDroneCmd, ItemChangeFighterCmd, ItemChangeFwEffectCmd, ItemChangeImplantCmd, ItemChangeModuleCmd,
    ItemChangeProjEffectCmd, ItemChangeRigCmd, ItemChangeServiceCmd, ItemChangeShipCmd, ItemChangeSkillCmd,
    ItemChangeStanceCmd, ItemChangeSubsystemCmd, ItemChangeSwEffectCmd,
    err::{
        AutochargeChangeError, ItemChangeBoosterError, ItemChangeCharacterError, ItemChangeChargeError,
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
    Autocharge(AutochargeChangeCmd),
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
            Self::Autocharge(cmd) => Ok(cmd.execute(core_item)?),
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
    Autocharge(#[from] AutochargeChangeError),
    #[error("failed to change booster")]
    Booster(#[from] ItemChangeBoosterError),
    #[error("failed to change character")]
    Character(#[from] ItemChangeCharacterError),
    #[error("failed to change charge")]
    Charge(#[from] ItemChangeChargeError),
    #[error("failed to change drone")]
    Drone(#[from] ItemChangeDroneError),
    #[error("failed to change fighter")]
    Fighter(#[from] ItemChangeFighterError),
    #[error("failed to change fit-wide effect")]
    FwEffect(#[from] ItemChangeFwEffectError),
    #[error("failed to change implant")]
    Implant(#[from] ItemChangeImplantError),
    #[error("failed to change module")]
    Module(#[from] ItemChangeModuleError),
    #[error("failed to change projected effect")]
    ProjEffect(#[from] ItemChangeProjEffectError),
    #[error("failed to change rig")]
    Rig(#[from] ItemChangeRigError),
    #[error("failed to change service")]
    Service(#[from] ItemChangeServiceError),
    #[error("failed to change ship")]
    Ship(#[from] ItemChangeShipError),
    #[error("failed to change skill")]
    Skill(#[from] ItemChangeSkillError),
    #[error("failed to change stance")]
    Stance(#[from] ItemChangeStanceError),
    #[error("failed to change subsystem")]
    Subsystem(#[from] ItemChangeSubsystemError),
    #[error("failed to change system-wide effect")]
    SwEffect(#[from] ItemChangeSwEffectError),
}
