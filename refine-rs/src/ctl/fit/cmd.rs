use crate::{
    AutochargeChangeCmd, BoosterAddCmd, BoosterChangeCmd, ChargeChangeCmd, CtlCmdResp, CtlCmdResps, FitAddDroneCmd,
    FitAddFighterCmd, FitAddFwEffectCmd, FitAddModuleCmd, FitAddSkillCmd, FitChangeCharacterCmd, FitChangeCmd,
    FitChangeDroneCmd, FitChangeFighterCmd, FitChangeFwEffectCmd, FitChangeModuleCmd, FitChangeShipCmd,
    FitChangeSkillCmd, FitChangeStanceCmd, FitSetCharacterCmd, FitSetShipCmd, FitSetStanceCmd, FitUnsetCharacterCmd,
    FitUnsetShipCmd, FitUnsetStanceCmd, ImplantAddCmd, ImplantChangeCmd, ItemIdBr, ItemRemoveCmd, RigAddCmd,
    RigChangeCmd, ServiceAddCmd, ServiceChangeCmd, SubsystemAddCmd, SubsystemChangeCmd,
    ctl::core::{
        AutochargeChangeCmdCtxItem, AutochargeChangeCmdCtxItemBr, BoosterChangeCmdCtxItem, BoosterChangeCmdCtxItemBr,
        ChargeChangeCmdCtxItem, ChargeChangeCmdCtxItemBr, ICmdCharacterChangeICtx, ICmdCharacterSetICtx,
        ICmdCharacterUnsetICtx, ICmdDroneAddICtxRIds, ICmdDroneChangeFCtxRIds, ICmdFighterAddICtxRIds,
        ICmdFighterChangeFCtxRIds, ICmdFwEffectAddICtx, ICmdFwEffectChangeFCtxRIds, ICmdModuleAddICtxRIds,
        ICmdModuleChangeFCtxRIds, ICmdShipChangeICtx, ICmdShipSetICtx, ICmdShipUnsetICtx, ICmdSkillAddICtx,
        ICmdSkillChangeFCtxRIds, ICmdStanceChangeICtx, ICmdStanceSetICtx, ICmdStanceUnsetICtx, ImplantChangeCmdCtxItem,
        ImplantChangeCmdCtxItemBr, ItemRemoveCmdCtxItem, ItemRemoveCmdCtxItemBr, RigChangeCmdCtxItem,
        RigChangeCmdCtxItemBr, ServiceChangeCmdCtxItem, ServiceChangeCmdCtxItemBr, SubsystemChangeCmdCtxItem,
        SubsystemChangeCmdCtxItemBr,
    },
    err::{
        BackrefRenderError, FitAddDroneError, FitAddFighterError, FitAddModuleError, FitAddSkillError,
        FitChangeCharacterError, FitChangeError, FitChangeShipError, FitChangeStanceError, GetItemChangeDroneError,
        GetItemChangeFighterError, GetItemChangeFwEffectError, GetItemChangeModuleError, GetItemChangeSkillError,
        ItemGetAutochargeChangeError, ItemGetBoosterChangeError, ItemGetChargeChangeError, ItemGetImplantChangeError,
        ItemGetItemRemoveError, ItemGetRigChangeError, ItemGetServiceChangeError, ItemGetSubsystemChangeError,
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
    SetCharacter(FitSetCharacterCmd),
    ChangeCharacter(FitChangeCharacterCmd),
    UnsetCharacter(FitUnsetCharacterCmd),
    // Item - charge
    ChangeCharge(ChargeChangeCmdCtxItemBr),
    // Item - drone
    AddDrone(FitAddDroneCmd),
    ChangeDrone(FitChangeDroneCmd),
    // Item - fighter
    AddFighter(FitAddFighterCmd),
    ChangeFighter(FitChangeFighterCmd),
    // Item - fit-wide effect
    AddFwEffect(FitAddFwEffectCmd),
    ChangeFwEffect(FitChangeFwEffectCmd),
    // Item - implant
    AddImplant(ImplantAddCmd),
    ChangeImplant(ImplantChangeCmdCtxItemBr),
    // Item - module
    AddModule(FitAddModuleCmd),
    ChangeModule(FitChangeModuleCmd),
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
    AddSkill(FitAddSkillCmd),
    ChangeSkill(FitChangeSkillCmd),
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
    SetCharacter(ICmdCharacterSetICtx),
    ChangeCharacter(ICmdCharacterChangeICtx),
    UnsetCharacter(ICmdCharacterUnsetICtx),
    // Item - charge
    ChangeCharge(ChargeChangeCmdCtxItem),
    // Item - drone
    AddDrone(ICmdDroneAddICtxRIds),
    ChangeDrone(ICmdDroneChangeFCtxRIds),
    // Item - fighter
    AddFighter(ICmdFighterAddICtxRIds),
    ChangeFighter(ICmdFighterChangeFCtxRIds),
    // Item - fit-wide effect
    AddFwEffect(ICmdFwEffectAddICtx),
    ChangeFwEffect(ICmdFwEffectChangeFCtxRIds),
    // Item - implant
    AddImplant(ImplantAddCmd),
    ChangeImplant(ImplantChangeCmdCtxItem),
    // Item - module
    AddModule(ICmdModuleAddICtxRIds),
    ChangeModule(ICmdModuleChangeFCtxRIds),
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
    AddSkill(ICmdSkillAddICtx),
    ChangeSkill(ICmdSkillChangeFCtxRIds),
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
// Item - charge
impl ChargeChangeCmd {
    pub fn into_fit_ctl(self, item_id: impl Into<ItemIdBr>) -> FitCtlCmd {
        FitCtlCmd::ChangeCharge(self.into_ctx_item_br(item_id))
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
            Self::SetCharacter(cmd) => FitCtlCmdRendered::SetCharacter(cmd.inner),
            Self::ChangeCharacter(cmd) => FitCtlCmdRendered::ChangeCharacter(cmd.inner),
            Self::UnsetCharacter(cmd) => FitCtlCmdRendered::UnsetCharacter(cmd.inner),
            // Item - charge
            Self::ChangeCharge(cmd) => FitCtlCmdRendered::ChangeCharge(cmd.render(resps)?),
            // Item - drone
            Self::AddDrone(cmd) => FitCtlCmdRendered::AddDrone(cmd.inner.render(resps)?),
            Self::ChangeDrone(cmd) => FitCtlCmdRendered::ChangeDrone(cmd.inner.render(resps)?),
            // Item - fighter
            Self::AddFighter(cmd) => FitCtlCmdRendered::AddFighter(cmd.inner.render(resps)?),
            Self::ChangeFighter(cmd) => FitCtlCmdRendered::ChangeFighter(cmd.inner.render(resps)?),
            // Item - fit-wide effect
            Self::AddFwEffect(cmd) => FitCtlCmdRendered::AddFwEffect(cmd.inner),
            Self::ChangeFwEffect(cmd) => FitCtlCmdRendered::ChangeFwEffect(cmd.inner.render(resps)?),
            // Item - implant
            Self::AddImplant(cmd) => FitCtlCmdRendered::AddImplant(cmd),
            Self::ChangeImplant(cmd) => FitCtlCmdRendered::ChangeImplant(cmd.render(resps)?),
            // Item - drone
            Self::AddModule(cmd) => FitCtlCmdRendered::AddModule(cmd.inner.render(resps)?),
            Self::ChangeModule(cmd) => FitCtlCmdRendered::ChangeModule(cmd.inner.render(resps)?),
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
            Self::AddSkill(cmd) => FitCtlCmdRendered::AddSkill(cmd.inner),
            Self::ChangeSkill(cmd) => FitCtlCmdRendered::ChangeSkill(cmd.inner.render(resps)?),
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
    CharacterChange(#[from] FitChangeCharacterError),
    // Item - charge
    #[error("failed to change charge")]
    ChargeChange(#[from] ItemGetChargeChangeError),
    // Item - drone
    #[error("failed to add drone")]
    DroneAdd(#[from] FitAddDroneError),
    #[error("failed to change drone")]
    DroneChange(#[from] GetItemChangeDroneError),
    // Item - fighter
    #[error("failed to add fighter")]
    FighterAdd(#[from] FitAddFighterError),
    #[error("failed to change fighter")]
    FighterChange(#[from] GetItemChangeFighterError),
    // Item - fit-wide effect
    #[error("failed to change fit-wide effect")]
    FwEffectChange(#[from] GetItemChangeFwEffectError),
    // Item - implant
    #[error("failed to change implant")]
    ImplantChange(#[from] ItemGetImplantChangeError),
    // Item - module
    #[error("failed to add module")]
    ModuleAdd(#[from] FitAddModuleError),
    #[error("failed to change module")]
    ModuleChange(#[from] GetItemChangeModuleError),
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
    SkillAdd(#[from] FitAddSkillError),
    #[error("failed to change skill")]
    SkillChange(#[from] GetItemChangeSkillError),
    // Item - stance
    #[error("failed to change stance")]
    StanceChange(#[from] FitChangeStanceError),
    // Item - subsystem
    #[error("failed to change subsystem")]
    SubsystemChange(#[from] ItemGetSubsystemChangeError),
}
