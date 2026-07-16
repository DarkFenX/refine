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
            Self::Skill(cmd) => Ok(cmd.inner.execute(core_item)?.into()),
            Self::Stance(cmd) => Ok(cmd.inner.execute_via_item(core_item)?.into()),
            Self::Subsystem(cmd) => Ok(cmd.inner.execute(core_item)?.into()),
            Self::SwEffect(cmd) => Ok(cmd.inner.execute(core_item)?.into()),
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
    #[error("failed to change skill: {0}")]
    SkillFailed(#[from] ItemChangeSkillError),
    #[error("failed to change stance: {0}")]
    StanceFailed(#[from] ItemChangeStanceError),
    #[error("failed to change subsystem: {0}")]
    SubsystemFailed(#[from] ItemChangeSubsystemError),
    #[error("failed to change system-wide effect: {0}")]
    SwEffectFailed(#[from] ItemChangeSwEffectError),
}
