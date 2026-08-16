use crate::{
    AddedItemIdsResp, BoosterAddCmd, FitId, ImplantAddCmd, ItemAddDroneCmd, ItemAddFighterCmd, ItemAddFwEffectCmd,
    ItemAddModuleCmd, ItemAddProjEffectCmd, ItemAddServiceCmd, ItemAddSkillCmd, ItemAddSubsystemCmd,
    ItemAddSwEffectCmd, ItemSetCharacterCmd, ItemSetShipCmd, ItemSetStanceCmd, RigAddCmd,
    ctl::core::{BoosterAddCmdCtxFit, ImplantAddCmdCtxFit, RigAddCmdCtxFit},
    err::{
        AddProjEffectError, FitGetBoosterAddError, FitGetImplantAddError, FitGetRigAddError, GetFitAddDroneError,
        GetFitAddFighterError, GetFitAddFwEffectError, GetFitAddModuleError, GetFitAddServiceError,
        GetFitAddSkillError, GetFitAddSubsystemError, GetFitSetCharacterError, GetFitSetShipError,
        GetFitSetStanceError,
    },
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
pub enum ItemAddCmd {
    Booster(BoosterAddCmdCtxFit),
    Character(ItemSetCharacterCmd),
    Drone(ItemAddDroneCmd),
    Fighter(ItemAddFighterCmd),
    FwEffect(ItemAddFwEffectCmd),
    Implant(ImplantAddCmdCtxFit),
    Module(ItemAddModuleCmd),
    ProjEffect(ItemAddProjEffectCmd),
    Rig(RigAddCmdCtxFit),
    Service(ItemAddServiceCmd),
    Ship(ItemSetShipCmd),
    Skill(ItemAddSkillCmd),
    Stance(ItemSetStanceCmd),
    Subsystem(ItemAddSubsystemCmd),
    SwEffect(ItemAddSwEffectCmd),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BoosterAddCmd {
    pub fn into_item_add(self, fit_id: FitId) -> ItemAddCmd {
        ItemAddCmd::Booster(self.into_ctx_fit(fit_id))
    }
}
impl ImplantAddCmd {
    pub fn into_item_add(self, fit_id: FitId) -> ItemAddCmd {
        ItemAddCmd::Implant(self.into_ctx_fit(fit_id))
    }
}
impl RigAddCmd {
    pub fn into_item_add(self, fit_id: FitId) -> ItemAddCmd {
        ItemAddCmd::Rig(self.into_ctx_fit(fit_id))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemAddCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<AddedItemIdsResp, ItemAddError> {
        match self {
            Self::Booster(cmd) => Ok(cmd.execute(core_sol)?),
            Self::Character(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::Drone(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::Fighter(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::FwEffect(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::Implant(cmd) => Ok(cmd.execute(core_sol)?),
            Self::Module(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::ProjEffect(cmd) => Ok(cmd.inner.execute(core_sol)?),
            Self::Rig(cmd) => Ok(cmd.execute(core_sol)?),
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
pub enum ItemAddError {
    #[error("failed to add booster")]
    Booster(#[from] FitGetBoosterAddError),
    #[error("failed to set character")]
    Character(#[from] GetFitSetCharacterError),
    #[error("failed to add drone")]
    Drone(#[from] GetFitAddDroneError),
    #[error("failed to add fighter")]
    Fighter(#[from] GetFitAddFighterError),
    #[error("failed to add fit-wide effect")]
    FwEffect(#[from] GetFitAddFwEffectError),
    #[error("failed to add implant")]
    Implant(#[from] FitGetImplantAddError),
    #[error("failed to add module")]
    Module(#[from] GetFitAddModuleError),
    #[error("failed to add projected effect")]
    ProjEffect(#[from] AddProjEffectError),
    #[error("failed to add rig")]
    Rig(#[from] FitGetRigAddError),
    #[error("failed to add service")]
    Service(#[from] GetFitAddServiceError),
    #[error("failed to set ship")]
    Ship(#[from] GetFitSetShipError),
    #[error("failed to add skill")]
    Skill(#[from] GetFitAddSkillError),
    #[error("failed to set stance")]
    Stance(#[from] GetFitSetStanceError),
    #[error("failed to add subsystem")]
    Subsystem(#[from] GetFitAddSubsystemError),
}
