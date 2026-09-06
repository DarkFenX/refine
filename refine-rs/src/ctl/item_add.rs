use crate::{
    AddedItemIdsResp, BoosterAddCmd, CharacterSetCmd, DroneAddCmd, FighterAddCmd, FitId, FwEffectAddCmd, ImplantAddCmd,
    ItemAutodetectAddCmd, ModuleAddCmd, ProjEffectAddCmd, RigAddCmd, ServiceAddCmd, ShipSetCmd, SkillAddCmd,
    StanceSetCmd, SubsystemAddCmd, SwEffectAddCmd,
    ctl::core::{
        BoosterAddCmdCtxFit, CharacterSetCmdCtxFit, DroneAddCmdCtxFit, FighterAddCmdCtxFit, FwEffectAddCmdCtxFit,
        ImplantAddCmdCtxFit, ItemAutodetectAddCmdCtxFit, ModuleAddCmdCtxFit, RigAddCmdCtxFit, ServiceAddCmdCtxFit,
        ShipSetCmdCtxFit, SkillAddCmdCtxFit, StanceSetCmdCtxFit, SubsystemAddCmdCtxFit,
    },
    err::{
        FitGetBoosterAddError, FitGetCharacterSetError, FitGetDroneAddError, FitGetFighterAddError,
        FitGetFwEffectAddError, FitGetImplantAddError, FitGetItemAutodetectAddError, FitGetModuleAddError,
        FitGetRigAddError, FitGetServiceAddError, FitGetShipSetError, FitGetSkillAddError, FitGetStanceSetError,
        FitGetSubsystemAddError, ProjEffectAddError,
    },
    shared::CmdResidue,
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
#[derive(Clone)]
pub enum ItemAddEnumCmd {
    Autodetect(ItemAutodetectAddCmdCtxFit),
    Booster(BoosterAddCmdCtxFit),
    Character(CharacterSetCmdCtxFit),
    Drone(DroneAddCmdCtxFit),
    Fighter(FighterAddCmdCtxFit),
    FwEffect(FwEffectAddCmdCtxFit),
    Implant(ImplantAddCmdCtxFit),
    Module(ModuleAddCmdCtxFit),
    ProjEffect(ProjEffectAddCmd),
    Rig(RigAddCmdCtxFit),
    Service(ServiceAddCmdCtxFit),
    Ship(ShipSetCmdCtxFit),
    Skill(SkillAddCmdCtxFit),
    Stance(StanceSetCmdCtxFit),
    Subsystem(SubsystemAddCmdCtxFit),
    SwEffect(SwEffectAddCmd),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemAutodetectAddCmd {
    pub fn into_item_ctl(self, fit_id: FitId) -> ItemAddEnumCmd {
        ItemAddEnumCmd::Autodetect(self.into_ctx_fit(fit_id))
    }
}
impl BoosterAddCmd {
    pub fn into_item_ctl(self, fit_id: FitId) -> ItemAddEnumCmd {
        ItemAddEnumCmd::Booster(self.into_ctx_fit(fit_id))
    }
}
impl CharacterSetCmd {
    pub fn into_item_ctl(self, fit_id: FitId) -> ItemAddEnumCmd {
        ItemAddEnumCmd::Character(self.into_ctx_fit(fit_id))
    }
}
impl DroneAddCmd {
    pub fn into_item_ctl(self, fit_id: FitId) -> ItemAddEnumCmd {
        ItemAddEnumCmd::Drone(self.into_ctx_fit(fit_id))
    }
}
impl FighterAddCmd {
    pub fn into_item_ctl(self, fit_id: FitId) -> ItemAddEnumCmd {
        ItemAddEnumCmd::Fighter(self.into_ctx_fit(fit_id))
    }
}
impl FwEffectAddCmd {
    pub fn into_item_ctl(self, fit_id: FitId) -> ItemAddEnumCmd {
        ItemAddEnumCmd::FwEffect(self.into_ctx_fit(fit_id))
    }
}
impl ImplantAddCmd {
    pub fn into_item_ctl(self, fit_id: FitId) -> ItemAddEnumCmd {
        ItemAddEnumCmd::Implant(self.into_ctx_fit(fit_id))
    }
}
impl ModuleAddCmd {
    pub fn into_item_ctl(self, fit_id: FitId) -> ItemAddEnumCmd {
        ItemAddEnumCmd::Module(self.into_ctx_fit(fit_id))
    }
}
impl ProjEffectAddCmd {
    pub fn into_item_ctl(self) -> ItemAddEnumCmd {
        ItemAddEnumCmd::ProjEffect(self)
    }
}
impl RigAddCmd {
    pub fn into_item_ctl(self, fit_id: FitId) -> ItemAddEnumCmd {
        ItemAddEnumCmd::Rig(self.into_ctx_fit(fit_id))
    }
}
impl ServiceAddCmd {
    pub fn into_item_ctl(self, fit_id: FitId) -> ItemAddEnumCmd {
        ItemAddEnumCmd::Service(self.into_ctx_fit(fit_id))
    }
}
impl ShipSetCmd {
    pub fn into_item_ctl(self, fit_id: FitId) -> ItemAddEnumCmd {
        ItemAddEnumCmd::Ship(self.into_ctx_fit(fit_id))
    }
}
impl SkillAddCmd {
    pub fn into_item_ctl(self, fit_id: FitId) -> ItemAddEnumCmd {
        ItemAddEnumCmd::Skill(self.into_ctx_fit(fit_id))
    }
}
impl StanceSetCmd {
    pub fn into_item_ctl(self, fit_id: FitId) -> ItemAddEnumCmd {
        ItemAddEnumCmd::Stance(self.into_ctx_fit(fit_id))
    }
}
impl SubsystemAddCmd {
    pub fn into_item_ctl(self, fit_id: FitId) -> ItemAddEnumCmd {
        ItemAddEnumCmd::Subsystem(self.into_ctx_fit(fit_id))
    }
}
impl SwEffectAddCmd {
    pub fn into_item_ctl(self) -> ItemAddEnumCmd {
        ItemAddEnumCmd::SwEffect(self)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemAddEnumCmd {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        match self {
            Self::Autodetect(cmd) => cmd.exec_residue(),
            Self::Booster(cmd) => cmd.exec_residue(),
            Self::Character(cmd) => cmd.exec_residue(),
            Self::Drone(cmd) => cmd.exec_residue(),
            Self::Fighter(cmd) => cmd.exec_residue(),
            Self::FwEffect(cmd) => cmd.exec_residue(),
            Self::Implant(cmd) => cmd.exec_residue(),
            Self::Module(cmd) => cmd.exec_residue(),
            Self::ProjEffect(cmd) => cmd.exec_residue(),
            Self::Rig(cmd) => cmd.exec_residue(),
            Self::Service(cmd) => cmd.exec_residue(),
            Self::Ship(cmd) => cmd.exec_residue(),
            Self::Skill(cmd) => cmd.exec_residue(),
            Self::Stance(cmd) => cmd.exec_residue(),
            Self::Subsystem(cmd) => cmd.exec_residue(),
            Self::SwEffect(cmd) => cmd.exec_residue(),
        }
    }
}

impl ItemAddEnumCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<AddedItemIdsResp, ItemAddEnumError> {
        Ok(match self {
            Self::Autodetect(cmd) => cmd.execute(core_sol)?,
            Self::Booster(cmd) => cmd.execute(core_sol)?,
            Self::Character(cmd) => cmd.execute(core_sol)?,
            Self::Drone(cmd) => cmd.execute(core_sol)?,
            Self::Fighter(cmd) => cmd.execute(core_sol)?,
            Self::FwEffect(cmd) => cmd.execute(core_sol)?,
            Self::Implant(cmd) => cmd.execute(core_sol)?,
            Self::Module(cmd) => cmd.execute(core_sol)?,
            Self::ProjEffect(cmd) => cmd.execute(core_sol)?,
            Self::Rig(cmd) => cmd.execute(core_sol)?,
            Self::Service(cmd) => cmd.execute(core_sol)?,
            Self::Ship(cmd) => cmd.execute(core_sol)?,
            Self::Skill(cmd) => cmd.execute(core_sol)?,
            Self::Stance(cmd) => cmd.execute(core_sol)?,
            Self::Subsystem(cmd) => cmd.execute(core_sol)?,
            Self::SwEffect(cmd) => cmd.execute(core_sol),
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemAddEnumError {
    #[error("failed to add autodetected item")]
    Autodetect(#[from] FitGetItemAutodetectAddError),
    #[error("failed to add booster")]
    Booster(#[from] FitGetBoosterAddError),
    #[error("failed to set character")]
    Character(#[from] FitGetCharacterSetError),
    #[error("failed to add drone")]
    Drone(#[from] FitGetDroneAddError),
    #[error("failed to add fighter")]
    Fighter(#[from] FitGetFighterAddError),
    #[error("failed to add fit-wide effect")]
    FwEffect(#[from] FitGetFwEffectAddError),
    #[error("failed to add implant")]
    Implant(#[from] FitGetImplantAddError),
    #[error("failed to add module")]
    Module(#[from] FitGetModuleAddError),
    #[error("failed to add projected effect")]
    ProjEffect(#[from] ProjEffectAddError),
    #[error("failed to add rig")]
    Rig(#[from] FitGetRigAddError),
    #[error("failed to add service")]
    Service(#[from] FitGetServiceAddError),
    #[error("failed to set ship")]
    Ship(#[from] FitGetShipSetError),
    #[error("failed to add skill")]
    Skill(#[from] FitGetSkillAddError),
    #[error("failed to set stance")]
    Stance(#[from] FitGetStanceSetError),
    #[error("failed to add subsystem")]
    Subsystem(#[from] FitGetSubsystemAddError),
}
