use crate::{
    AutochargeChangeCmd, BoosterAddCmd, BoosterChangeCmd, CharacterChangeCmd, CharacterSetCmd, CharacterUnsetCmd,
    ChargeChangeCmd, CtlCmdResp, CtlCmdResps, DroneAddCmd, DroneAddCmdBr, DroneChangeCmd, DroneChangeCmdBr,
    FighterAddCmd, FighterAddCmdBr, FighterChangeCmd, FighterChangeCmdBr, FitChangeCmd, FitChangeShipCmd,
    FitChangeStanceCmd, FitSetShipCmd, FitSetStanceCmd, FitUnsetShipCmd, FitUnsetStanceCmd, FwEffectAddCmd,
    FwEffectChangeCmd, ImplantAddCmd, ImplantChangeCmd, ItemIdBr, ItemRemoveCmd, ModuleAddCmd, ModuleAddCmdBr,
    ModuleChangeCmd, ModuleChangeCmdBr, RigAddCmd, RigChangeCmd, ServiceAddCmd, ServiceChangeCmd, SkillAddCmd,
    SkillChangeCmd, SubsystemAddCmd, SubsystemChangeCmd,
    ctl::core::{
        AutochargeChangeCmdCtxItem, AutochargeChangeCmdCtxItemBr, BoosterChangeCmdCtxItem, BoosterChangeCmdCtxItemBr,
        ChargeChangeCmdCtxItem, ChargeChangeCmdCtxItemBr, DroneChangeCmdCtxItem, DroneChangeCmdCtxItemBr,
        FighterChangeCmdCtxItem, FighterChangeCmdCtxItemBr, FwEffectChangeCmdCtxItem, FwEffectChangeCmdCtxItemBr,
        ICmdShipChangeICtx, ICmdShipSetICtx, ICmdShipUnsetICtx, ICmdStanceChangeICtx, ICmdStanceSetICtx,
        ICmdStanceUnsetICtx, ImplantChangeCmdCtxItem, ImplantChangeCmdCtxItemBr, ItemRemoveCmdCtxItem,
        ItemRemoveCmdCtxItemBr, ModuleChangeCmdCtxItem, ModuleChangeCmdCtxItemBr, RigChangeCmdCtxItem,
        RigChangeCmdCtxItemBr, ServiceChangeCmdCtxItem, ServiceChangeCmdCtxItemBr, SkillChangeCmdCtxItem,
        SkillChangeCmdCtxItemBr, SubsystemChangeCmdCtxItem, SubsystemChangeCmdCtxItemBr,
    },
    err::{
        BackrefRenderError, DroneAddError, FighterAddError, FitChangeError, FitChangeShipError, FitChangeStanceError,
        FitCharacterChangeError, ItemGetAutochargeChangeError, ItemGetBoosterChangeError, ItemGetChargeChangeError,
        ItemGetDroneChangeError, ItemGetFighterChangeError, ItemGetFwEffectChangeError, ItemGetImplantChangeError,
        ItemGetItemRemoveError, ItemGetModuleChangeError, ItemGetRigChangeError, ItemGetServiceChangeError,
        ItemGetSkillChangeError, ItemGetSubsystemChangeError, ModuleAddError, SkillAddError,
    },
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
pub enum FitCtlCmd {
    // Fit
    ChangeFit(FitChangeCmd),
    // Item
    RemoveItem(ItemRemoveCmdCtxItemBr),
    // Item - autocharge
    ChangeAutocharge(AutochargeChangeCmdCtxItemBr),
    // Item - booster
    AddBooster(BoosterAddCmd),
    ChangeBooster(BoosterChangeCmdCtxItemBr),
    // Item - character
    SetCharacter(CharacterSetCmd),
    ChangeCharacter(CharacterChangeCmd),
    UnsetCharacter(CharacterUnsetCmd),
    // Item - charge
    ChangeCharge(ChargeChangeCmdCtxItemBr),
    // Item - drone
    AddDrone(DroneAddCmdBr),
    ChangeDrone(DroneChangeCmdCtxItemBr),
    // Item - fighter
    AddFighter(FighterAddCmdBr),
    ChangeFighter(FighterChangeCmdCtxItemBr),
    // Item - fit-wide effect
    AddFwEffect(FwEffectAddCmd),
    ChangeFwEffect(FwEffectChangeCmdCtxItemBr),
    // Item - implant
    AddImplant(ImplantAddCmd),
    ChangeImplant(ImplantChangeCmdCtxItemBr),
    // Item - module
    AddModule(ModuleAddCmdBr),
    ChangeModule(ModuleChangeCmdCtxItemBr),
    // Item - rig
    AddRig(RigAddCmd),
    ChangeRig(RigChangeCmdCtxItemBr),
    // Item - service
    AddService(ServiceAddCmd),
    ChangeService(ServiceChangeCmdCtxItemBr),
    // Item - ship
    SetShip(FitSetShipCmd),
    ChangeShip(FitChangeShipCmd),
    UnsetShip(FitUnsetShipCmd),
    // Item - skill
    AddSkill(SkillAddCmd),
    ChangeSkill(SkillChangeCmdCtxItemBr),
    // Item - stance
    SetStance(FitSetStanceCmd),
    ChangeStance(FitChangeStanceCmd),
    UnsetStance(FitUnsetStanceCmd),
    // Item - subsystem
    AddSubsystem(SubsystemAddCmd),
    ChangeSubsystem(SubsystemChangeCmdCtxItemBr),
}

pub(crate) enum FitCtlCmdRendered {
    // Fit
    ChangeFit(FitChangeCmd),
    // Item
    RemoveItem(ItemRemoveCmdCtxItem),
    // Item - autocharge
    ChangeAutocharge(AutochargeChangeCmdCtxItem),
    // Item - booster
    AddBooster(BoosterAddCmd),
    ChangeBooster(BoosterChangeCmdCtxItem),
    // Item - character
    SetCharacter(CharacterSetCmd),
    ChangeCharacter(CharacterChangeCmd),
    UnsetCharacter(CharacterUnsetCmd),
    // Item - charge
    ChangeCharge(ChargeChangeCmdCtxItem),
    // Item - drone
    AddDrone(DroneAddCmd),
    ChangeDrone(DroneChangeCmdCtxItem),
    // Item - fighter
    AddFighter(FighterAddCmd),
    ChangeFighter(FighterChangeCmdCtxItem),
    // Item - fit-wide effect
    AddFwEffect(FwEffectAddCmd),
    ChangeFwEffect(FwEffectChangeCmdCtxItem),
    // Item - implant
    AddImplant(ImplantAddCmd),
    ChangeImplant(ImplantChangeCmdCtxItem),
    // Item - module
    AddModule(ModuleAddCmd),
    ChangeModule(ModuleChangeCmdCtxItem),
    // Item - rig
    AddRig(RigAddCmd),
    ChangeRig(RigChangeCmdCtxItem),
    // Item - service
    AddService(ServiceAddCmd),
    ChangeService(ServiceChangeCmdCtxItem),
    // Item - ship
    SetShip(ICmdShipSetICtx),
    ChangeShip(ICmdShipChangeICtx),
    UnsetShip(ICmdShipUnsetICtx),
    // Item - skill
    AddSkill(SkillAddCmd),
    ChangeSkill(SkillChangeCmdCtxItem),
    // Item - stance
    SetStance(ICmdStanceSetICtx),
    ChangeStance(ICmdStanceChangeICtx),
    UnsetStance(ICmdStanceUnsetICtx),
    // Item - subsystem
    AddSubsystem(SubsystemAddCmd),
    ChangeSubsystem(SubsystemChangeCmdCtxItem),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
// Fit
impl FitChangeCmd {
    pub fn into_fit_ctl(self) -> FitCtlCmd {
        FitCtlCmd::ChangeFit(self)
    }
}
// Item
impl ItemRemoveCmd {
    pub fn into_fit_ctl(self, item_id: impl Into<ItemIdBr>) -> FitCtlCmd {
        FitCtlCmd::RemoveItem(self.into_ctx_item_br(item_id))
    }
}
// Item - autocharge
impl AutochargeChangeCmd {
    pub fn into_fit_ctl(self, item_id: impl Into<ItemIdBr>) -> FitCtlCmd {
        FitCtlCmd::ChangeAutocharge(self.into_ctx_item_br(item_id))
    }
}
// Item - booster
impl BoosterAddCmd {
    pub fn into_fit_ctl(self) -> FitCtlCmd {
        FitCtlCmd::AddBooster(self)
    }
}
impl BoosterChangeCmd {
    pub fn into_fit_ctl(self, item_id: impl Into<ItemIdBr>) -> FitCtlCmd {
        FitCtlCmd::ChangeBooster(self.into_ctx_item_br(item_id))
    }
}
// Item - character
impl CharacterSetCmd {
    pub fn into_fit_ctl(self) -> FitCtlCmd {
        FitCtlCmd::SetCharacter(self)
    }
}
impl CharacterChangeCmd {
    pub fn into_fit_ctl(self) -> FitCtlCmd {
        FitCtlCmd::ChangeCharacter(self)
    }
}
impl CharacterUnsetCmd {
    pub fn into_fit_ctl(self) -> FitCtlCmd {
        FitCtlCmd::UnsetCharacter(self)
    }
}
// Item - charge
impl ChargeChangeCmd {
    pub fn into_fit_ctl(self, item_id: impl Into<ItemIdBr>) -> FitCtlCmd {
        FitCtlCmd::ChangeCharge(self.into_ctx_item_br(item_id))
    }
}
// Item - drone
impl DroneAddCmd {
    pub fn into_fit_ctl(self) -> FitCtlCmd {
        FitCtlCmd::AddDrone(self.into_br())
    }
}
impl DroneAddCmdBr {
    pub fn into_fit_ctl(self) -> FitCtlCmd {
        FitCtlCmd::AddDrone(self)
    }
}
impl DroneChangeCmd {
    pub fn into_fit_ctl(self, item_id: impl Into<ItemIdBr>) -> FitCtlCmd {
        FitCtlCmd::ChangeDrone(self.into_ctx_item_br(item_id))
    }
}
impl DroneChangeCmdBr {
    pub fn into_fit_ctl(self, item_id: impl Into<ItemIdBr>) -> FitCtlCmd {
        FitCtlCmd::ChangeDrone(self.into_ctx_item_br(item_id))
    }
}
// Item - fighter
impl FighterAddCmd {
    pub fn into_fit_ctl(self) -> FitCtlCmd {
        FitCtlCmd::AddFighter(self.into_br())
    }
}
impl FighterAddCmdBr {
    pub fn into_fit_ctl(self) -> FitCtlCmd {
        FitCtlCmd::AddFighter(self)
    }
}
impl FighterChangeCmd {
    pub fn into_fit_ctl(self, item_id: impl Into<ItemIdBr>) -> FitCtlCmd {
        FitCtlCmd::ChangeFighter(self.into_ctx_item_br(item_id))
    }
}
impl FighterChangeCmdBr {
    pub fn into_fit_ctl(self, item_id: impl Into<ItemIdBr>) -> FitCtlCmd {
        FitCtlCmd::ChangeFighter(self.into_ctx_item_br(item_id))
    }
}
// Item - fit-wide effect
impl FwEffectAddCmd {
    pub fn into_fit_ctl(self) -> FitCtlCmd {
        FitCtlCmd::AddFwEffect(self)
    }
}
impl FwEffectChangeCmd {
    pub fn into_fit_ctl(self, item_id: impl Into<ItemIdBr>) -> FitCtlCmd {
        FitCtlCmd::ChangeFwEffect(self.into_ctx_item_br(item_id))
    }
}
// Item - implant
impl ImplantAddCmd {
    pub fn into_fit_ctl(self) -> FitCtlCmd {
        FitCtlCmd::AddImplant(self)
    }
}
impl ImplantChangeCmd {
    pub fn into_fit_ctl(self, item_id: impl Into<ItemIdBr>) -> FitCtlCmd {
        FitCtlCmd::ChangeImplant(self.into_ctx_item_br(item_id))
    }
}
// Item - module
impl ModuleAddCmd {
    pub fn into_fit_ctl(self) -> FitCtlCmd {
        FitCtlCmd::AddModule(self.into_br())
    }
}
impl ModuleAddCmdBr {
    pub fn into_fit_ctl(self) -> FitCtlCmd {
        FitCtlCmd::AddModule(self)
    }
}
impl ModuleChangeCmd {
    pub fn into_fit_ctl(self, item_id: impl Into<ItemIdBr>) -> FitCtlCmd {
        FitCtlCmd::ChangeModule(self.into_ctx_item_br(item_id))
    }
}
impl ModuleChangeCmdBr {
    pub fn into_fit_ctl(self, item_id: impl Into<ItemIdBr>) -> FitCtlCmd {
        FitCtlCmd::ChangeModule(self.into_ctx_item_br(item_id))
    }
}
// Item - rig
impl RigAddCmd {
    pub fn into_fit_ctl(self) -> FitCtlCmd {
        FitCtlCmd::AddRig(self)
    }
}
impl RigChangeCmd {
    pub fn into_fit_ctl(self, item_id: impl Into<ItemIdBr>) -> FitCtlCmd {
        FitCtlCmd::ChangeRig(self.into_ctx_item_br(item_id))
    }
}
// Item - service
impl ServiceAddCmd {
    pub fn into_fit_ctl(self) -> FitCtlCmd {
        FitCtlCmd::AddService(self)
    }
}
impl ServiceChangeCmd {
    pub fn into_fit_ctl(self, item_id: impl Into<ItemIdBr>) -> FitCtlCmd {
        FitCtlCmd::ChangeService(self.into_ctx_item_br(item_id))
    }
}
// Item - skill
impl SkillAddCmd {
    pub fn into_fit_ctl(self) -> FitCtlCmd {
        FitCtlCmd::AddSkill(self)
    }
}
impl SkillChangeCmd {
    pub fn into_fit_ctl(self, item_id: impl Into<ItemIdBr>) -> FitCtlCmd {
        FitCtlCmd::ChangeSkill(self.into_ctx_item_br(item_id))
    }
}
// Item - subsystem
impl SubsystemAddCmd {
    pub fn into_fit_ctl(self) -> FitCtlCmd {
        FitCtlCmd::AddSubsystem(self)
    }
}
impl SubsystemChangeCmd {
    pub fn into_fit_ctl(self, item_id: impl Into<ItemIdBr>) -> FitCtlCmd {
        FitCtlCmd::ChangeSubsystem(self.into_ctx_item_br(item_id))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitCtlCmd {
    pub(crate) fn render(self, resps: &CtlCmdResps) -> Result<FitCtlCmdRendered, BackrefRenderError> {
        Ok(match self {
            // Fit
            Self::ChangeFit(cmd) => FitCtlCmdRendered::ChangeFit(cmd),
            // Item
            Self::RemoveItem(cmd) => FitCtlCmdRendered::RemoveItem(cmd.render(resps)?),
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => FitCtlCmdRendered::ChangeAutocharge(cmd.render(resps)?),
            // Item - booster
            Self::AddBooster(cmd) => FitCtlCmdRendered::AddBooster(cmd),
            Self::ChangeBooster(cmd) => FitCtlCmdRendered::ChangeBooster(cmd.render(resps)?),
            // Item - character
            Self::SetCharacter(cmd) => FitCtlCmdRendered::SetCharacter(cmd),
            Self::ChangeCharacter(cmd) => FitCtlCmdRendered::ChangeCharacter(cmd),
            Self::UnsetCharacter(cmd) => FitCtlCmdRendered::UnsetCharacter(cmd),
            // Item - charge
            Self::ChangeCharge(cmd) => FitCtlCmdRendered::ChangeCharge(cmd.render(resps)?),
            // Item - drone
            Self::AddDrone(cmd) => FitCtlCmdRendered::AddDrone(cmd.render(resps)?),
            Self::ChangeDrone(cmd) => FitCtlCmdRendered::ChangeDrone(cmd.render(resps)?),
            // Item - fighter
            Self::AddFighter(cmd) => FitCtlCmdRendered::AddFighter(cmd.render(resps)?),
            Self::ChangeFighter(cmd) => FitCtlCmdRendered::ChangeFighter(cmd.render(resps)?),
            // Item - fit-wide effect
            Self::AddFwEffect(cmd) => FitCtlCmdRendered::AddFwEffect(cmd),
            Self::ChangeFwEffect(cmd) => FitCtlCmdRendered::ChangeFwEffect(cmd.render(resps)?),
            // Item - implant
            Self::AddImplant(cmd) => FitCtlCmdRendered::AddImplant(cmd),
            Self::ChangeImplant(cmd) => FitCtlCmdRendered::ChangeImplant(cmd.render(resps)?),
            // Item - drone
            Self::AddModule(cmd) => FitCtlCmdRendered::AddModule(cmd.render(resps)?),
            Self::ChangeModule(cmd) => FitCtlCmdRendered::ChangeModule(cmd.render(resps)?),
            // Item - rig
            Self::AddRig(cmd) => FitCtlCmdRendered::AddRig(cmd),
            Self::ChangeRig(cmd) => FitCtlCmdRendered::ChangeRig(cmd.render(resps)?),
            // Item - service
            Self::AddService(cmd) => FitCtlCmdRendered::AddService(cmd),
            Self::ChangeService(cmd) => FitCtlCmdRendered::ChangeService(cmd.render(resps)?),
            // Item - ship
            Self::SetShip(cmd) => FitCtlCmdRendered::SetShip(cmd.inner),
            Self::ChangeShip(cmd) => FitCtlCmdRendered::ChangeShip(cmd.inner),
            Self::UnsetShip(cmd) => FitCtlCmdRendered::UnsetShip(cmd.inner),
            // Item - skill
            Self::AddSkill(cmd) => FitCtlCmdRendered::AddSkill(cmd),
            Self::ChangeSkill(cmd) => FitCtlCmdRendered::ChangeSkill(cmd.render(resps)?),
            // Item - stance
            Self::SetStance(cmd) => FitCtlCmdRendered::SetStance(cmd.inner),
            Self::ChangeStance(cmd) => FitCtlCmdRendered::ChangeStance(cmd.inner),
            Self::UnsetStance(cmd) => FitCtlCmdRendered::UnsetStance(cmd.inner),
            // Item - subsystem
            Self::AddSubsystem(cmd) => FitCtlCmdRendered::AddSubsystem(cmd),
            Self::ChangeSubsystem(cmd) => FitCtlCmdRendered::ChangeSubsystem(cmd.render(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitCtlCmdRendered {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> Result<CtlCmdResp, FitCtlCmdError> {
        match self {
            // Fit
            Self::ChangeFit(cmd) => Ok(cmd.execute(core_fit)?.into()),
            // Item
            Self::RemoveItem(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - booster
            Self::AddBooster(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeBooster(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - character
            Self::SetCharacter(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeCharacter(cmd) => Ok(cmd.execute_via_fit(core_fit)?.into()),
            #[expect(clippy::unit_arg)]
            Self::UnsetCharacter(cmd) => Ok(cmd.execute(core_fit).into()),
            // Item - charge
            Self::ChangeCharge(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - drone
            Self::AddDrone(cmd) => Ok(cmd.execute(core_fit)?.into()),
            Self::ChangeDrone(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - fighter
            Self::AddFighter(cmd) => Ok(cmd.execute(core_fit)?.into()),
            Self::ChangeFighter(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - fit-wide effect
            Self::AddFwEffect(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeFwEffect(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - implant
            Self::AddImplant(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeImplant(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - module
            Self::AddModule(cmd) => Ok(cmd.execute(core_fit)?.into()),
            Self::ChangeModule(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - rig
            Self::AddRig(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeRig(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - service
            Self::AddService(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeService(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - ship
            Self::SetShip(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeShip(cmd) => Ok(cmd.execute_via_fit(core_fit)?.into()),
            #[expect(clippy::unit_arg)]
            Self::UnsetShip(cmd) => Ok(cmd.execute(core_fit).into()),
            // Item - skill
            Self::AddSkill(cmd) => Ok(cmd.execute(core_fit)?.into()),
            Self::ChangeSkill(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
            // Item - stance
            Self::SetStance(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeStance(cmd) => Ok(cmd.execute_via_fit(core_fit)?.into()),
            #[expect(clippy::unit_arg)]
            Self::UnsetStance(cmd) => Ok(cmd.execute(core_fit).into()),
            // Item - subsystem
            Self::AddSubsystem(cmd) => Ok(cmd.execute(core_fit).into()),
            Self::ChangeSubsystem(cmd) => Ok(cmd.execute(core_fit.get_sol_mut())?.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitCtlCmdError {
    // Fit
    #[error("failed to change fleet")]
    FitChange(#[from] FitChangeError),
    // Item
    #[error("failed to remove item")]
    ItemRemove(#[from] ItemGetItemRemoveError),
    // Item - autocharge
    #[error("failed to change autocharge")]
    AutochargeChange(#[from] ItemGetAutochargeChangeError),
    // Item - booster
    #[error("failed to change booster")]
    BoosterChange(#[from] ItemGetBoosterChangeError),
    // Item - character
    #[error("failed to change character")]
    CharacterChange(#[from] FitCharacterChangeError),
    // Item - charge
    #[error("failed to change charge")]
    ChargeChange(#[from] ItemGetChargeChangeError),
    // Item - drone
    #[error("failed to add drone")]
    DroneAdd(#[from] DroneAddError),
    #[error("failed to change drone")]
    DroneChange(#[from] ItemGetDroneChangeError),
    // Item - fighter
    #[error("failed to add fighter")]
    FighterAdd(#[from] FighterAddError),
    #[error("failed to change fighter")]
    FighterChange(#[from] ItemGetFighterChangeError),
    // Item - fit-wide effect
    #[error("failed to change fit-wide effect")]
    FwEffectChange(#[from] ItemGetFwEffectChangeError),
    // Item - implant
    #[error("failed to change implant")]
    ImplantChange(#[from] ItemGetImplantChangeError),
    // Item - module
    #[error("failed to add module")]
    ModuleAdd(#[from] ModuleAddError),
    #[error("failed to change module")]
    ModuleChange(#[from] ItemGetModuleChangeError),
    // Item - rig
    #[error("failed to change rig")]
    RigChange(#[from] ItemGetRigChangeError),
    // Item - service
    #[error("failed to change service")]
    ServiceChange(#[from] ItemGetServiceChangeError),
    // Item - ship
    #[error("failed to change ship")]
    ShipChange(#[from] FitChangeShipError),
    // Item - skill
    #[error("failed to add skill")]
    SkillAdd(#[from] SkillAddError),
    #[error("failed to change skill")]
    SkillChange(#[from] ItemGetSkillChangeError),
    // Item - stance
    #[error("failed to change stance")]
    StanceChange(#[from] FitChangeStanceError),
    // Item - subsystem
    #[error("failed to change subsystem")]
    SubsystemChange(#[from] ItemGetSubsystemChangeError),
}
