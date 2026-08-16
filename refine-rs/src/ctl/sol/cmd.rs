use super::{
    sub_item_character::SolChangeCharacterCmdRIds, sub_item_ship::SolChangeShipCmdRIds,
    sub_item_stance::SolChangeStanceCmdRIds,
};
use crate::{
    AutochargeChangeCmd, BoosterAddCmd, BoosterChangeCmd, ChargeChangeCmd, CtlCmdResp, CtlCmdResps, FitAddCmd,
    FitAddCmdBr, FitChangeCmd, FitChangeCmdBr, FitIdBr, FitRemoveCmd, FleetAddCmdBr, FleetChangeCmd, FleetChangeCmdBr,
    FleetIdBr, FleetRemoveCmd, FwEffectAddCmd, FwEffectChangeCmd, ImplantAddCmd, ImplantChangeCmd, ItemIdBr,
    ItemRemoveCmd, RigAddCmd, RigChangeCmd, ServiceAddCmd, ServiceChangeCmd, SkillAddCmd, SkillChangeCmd,
    SolAddDroneCmd, SolAddFighterCmd, SolAddModuleCmd, SolAddProjEffectCmd, SolAddSwEffectCmd, SolChangeCharacterCmd,
    SolChangeCmd, SolChangeDroneCmd, SolChangeFighterCmd, SolChangeModuleCmd, SolChangeProjEffectCmd, SolChangeShipCmd,
    SolChangeStanceCmd, SolChangeSwEffectCmd, SolSetCharacterCmd, SolSetShipCmd, SolSetStanceCmd, SolUnsetCharacterCmd,
    SolUnsetShipCmd, SolUnsetStanceCmd, SubsystemAddCmd, SubsystemChangeCmd,
    ctl::core::{
        AutochargeChangeCmdCtxItem, AutochargeChangeCmdCtxItemBr, BoosterAddCmdCtxFit, BoosterAddCmdCtxFitBr,
        BoosterChangeCmdCtxItem, BoosterChangeCmdCtxItemBr, ChargeChangeCmdCtxItem, ChargeChangeCmdCtxItemBr,
        FitChangeCmdCtxFit, FitChangeCmdCtxFitBr, FitRemoveCmdCtxFit, FitRemoveCmdCtxFitBr, FleetAddCmd,
        FleetChangeCmdCtxFleet, FleetChangeCmdCtxFleetBr, FleetRemoveCmdCtxFleet, FleetRemoveCmdCtxFleetBr,
        FwEffectAddCmdCtxFit, FwEffectAddCmdCtxFitBr, FwEffectChangeCmdCtxItem, FwEffectChangeCmdCtxItemBr,
        ICmdCharacterSetFCtxRIds, ICmdCharacterUnsetFCtxRIds, ICmdDroneAddFCtxRIds, ICmdDroneChangeFCtxRIds,
        ICmdFighterAddFCtxRIds, ICmdFighterChangeFCtxRIds, ICmdModuleAddFCtxRIds, ICmdModuleChangeFCtxRIds,
        ICmdProjEffectAddFCtxRIds, ICmdProjEffectChangeFCtxRIds, ICmdShipSetFCtxRIds, ICmdShipUnsetFCtxRIds,
        ICmdStanceSetFCtxRIds, ICmdStanceUnsetFCtxRIds, ICmdSwEffectAddFCtx, ICmdSwEffectChangeFCtxRIds,
        ImplantAddCmdCtxFit, ImplantAddCmdCtxFitBr, ImplantChangeCmdCtxItem, ImplantChangeCmdCtxItemBr,
        ItemRemoveCmdCtxItem, ItemRemoveCmdCtxItemBr, RigAddCmdCtxFit, RigAddCmdCtxFitBr, RigChangeCmdCtxItem,
        RigChangeCmdCtxItemBr, ServiceAddCmdCtxFit, ServiceAddCmdCtxFitBr, ServiceChangeCmdCtxItem,
        ServiceChangeCmdCtxItemBr, SkillAddCmdCtxFit, SkillAddCmdCtxFitBr, SkillChangeCmdCtxItem,
        SkillChangeCmdCtxItemBr, SubsystemAddCmdCtxFit, SubsystemAddCmdCtxFitBr, SubsystemChangeCmdCtxItem,
        SubsystemChangeCmdCtxItemBr,
    },
    err::{
        AddProjEffectError, BackrefRenderError, ChangeCharacterError, ChangeShipError, ChangeStanceError, FitAddError,
        FitGetBoosterAddError, FitGetFitChangeError, FitGetFitRemoveError, FitGetFwEffectAddError,
        FitGetImplantAddError, FitGetRigAddError, FitGetServiceAddError, FitGetSkillAddError, FitGetSubsystemAddError,
        FleetAddError, FleetGetFleetChangeError, FleetGetFleetRemoveError, GetFitAddDroneError, GetFitAddFighterError,
        GetFitAddModuleError, GetFitSetCharacterError, GetFitSetShipError, GetFitSetStanceError,
        GetFitUnsetCharacterError, GetFitUnsetShipError, GetFitUnsetStanceError, GetItemChangeDroneError,
        GetItemChangeFighterError, GetItemChangeModuleError, GetItemChangeProjEffectError, GetItemChangeSwEffectError,
        ItemGetAutochargeChangeError, ItemGetBoosterChangeError, ItemGetChargeChangeError, ItemGetFwEffectChangeError,
        ItemGetImplantChangeError, ItemGetItemRemoveError, ItemGetRigChangeError, ItemGetServiceChangeError,
        ItemGetSkillChangeError, ItemGetSubsystemChangeError,
    },
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
pub enum SolCtlCmd {
    // Solar system
    ChangeSol(SolChangeCmd),
    // Fleet
    AddFleet(FleetAddCmdBr),
    ChangeFleet(FleetChangeCmdCtxFleetBr),
    RemoveFleet(FleetRemoveCmdCtxFleetBr),
    // Fit
    AddFit(FitAddCmdBr),
    ChangeFit(FitChangeCmdCtxFitBr),
    RemoveFit(FitRemoveCmdCtxFitBr),
    // Item
    RemoveItem(ItemRemoveCmdCtxItemBr),
    // Item - autocharge
    ChangeAutocharge(AutochargeChangeCmdCtxItemBr),
    // Item - booster
    AddBooster(BoosterAddCmdCtxFitBr),
    ChangeBooster(BoosterChangeCmdCtxItemBr),
    // Item - character
    SetCharacter(SolSetCharacterCmd),
    ChangeCharacter(SolChangeCharacterCmd),
    UnsetCharacter(SolUnsetCharacterCmd),
    // Item - charge
    ChangeCharge(ChargeChangeCmdCtxItemBr),
    // Item - drone
    AddDrone(SolAddDroneCmd),
    ChangeDrone(SolChangeDroneCmd),
    // Item - fighter
    AddFighter(SolAddFighterCmd),
    ChangeFighter(SolChangeFighterCmd),
    // Item - fit-wide effect
    AddFwEffect(FwEffectAddCmdCtxFitBr),
    ChangeFwEffect(FwEffectChangeCmdCtxItemBr),
    // Item - implant
    AddImplant(ImplantAddCmdCtxFitBr),
    ChangeImplant(ImplantChangeCmdCtxItemBr),
    // Item - module
    AddModule(SolAddModuleCmd),
    ChangeModule(SolChangeModuleCmd),
    // Item - projected effect
    AddProjEffect(SolAddProjEffectCmd),
    ChangeProjEffect(SolChangeProjEffectCmd),
    // Item - rig
    AddRig(RigAddCmdCtxFitBr),
    ChangeRig(RigChangeCmdCtxItemBr),
    // Item - service
    AddService(ServiceAddCmdCtxFitBr),
    ChangeService(ServiceChangeCmdCtxItemBr),
    // Item - ship
    SetShip(SolSetShipCmd),
    ChangeShip(SolChangeShipCmd),
    UnsetShip(SolUnsetShipCmd),
    // Item - skill
    AddSkill(SkillAddCmdCtxFitBr),
    ChangeSkill(SkillChangeCmdCtxItemBr),
    // Item - stance
    SetStance(SolSetStanceCmd),
    ChangeStance(SolChangeStanceCmd),
    UnsetStance(SolUnsetStanceCmd),
    // Item - subsystem
    AddSubsystem(SubsystemAddCmdCtxFitBr),
    ChangeSubsystem(SubsystemChangeCmdCtxItemBr),
    // Item - system-wide effect
    AddSwEffect(SolAddSwEffectCmd),
    ChangeSwEffect(SolChangeSwEffectCmd),
}

pub(crate) enum SolCtlCmdRendered {
    // Solar system
    ChangeSol(SolChangeCmd),
    // Fleet
    AddFleet(FleetAddCmd),
    ChangeFleet(FleetChangeCmdCtxFleet),
    RemoveFleet(FleetRemoveCmdCtxFleet),
    // Fit
    AddFit(FitAddCmd),
    ChangeFit(FitChangeCmdCtxFit),
    RemoveFit(FitRemoveCmdCtxFit),
    // Item
    RemoveItem(ItemRemoveCmdCtxItem),
    // Item - autocharge
    ChangeAutocharge(AutochargeChangeCmdCtxItem),
    // Item - booster
    AddBooster(BoosterAddCmdCtxFit),
    ChangeBooster(BoosterChangeCmdCtxItem),
    // Item - character
    SetCharacter(ICmdCharacterSetFCtxRIds),
    ChangeCharacter(SolChangeCharacterCmdRIds),
    UnsetCharacter(ICmdCharacterUnsetFCtxRIds),
    // Item - charge
    ChangeCharge(ChargeChangeCmdCtxItem),
    // Item - drone
    AddDrone(ICmdDroneAddFCtxRIds),
    ChangeDrone(ICmdDroneChangeFCtxRIds),
    // Item - fighter
    AddFighter(ICmdFighterAddFCtxRIds),
    ChangeFighter(ICmdFighterChangeFCtxRIds),
    // Item - fit-wide effect
    AddFwEffect(FwEffectAddCmdCtxFit),
    ChangeFwEffect(FwEffectChangeCmdCtxItem),
    // Item - implant
    AddImplant(ImplantAddCmdCtxFit),
    ChangeImplant(ImplantChangeCmdCtxItem),
    // Item - module
    AddModule(ICmdModuleAddFCtxRIds),
    ChangeModule(ICmdModuleChangeFCtxRIds),
    // Item - projected effect
    AddProjEffect(ICmdProjEffectAddFCtxRIds),
    ChangeProjEffect(ICmdProjEffectChangeFCtxRIds),
    // Item - rig
    AddRig(RigAddCmdCtxFit),
    ChangeRig(RigChangeCmdCtxItem),
    // Item - service
    AddService(ServiceAddCmdCtxFit),
    ChangeService(ServiceChangeCmdCtxItem),
    // Item - ship
    SetShip(ICmdShipSetFCtxRIds),
    ChangeShip(SolChangeShipCmdRIds),
    UnsetShip(ICmdShipUnsetFCtxRIds),
    // Item - skill
    AddSkill(SkillAddCmdCtxFit),
    ChangeSkill(SkillChangeCmdCtxItem),
    // Item - stance
    SetStance(ICmdStanceSetFCtxRIds),
    ChangeStance(SolChangeStanceCmdRIds),
    UnsetStance(ICmdStanceUnsetFCtxRIds),
    // Item - subsystem
    AddSubsystem(SubsystemAddCmdCtxFit),
    ChangeSubsystem(SubsystemChangeCmdCtxItem),
    // Item - system-wide effect
    AddSwEffect(ICmdSwEffectAddFCtx),
    ChangeSwEffect(ICmdSwEffectChangeFCtxRIds),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
// Solar system
impl SolChangeCmd {
    pub fn into_sol_ctl(self) -> SolCtlCmd {
        SolCtlCmd::ChangeSol(self)
    }
}
// Fleet
impl FleetAddCmd {
    pub fn into_sol_ctl(self) -> SolCtlCmd {
        SolCtlCmd::AddFleet(self.into_br())
    }
}
impl FleetAddCmdBr {
    pub fn into_sol_ctl(self) -> SolCtlCmd {
        SolCtlCmd::AddFleet(self)
    }
}
impl FleetChangeCmd {
    pub fn into_sol_ctl(self, fleet_id: impl Into<FleetIdBr>) -> SolCtlCmd {
        SolCtlCmd::ChangeFleet(self.into_ctx_fleet_br(fleet_id))
    }
}
impl FleetChangeCmdBr {
    pub fn into_sol_ctl(self, fleet_id: impl Into<FleetIdBr>) -> SolCtlCmd {
        SolCtlCmd::ChangeFleet(self.into_ctx_fleet_br(fleet_id))
    }
}
impl FleetRemoveCmd {
    pub fn into_sol_ctl(self, fleet_id: impl Into<FleetIdBr>) -> SolCtlCmd {
        SolCtlCmd::RemoveFleet(self.into_ctx_fleet_br(fleet_id))
    }
}
// Fit
impl FitAddCmd {
    pub fn into_sol_ctl(self) -> SolCtlCmd {
        SolCtlCmd::AddFit(self.into_br())
    }
}
impl FitAddCmdBr {
    pub fn into_sol_ctl(self) -> SolCtlCmd {
        SolCtlCmd::AddFit(self)
    }
}
impl FitChangeCmd {
    pub fn into_sol_ctl(self, fit_id: impl Into<FitIdBr>) -> SolCtlCmd {
        SolCtlCmd::ChangeFit(self.into_ctx_fit_br(fit_id))
    }
}
impl FitChangeCmdBr {
    pub fn into_sol_ctl(self, fit_id: impl Into<FitIdBr>) -> SolCtlCmd {
        SolCtlCmd::ChangeFit(self.into_ctx_fit_br(fit_id))
    }
}
impl FitRemoveCmd {
    pub fn into_sol_ctl(self, fit_id: impl Into<FitIdBr>) -> SolCtlCmd {
        SolCtlCmd::RemoveFit(self.into_ctx_fit_br(fit_id))
    }
}
// Item
impl ItemRemoveCmd {
    pub fn into_sol_ctl(self, item_id: impl Into<ItemIdBr>) -> SolCtlCmd {
        SolCtlCmd::RemoveItem(self.into_ctx_item_br(item_id))
    }
}
// Item - autocharge
impl AutochargeChangeCmd {
    pub fn into_sol_ctl(self, item_id: impl Into<ItemIdBr>) -> SolCtlCmd {
        SolCtlCmd::ChangeAutocharge(self.into_ctx_item_br(item_id))
    }
}
// Item - booster
impl BoosterAddCmd {
    pub fn into_sol_ctl(self, fit_id: impl Into<FitIdBr>) -> SolCtlCmd {
        SolCtlCmd::AddBooster(self.into_ctx_fit_br(fit_id))
    }
}
impl BoosterChangeCmd {
    pub fn into_sol_ctl(self, item_id: impl Into<ItemIdBr>) -> SolCtlCmd {
        SolCtlCmd::ChangeBooster(self.into_ctx_item_br(item_id))
    }
}
// Item - charge
impl ChargeChangeCmd {
    pub fn into_sol_ctl(self, item_id: impl Into<ItemIdBr>) -> SolCtlCmd {
        SolCtlCmd::ChangeCharge(self.into_ctx_item_br(item_id))
    }
}
// Item - fit-wide effect
impl FwEffectAddCmd {
    pub fn into_sol_ctl(self, fit_id: impl Into<FitIdBr>) -> SolCtlCmd {
        SolCtlCmd::AddFwEffect(self.into_ctx_fit_br(fit_id))
    }
}
impl FwEffectChangeCmd {
    pub fn into_sol_ctl(self, item_id: impl Into<ItemIdBr>) -> SolCtlCmd {
        SolCtlCmd::ChangeFwEffect(self.into_ctx_item_br(item_id))
    }
}
// Item - implant
impl ImplantAddCmd {
    pub fn into_sol_ctl(self, fit_id: impl Into<FitIdBr>) -> SolCtlCmd {
        SolCtlCmd::AddImplant(self.into_ctx_fit_br(fit_id))
    }
}
impl ImplantChangeCmd {
    pub fn into_sol_ctl(self, item_id: impl Into<ItemIdBr>) -> SolCtlCmd {
        SolCtlCmd::ChangeImplant(self.into_ctx_item_br(item_id))
    }
}
// Item - rig
impl RigAddCmd {
    pub fn into_sol_ctl(self, fit_id: impl Into<FitIdBr>) -> SolCtlCmd {
        SolCtlCmd::AddRig(self.into_ctx_fit_br(fit_id))
    }
}
impl RigChangeCmd {
    pub fn into_sol_ctl(self, item_id: impl Into<ItemIdBr>) -> SolCtlCmd {
        SolCtlCmd::ChangeRig(self.into_ctx_item_br(item_id))
    }
}
// Item - service
impl ServiceAddCmd {
    pub fn into_sol_ctl(self, fit_id: impl Into<FitIdBr>) -> SolCtlCmd {
        SolCtlCmd::AddService(self.into_ctx_fit_br(fit_id))
    }
}
impl ServiceChangeCmd {
    pub fn into_sol_ctl(self, item_id: impl Into<ItemIdBr>) -> SolCtlCmd {
        SolCtlCmd::ChangeService(self.into_ctx_item_br(item_id))
    }
}
// Item - skill
impl SkillAddCmd {
    pub fn into_sol_ctl(self, fit_id: impl Into<FitIdBr>) -> SolCtlCmd {
        SolCtlCmd::AddSkill(self.into_ctx_fit_br(fit_id))
    }
}
impl SkillChangeCmd {
    pub fn into_sol_ctl(self, item_id: impl Into<ItemIdBr>) -> SolCtlCmd {
        SolCtlCmd::ChangeSkill(self.into_ctx_item_br(item_id))
    }
}
// Item - subsystem
impl SubsystemAddCmd {
    pub fn into_sol_ctl(self, fit_id: impl Into<FitIdBr>) -> SolCtlCmd {
        SolCtlCmd::AddSubsystem(self.into_ctx_fit_br(fit_id))
    }
}
impl SubsystemChangeCmd {
    pub fn into_sol_ctl(self, item_id: impl Into<ItemIdBr>) -> SolCtlCmd {
        SolCtlCmd::ChangeSubsystem(self.into_ctx_item_br(item_id))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCtlCmd {
    pub(crate) fn render(self, resps: &CtlCmdResps) -> Result<SolCtlCmdRendered, BackrefRenderError> {
        Ok(match self {
            // Solar system
            Self::ChangeSol(cmd) => SolCtlCmdRendered::ChangeSol(cmd),
            // Fleet
            Self::AddFleet(cmd) => SolCtlCmdRendered::AddFleet(cmd.render(resps)?),
            Self::ChangeFleet(cmd) => SolCtlCmdRendered::ChangeFleet(cmd.render(resps)?),
            Self::RemoveFleet(cmd) => SolCtlCmdRendered::RemoveFleet(cmd.render(resps)?),
            // Fit
            Self::AddFit(cmd) => SolCtlCmdRendered::AddFit(cmd.render(resps)?),
            Self::ChangeFit(cmd) => SolCtlCmdRendered::ChangeFit(cmd.render(resps)?),
            Self::RemoveFit(cmd) => SolCtlCmdRendered::RemoveFit(cmd.render(resps)?),
            // Item
            Self::RemoveItem(cmd) => SolCtlCmdRendered::RemoveItem(cmd.render(resps)?),
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => SolCtlCmdRendered::ChangeAutocharge(cmd.render(resps)?),
            // Item - booster
            Self::AddBooster(cmd) => SolCtlCmdRendered::AddBooster(cmd.render(resps)?),
            Self::ChangeBooster(cmd) => SolCtlCmdRendered::ChangeBooster(cmd.render(resps)?),
            // Item - character
            Self::SetCharacter(cmd) => SolCtlCmdRendered::SetCharacter(cmd.inner.render(resps)?),
            Self::ChangeCharacter(cmd) => SolCtlCmdRendered::ChangeCharacter(cmd.render(resps)?),
            Self::UnsetCharacter(cmd) => SolCtlCmdRendered::UnsetCharacter(cmd.inner.render(resps)?),
            // Item - charge
            Self::ChangeCharge(cmd) => SolCtlCmdRendered::ChangeCharge(cmd.render(resps)?),
            // Item - drone
            Self::AddDrone(cmd) => SolCtlCmdRendered::AddDrone(cmd.inner.render(resps)?),
            Self::ChangeDrone(cmd) => SolCtlCmdRendered::ChangeDrone(cmd.inner.render(resps)?),
            // Item - fighter
            Self::AddFighter(cmd) => SolCtlCmdRendered::AddFighter(cmd.inner.render(resps)?),
            Self::ChangeFighter(cmd) => SolCtlCmdRendered::ChangeFighter(cmd.inner.render(resps)?),
            // Item - fit-wide effect
            Self::AddFwEffect(cmd) => SolCtlCmdRendered::AddFwEffect(cmd.render(resps)?),
            Self::ChangeFwEffect(cmd) => SolCtlCmdRendered::ChangeFwEffect(cmd.render(resps)?),
            // Item - implant
            Self::AddImplant(cmd) => SolCtlCmdRendered::AddImplant(cmd.render(resps)?),
            Self::ChangeImplant(cmd) => SolCtlCmdRendered::ChangeImplant(cmd.render(resps)?),
            // Item - module
            Self::AddModule(cmd) => SolCtlCmdRendered::AddModule(cmd.inner.render(resps)?),
            Self::ChangeModule(cmd) => SolCtlCmdRendered::ChangeModule(cmd.inner.render(resps)?),
            // Item - projected effect
            Self::AddProjEffect(cmd) => SolCtlCmdRendered::AddProjEffect(cmd.inner.render(resps)?),
            Self::ChangeProjEffect(cmd) => SolCtlCmdRendered::ChangeProjEffect(cmd.inner.render(resps)?),
            // Item - rig
            Self::AddRig(cmd) => SolCtlCmdRendered::AddRig(cmd.render(resps)?),
            Self::ChangeRig(cmd) => SolCtlCmdRendered::ChangeRig(cmd.render(resps)?),
            // Item - service
            Self::AddService(cmd) => SolCtlCmdRendered::AddService(cmd.render(resps)?),
            Self::ChangeService(cmd) => SolCtlCmdRendered::ChangeService(cmd.render(resps)?),
            // Item - ship
            Self::SetShip(cmd) => SolCtlCmdRendered::SetShip(cmd.inner.render(resps)?),
            Self::ChangeShip(cmd) => SolCtlCmdRendered::ChangeShip(cmd.render(resps)?),
            Self::UnsetShip(cmd) => SolCtlCmdRendered::UnsetShip(cmd.inner.render(resps)?),
            // Item - skill
            Self::AddSkill(cmd) => SolCtlCmdRendered::AddSkill(cmd.render(resps)?),
            Self::ChangeSkill(cmd) => SolCtlCmdRendered::ChangeSkill(cmd.render(resps)?),
            // Item - stance
            Self::SetStance(cmd) => SolCtlCmdRendered::SetStance(cmd.inner.render(resps)?),
            Self::ChangeStance(cmd) => SolCtlCmdRendered::ChangeStance(cmd.render(resps)?),
            Self::UnsetStance(cmd) => SolCtlCmdRendered::UnsetStance(cmd.inner.render(resps)?),
            // Item - subsystem
            Self::AddSubsystem(cmd) => SolCtlCmdRendered::AddSubsystem(cmd.render(resps)?),
            Self::ChangeSubsystem(cmd) => SolCtlCmdRendered::ChangeSubsystem(cmd.render(resps)?),
            // Item - system-wide effect
            Self::AddSwEffect(cmd) => SolCtlCmdRendered::AddSwEffect(cmd.inner),
            Self::ChangeSwEffect(cmd) => SolCtlCmdRendered::ChangeSwEffect(cmd.inner.render(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCtlCmdRendered {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<CtlCmdResp, ChangeSolEnumError> {
        match self {
            // Solar system
            #[expect(clippy::unit_arg)]
            Self::ChangeSol(cmd) => Ok(cmd.execute(core_sol).into()),
            // Fleet
            Self::AddFleet(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeFleet(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::RemoveFleet(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Fit
            Self::AddFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::RemoveFit(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item
            Self::RemoveItem(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - autocharge
            Self::ChangeAutocharge(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - booster
            Self::AddBooster(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeBooster(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - character
            Self::SetCharacter(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeCharacter(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::UnsetCharacter(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - charge
            Self::ChangeCharge(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - drone
            Self::AddDrone(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeDrone(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - fighter
            Self::AddFighter(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeFighter(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - fit-wide effect
            Self::AddFwEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeFwEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - implant
            Self::AddImplant(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeImplant(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - module
            Self::AddModule(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeModule(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - projected effect
            Self::AddProjEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeProjEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - rig
            Self::AddRig(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeRig(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - service
            Self::AddService(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeService(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - ship
            Self::SetShip(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeShip(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::UnsetShip(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - skill
            Self::AddSkill(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeSkill(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - stance
            Self::SetStance(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeStance(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::UnsetStance(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - subsystem
            Self::AddSubsystem(cmd) => Ok(cmd.execute(core_sol)?.into()),
            Self::ChangeSubsystem(cmd) => Ok(cmd.execute(core_sol)?.into()),
            // Item - system-wide effect
            Self::AddSwEffect(cmd) => Ok(cmd.execute(core_sol).into()),
            Self::ChangeSwEffect(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeSolEnumError {
    // Fleet
    #[error("failed to add fleet")]
    FleetAdd(#[from] FleetAddError),
    #[error("failed to change fleet")]
    FleetChange(#[from] FleetGetFleetChangeError),
    #[error("failed to remove fleet")]
    FleetRemove(#[from] FleetGetFleetRemoveError),
    // Fit
    #[error("failed to add fit")]
    FitAdd(#[from] FitAddError),
    #[error("failed to change fit")]
    FitChange(#[from] FitGetFitChangeError),
    #[error("failed to remove fit")]
    FitRemove(#[from] FitGetFitRemoveError),
    // Item
    #[error("failed to remove item")]
    ItemRemove(#[from] ItemGetItemRemoveError),
    // Item - autocharge
    #[error("failed to change autocharge")]
    AutochargeChange(#[from] ItemGetAutochargeChangeError),
    // Item - booster
    #[error("failed to add booster")]
    BoosterAdd(#[from] FitGetBoosterAddError),
    #[error("failed to change booster")]
    BoosterChange(#[from] ItemGetBoosterChangeError),
    // Item - character
    #[error("failed to set character")]
    CharacterSet(#[from] GetFitSetCharacterError),
    #[error("failed to change character")]
    CharacterChange(#[from] ChangeCharacterError),
    #[error("failed to unset character")]
    CharacterUnset(#[from] GetFitUnsetCharacterError),
    // Item - charge
    #[error("failed to change charge")]
    ChargeChange(#[from] ItemGetChargeChangeError),
    // Item - drone
    #[error("failed to add drone")]
    DroneAdd(#[from] GetFitAddDroneError),
    #[error("failed to change drone")]
    DroneChange(#[from] GetItemChangeDroneError),
    // Item - fighter
    #[error("failed to add fighter")]
    FighterAdd(#[from] GetFitAddFighterError),
    #[error("failed to change fighter")]
    FighterChange(#[from] GetItemChangeFighterError),
    // Item - fit-wide effect
    #[error("failed to add fit-wide effect")]
    FwEffectAdd(#[from] FitGetFwEffectAddError),
    #[error("failed to change fit-wide effect")]
    FwEffectChange(#[from] ItemGetFwEffectChangeError),
    // Item - implant
    #[error("failed to add implant")]
    ImplantAdd(#[from] FitGetImplantAddError),
    #[error("failed to change implant")]
    ImplantChange(#[from] ItemGetImplantChangeError),
    // Item - module
    #[error("failed to add module")]
    ModuleAdd(#[from] GetFitAddModuleError),
    #[error("failed to change module")]
    ModuleChange(#[from] GetItemChangeModuleError),
    // Item - projected effect
    #[error("failed to add projected effect")]
    ProjEffectAdd(#[from] AddProjEffectError),
    #[error("failed to change projected effect")]
    ProjEffectChange(#[from] GetItemChangeProjEffectError),
    // Item - rig
    #[error("failed to add rig")]
    RigAdd(#[from] FitGetRigAddError),
    #[error("failed to change rig")]
    RigChange(#[from] ItemGetRigChangeError),
    // Item - service
    #[error("failed to add service")]
    ServiceAdd(#[from] FitGetServiceAddError),
    #[error("failed to change service")]
    ServiceChange(#[from] ItemGetServiceChangeError),
    // Item - ship
    #[error("failed to set ship")]
    ShipSet(#[from] GetFitSetShipError),
    #[error("failed to change ship")]
    ShipChange(#[from] ChangeShipError),
    #[error("failed to unset ship")]
    ShipUnset(#[from] GetFitUnsetShipError),
    // Item - skill
    #[error("failed to add skill")]
    SkillAdd(#[from] FitGetSkillAddError),
    #[error("failed to change skill")]
    SkillChange(#[from] ItemGetSkillChangeError),
    // Item - stance
    #[error("failed to set stance")]
    StanceSet(#[from] GetFitSetStanceError),
    #[error("failed to change stance")]
    StanceChange(#[from] ChangeStanceError),
    #[error("failed to unset stance")]
    StanceUnset(#[from] GetFitUnsetStanceError),
    // Item - subsystem
    #[error("failed to add subsystem")]
    SubsystemAdd(#[from] FitGetSubsystemAddError),
    #[error("failed to change subsystem")]
    SubsystemChange(#[from] ItemGetSubsystemChangeError),
    // Item - system-wide effect
    #[error("failed to change system-wide effect")]
    SwEffectChange(#[from] GetItemChangeSwEffectError),
}
