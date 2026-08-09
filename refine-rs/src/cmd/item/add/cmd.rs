use crate::{
    AddedItemIdsResp, ItemAddBoosterCmd, ItemAddDroneCmd, ItemAddFighterCmd, ItemAddFwEffectCmd, ItemAddImplantCmd,
    ItemAddModuleCmd, ItemAddProjEffectCmd, ItemAddRigCmd, ItemAddServiceCmd, ItemAddSkillCmd, ItemAddSubsystemCmd,
    ItemAddSwEffectCmd, ItemSetCharacterCmd, ItemSetShipCmd, ItemSetStanceCmd,
    err::{
        AddProjEffectError, GetFitAddBoosterError, GetFitAddDroneError, GetFitAddFighterError, GetFitAddFwEffectError,
        GetFitAddImplantError, GetFitAddModuleError, GetFitAddRigError, GetFitAddServiceError, GetFitAddSkillError,
        GetFitAddSubsystemError, GetFitSetCharacterError, GetFitSetShipError, GetFitSetStanceError,
    },
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
pub enum AddItemEnumCmd {
    Booster(ItemAddBoosterCmd),
    Character(ItemSetCharacterCmd),
    Drone(ItemAddDroneCmd),
    Fighter(ItemAddFighterCmd),
    FwEffect(ItemAddFwEffectCmd),
    Implant(ItemAddImplantCmd),
    Module(ItemAddModuleCmd),
    ProjEffect(ItemAddProjEffectCmd),
    Rig(ItemAddRigCmd),
    Service(ItemAddServiceCmd),
    Ship(ItemSetShipCmd),
    Skill(ItemAddSkillCmd),
    Stance(ItemSetStanceCmd),
    Subsystem(ItemAddSubsystemCmd),
    SwEffect(ItemAddSwEffectCmd),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AddItemEnumCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<AddedItemIdsResp, AddItemEnumError> {
        match self {
            Self::Booster(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::Character(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::Drone(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::Fighter(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::FwEffect(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::Implant(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::Module(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::ProjEffect(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::Rig(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::Service(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::Ship(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::Skill(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::Stance(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::Subsystem(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::SwEffect(cmd) => Ok(cmd.inner.execute(core_sol)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddItemEnumError {
    #[error("failed to add booster")]
    BoosterFailed(#[from] GetFitAddBoosterError),
    #[error("failed to set character")]
    CharacterFailed(#[from] GetFitSetCharacterError),
    #[error("failed to add drone")]
    DroneFailed(#[from] GetFitAddDroneError),
    #[error("failed to add fighter")]
    FighterFailed(#[from] GetFitAddFighterError),
    #[error("failed to add fit-wide effect")]
    FwEffectFailed(#[from] GetFitAddFwEffectError),
    #[error("failed to add implant")]
    ImplantFailed(#[from] GetFitAddImplantError),
    #[error("failed to add module")]
    ModuleFailed(#[from] GetFitAddModuleError),
    #[error("failed to add projected effect")]
    ProjEffectFailed(#[from] AddProjEffectError),
    #[error("failed to add rig")]
    RigFailed(#[from] GetFitAddRigError),
    #[error("failed to add service")]
    ServiceFailed(#[from] GetFitAddServiceError),
    #[error("failed to set ship")]
    ShipFailed(#[from] GetFitSetShipError),
    #[error("failed to add skill")]
    SkillFailed(#[from] GetFitAddSkillError),
    #[error("failed to set stance")]
    StanceFailed(#[from] GetFitSetStanceError),
    #[error("failed to add subsystem")]
    SubsystemFailed(#[from] GetFitAddSubsystemError),
}
