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
            Self::Booster(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::Character(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::Drone(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::Fighter(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::FwEffect(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::Implant(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::Module(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::ProjEffect(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::Rig(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::Service(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::Ship(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::Skill(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::Stance(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::Subsystem(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            Self::SwEffect(cmd) => Ok(cmd.inner.execute(core_sol).into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddItemEnumError {
    #[error("failed to add booster: {0}")]
    BoosterFailed(#[from] GetFitAddBoosterError),
    #[error("failed to set character: {0}")]
    CharacterFailed(#[from] GetFitSetCharacterError),
    #[error("failed to add drone: {0}")]
    DroneFailed(#[from] GetFitAddDroneError),
    #[error("failed to add fighter: {0}")]
    FighterFailed(#[from] GetFitAddFighterError),
    #[error("failed to add fit-wide effect: {0}")]
    FwEffectFailed(#[from] GetFitAddFwEffectError),
    #[error("failed to add implant: {0}")]
    ImplantFailed(#[from] GetFitAddImplantError),
    #[error("failed to add module: {0}")]
    ModuleFailed(#[from] GetFitAddModuleError),
    #[error("failed to add projected effect: {0}")]
    ProjEffect(#[from] AddProjEffectError),
    #[error("failed to add rig: {0}")]
    RigFailed(#[from] GetFitAddRigError),
    #[error("failed to add service: {0}")]
    ServiceFailed(#[from] GetFitAddServiceError),
    #[error("failed to set ship: {0}")]
    ShipFailed(#[from] GetFitSetShipError),
    #[error("failed to add skill: {0}")]
    SkillFailed(#[from] GetFitAddSkillError),
    #[error("failed to set stance: {0}")]
    StanceFailed(#[from] GetFitSetStanceError),
    #[error("failed to add subsystem: {0}")]
    SubsystemFailed(#[from] GetFitAddSubsystemError),
}
