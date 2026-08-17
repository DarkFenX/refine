use crate::{
    AutochargeChangeCmd, BoosterAddCmd, BoosterChangeCmd, CharacterChangeCmd, CharacterSetCmd, CharacterUnsetCmd,
    ChargeChangeCmd, CtlCmdResp, CtlCmdResps, DroneAddCmd, DroneAddCmdBr, DroneChangeCmd, DroneChangeCmdBr,
    FighterAddCmd, FighterAddCmdBr, FighterChangeCmd, FighterChangeCmdBr, FitAddCmd, FitAddCmdBr, FitChangeCmd,
    FitChangeCmdBr, FitIdBr, FitRemoveCmd, FleetAddCmdBr, FleetChangeCmd, FleetChangeCmdBr, FleetIdBr, FleetRemoveCmd,
    FwEffectAddCmd, FwEffectChangeCmd, ImplantAddCmd, ImplantChangeCmd, ItemIdBr, ItemRemoveCmd, ModuleAddCmd,
    ModuleAddCmdBr, ModuleChangeCmd, ModuleChangeCmdBr, ProjEffectAddCmd, ProjEffectAddCmdBr, ProjEffectChangeCmd,
    ProjEffectChangeCmdBr, RigAddCmd, RigChangeCmd, ServiceAddCmd, ServiceChangeCmd, ShipChangeCmd, ShipSetCmd,
    ShipUnsetCmd, SkillAddCmd, SkillChangeCmd, SolChangeCmd, StanceChangeCmd, StanceSetCmd, StanceUnsetCmd,
    SubsystemAddCmd, SubsystemChangeCmd, SwEffectAddCmd, SwEffectChangeCmd,
    ctl::core::{
        AutochargeChangeCmdCtxItem, AutochargeChangeCmdCtxItemBr, BoosterAddCmdCtxFit, BoosterAddCmdCtxFitBr,
        BoosterChangeCmdCtxItem, BoosterChangeCmdCtxItemBr, CharacterChangeCmdCtxAny, CharacterChangeCmdCtxAnyBr,
        CharacterSetCmdCtxFit, CharacterSetCmdCtxFitBr, CharacterUnsetCmdCtxFit, CharacterUnsetCmdCtxFitBr,
        ChargeChangeCmdCtxItem, ChargeChangeCmdCtxItemBr, DroneAddCmdCtxFit, DroneAddCmdCtxFitBr,
        DroneChangeCmdCtxItem, DroneChangeCmdCtxItemBr, FighterAddCmdCtxFit, FighterAddCmdCtxFitBr,
        FighterChangeCmdCtxItem, FighterChangeCmdCtxItemBr, FitChangeCmdCtxFit, FitChangeCmdCtxFitBr,
        FitRemoveCmdCtxFit, FitRemoveCmdCtxFitBr, FleetAddCmd, FleetChangeCmdCtxFleet, FleetChangeCmdCtxFleetBr,
        FleetRemoveCmdCtxFleet, FleetRemoveCmdCtxFleetBr, FwEffectAddCmdCtxFit, FwEffectAddCmdCtxFitBr,
        FwEffectChangeCmdCtxItem, FwEffectChangeCmdCtxItemBr, ImplantAddCmdCtxFit, ImplantAddCmdCtxFitBr,
        ImplantChangeCmdCtxItem, ImplantChangeCmdCtxItemBr, ItemRemoveCmdCtxItem, ItemRemoveCmdCtxItemBr,
        ModuleAddCmdCtxFit, ModuleAddCmdCtxFitBr, ModuleChangeCmdCtxItem, ModuleChangeCmdCtxItemBr,
        ProjEffectChangeCmdCtxItem, ProjEffectChangeCmdCtxItemBr, RigAddCmdCtxFit, RigAddCmdCtxFitBr,
        RigChangeCmdCtxItem, RigChangeCmdCtxItemBr, ServiceAddCmdCtxFit, ServiceAddCmdCtxFitBr,
        ServiceChangeCmdCtxItem, ServiceChangeCmdCtxItemBr, ShipChangeCmdCtxAny, ShipChangeCmdCtxAnyBr,
        ShipSetCmdCtxFit, ShipSetCmdCtxFitBr, ShipUnsetCmdCtxFit, ShipUnsetCmdCtxFitBr, SkillAddCmdCtxFit,
        SkillAddCmdCtxFitBr, SkillChangeCmdCtxItem, SkillChangeCmdCtxItemBr, StanceChangeCmdCtxAny,
        StanceChangeCmdCtxAnyBr, StanceSetCmdCtxFit, StanceSetCmdCtxFitBr, StanceUnsetCmdCtxFit,
        StanceUnsetCmdCtxFitBr, SubsystemAddCmdCtxFit, SubsystemAddCmdCtxFitBr, SubsystemChangeCmdCtxItem,
        SubsystemChangeCmdCtxItemBr, SwEffectChangeCmdCtxItem, SwEffectChangeCmdCtxItemBr,
    },
    err::{
        BackrefRenderError, CharacterChangeError, FitAddError, FitGetBoosterAddError, FitGetCharacterSetError,
        FitGetCharacterUnsetError, FitGetDroneAddError, FitGetFighterAddError, FitGetFitChangeError,
        FitGetFitRemoveError, FitGetFwEffectAddError, FitGetImplantAddError, FitGetModuleAddError, FitGetRigAddError,
        FitGetServiceAddError, FitGetShipSetError, FitGetShipUnsetError, FitGetSkillAddError, FitGetStanceSetError,
        FitGetStanceUnsetError, FitGetSubsystemAddError, FleetAddError, FleetGetFleetChangeError,
        FleetGetFleetRemoveError, ItemGetAutochargeChangeError, ItemGetBoosterChangeError, ItemGetChargeChangeError,
        ItemGetDroneChangeError, ItemGetFighterChangeError, ItemGetFwEffectChangeError, ItemGetImplantChangeError,
        ItemGetItemRemoveError, ItemGetModuleChangeError, ItemGetProjEffectChangeError, ItemGetRigChangeError,
        ItemGetServiceChangeError, ItemGetSkillChangeError, ItemGetSubsystemChangeError, ItemGetSwEffectChangeError,
        ProjEffectAddError, ShipChangeError, StanceChangeError,
    },
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
pub enum SolChangeEnumCmd {
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
    SetCharacter(CharacterSetCmdCtxFitBr),
    ChangeCharacter(CharacterChangeCmdCtxAnyBr),
    UnsetCharacter(CharacterUnsetCmdCtxFitBr),
    // Item - charge
    ChangeCharge(ChargeChangeCmdCtxItemBr),
    // Item - drone
    AddDrone(DroneAddCmdCtxFitBr),
    ChangeDrone(DroneChangeCmdCtxItemBr),
    // Item - fighter
    AddFighter(FighterAddCmdCtxFitBr),
    ChangeFighter(FighterChangeCmdCtxItemBr),
    // Item - fit-wide effect
    AddFwEffect(FwEffectAddCmdCtxFitBr),
    ChangeFwEffect(FwEffectChangeCmdCtxItemBr),
    // Item - implant
    AddImplant(ImplantAddCmdCtxFitBr),
    ChangeImplant(ImplantChangeCmdCtxItemBr),
    // Item - module
    AddModule(ModuleAddCmdCtxFitBr),
    ChangeModule(ModuleChangeCmdCtxItemBr),
    // Item - projected effect
    AddProjEffect(ProjEffectAddCmdBr),
    ChangeProjEffect(ProjEffectChangeCmdCtxItemBr),
    // Item - rig
    AddRig(RigAddCmdCtxFitBr),
    ChangeRig(RigChangeCmdCtxItemBr),
    // Item - service
    AddService(ServiceAddCmdCtxFitBr),
    ChangeService(ServiceChangeCmdCtxItemBr),
    // Item - ship
    SetShip(ShipSetCmdCtxFitBr),
    ChangeShip(ShipChangeCmdCtxAnyBr),
    UnsetShip(ShipUnsetCmdCtxFitBr),
    // Item - skill
    AddSkill(SkillAddCmdCtxFitBr),
    ChangeSkill(SkillChangeCmdCtxItemBr),
    // Item - stance
    SetStance(StanceSetCmdCtxFitBr),
    ChangeStance(StanceChangeCmdCtxAnyBr),
    UnsetStance(StanceUnsetCmdCtxFitBr),
    // Item - subsystem
    AddSubsystem(SubsystemAddCmdCtxFitBr),
    ChangeSubsystem(SubsystemChangeCmdCtxItemBr),
    // Item - system-wide effect
    AddSwEffect(SwEffectAddCmd),
    ChangeSwEffect(SwEffectChangeCmdCtxItemBr),
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
    SetCharacter(CharacterSetCmdCtxFit),
    ChangeCharacter(CharacterChangeCmdCtxAny),
    UnsetCharacter(CharacterUnsetCmdCtxFit),
    // Item - charge
    ChangeCharge(ChargeChangeCmdCtxItem),
    // Item - drone
    AddDrone(DroneAddCmdCtxFit),
    ChangeDrone(DroneChangeCmdCtxItem),
    // Item - fighter
    AddFighter(FighterAddCmdCtxFit),
    ChangeFighter(FighterChangeCmdCtxItem),
    // Item - fit-wide effect
    AddFwEffect(FwEffectAddCmdCtxFit),
    ChangeFwEffect(FwEffectChangeCmdCtxItem),
    // Item - implant
    AddImplant(ImplantAddCmdCtxFit),
    ChangeImplant(ImplantChangeCmdCtxItem),
    // Item - module
    AddModule(ModuleAddCmdCtxFit),
    ChangeModule(ModuleChangeCmdCtxItem),
    // Item - projected effect
    AddProjEffect(ProjEffectAddCmd),
    ChangeProjEffect(ProjEffectChangeCmdCtxItem),
    // Item - rig
    AddRig(RigAddCmdCtxFit),
    ChangeRig(RigChangeCmdCtxItem),
    // Item - service
    AddService(ServiceAddCmdCtxFit),
    ChangeService(ServiceChangeCmdCtxItem),
    // Item - ship
    SetShip(ShipSetCmdCtxFit),
    ChangeShip(ShipChangeCmdCtxAny),
    UnsetShip(ShipUnsetCmdCtxFit),
    // Item - skill
    AddSkill(SkillAddCmdCtxFit),
    ChangeSkill(SkillChangeCmdCtxItem),
    // Item - stance
    SetStance(StanceSetCmdCtxFit),
    ChangeStance(StanceChangeCmdCtxAny),
    UnsetStance(StanceUnsetCmdCtxFit),
    // Item - subsystem
    AddSubsystem(SubsystemAddCmdCtxFit),
    ChangeSubsystem(SubsystemChangeCmdCtxItem),
    // Item - system-wide effect
    AddSwEffect(SwEffectAddCmd),
    ChangeSwEffect(SwEffectChangeCmdCtxItem),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
// Solar system
impl SolChangeCmd {
    pub fn into_sol(self) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeSol(self)
    }
}
// Fleet
impl FleetAddCmd {
    pub fn into_sol(self) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddFleet(self.into_br())
    }
}
impl FleetAddCmdBr {
    pub fn into_sol(self) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddFleet(self)
    }
}
impl FleetChangeCmd {
    pub fn into_sol(self, fleet_id: impl Into<FleetIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeFleet(self.into_ctx_fleet_br(fleet_id))
    }
}
impl FleetChangeCmdBr {
    pub fn into_sol(self, fleet_id: impl Into<FleetIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeFleet(self.into_ctx_fleet_br(fleet_id))
    }
}
impl FleetRemoveCmd {
    pub fn into_sol(self, fleet_id: impl Into<FleetIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::RemoveFleet(self.into_ctx_fleet_br(fleet_id))
    }
}
// Fit
impl FitAddCmd {
    pub fn into_sol(self) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddFit(self.into_br())
    }
}
impl FitAddCmdBr {
    pub fn into_sol(self) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddFit(self)
    }
}
impl FitChangeCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeFit(self.into_ctx_fit_br(fit_id))
    }
}
impl FitChangeCmdBr {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeFit(self.into_ctx_fit_br(fit_id))
    }
}
impl FitRemoveCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::RemoveFit(self.into_ctx_fit_br(fit_id))
    }
}
// Item
impl ItemRemoveCmd {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::RemoveItem(self.into_ctx_item_br(item_id))
    }
}
// Item - autocharge
impl AutochargeChangeCmd {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeAutocharge(self.into_ctx_item_br(item_id))
    }
}
// Item - booster
impl BoosterAddCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddBooster(self.into_ctx_fit_br(fit_id))
    }
}
impl BoosterChangeCmd {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeBooster(self.into_ctx_item_br(item_id))
    }
}
// Item - character
impl CharacterSetCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::SetCharacter(self.into_ctx_fit_br(fit_id))
    }
}
impl CharacterChangeCmd {
    pub fn into_sol_via_fit(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeCharacter(self.into_ctx_br_via_fit(fit_id))
    }
    pub fn into_sol_via_item(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeCharacter(self.into_ctx_br_via_item(item_id))
    }
}
impl CharacterUnsetCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::UnsetCharacter(self.into_ctx_fit_br(fit_id))
    }
}
// Item - charge
impl ChargeChangeCmd {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeCharge(self.into_ctx_item_br(item_id))
    }
}
// Item - drone
impl DroneAddCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddDrone(self.into_ctx_fit_br(fit_id))
    }
}
impl DroneAddCmdBr {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddDrone(self.into_ctx_fit_br(fit_id))
    }
}
impl DroneChangeCmd {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeDrone(self.into_ctx_item_br(item_id))
    }
}
impl DroneChangeCmdBr {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeDrone(self.into_ctx_item_br(item_id))
    }
}
// Item - fighter
impl FighterAddCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddFighter(self.into_ctx_fit_br(fit_id))
    }
}
impl FighterAddCmdBr {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddFighter(self.into_ctx_fit_br(fit_id))
    }
}
impl FighterChangeCmd {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeFighter(self.into_ctx_item_br(item_id))
    }
}
impl FighterChangeCmdBr {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeFighter(self.into_ctx_item_br(item_id))
    }
}
// Item - fit-wide effect
impl FwEffectAddCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddFwEffect(self.into_ctx_fit_br(fit_id))
    }
}
impl FwEffectChangeCmd {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeFwEffect(self.into_ctx_item_br(item_id))
    }
}
// Item - implant
impl ImplantAddCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddImplant(self.into_ctx_fit_br(fit_id))
    }
}
impl ImplantChangeCmd {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeImplant(self.into_ctx_item_br(item_id))
    }
}
// Item - module
impl ModuleAddCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddModule(self.into_ctx_fit_br(fit_id))
    }
}
impl ModuleAddCmdBr {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddModule(self.into_ctx_fit_br(fit_id))
    }
}
impl ModuleChangeCmd {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeModule(self.into_ctx_item_br(item_id))
    }
}
impl ModuleChangeCmdBr {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeModule(self.into_ctx_item_br(item_id))
    }
}
// Item - projected effect
impl ProjEffectAddCmd {
    pub fn into_sol(self) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddProjEffect(self.into_br())
    }
}
impl ProjEffectAddCmdBr {
    pub fn into_sol(self) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddProjEffect(self)
    }
}
impl ProjEffectChangeCmd {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeProjEffect(self.into_ctx_item_br(item_id))
    }
}
impl ProjEffectChangeCmdBr {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeProjEffect(self.into_ctx_item_br(item_id))
    }
}
// Item - rig
impl RigAddCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddRig(self.into_ctx_fit_br(fit_id))
    }
}
impl RigChangeCmd {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeRig(self.into_ctx_item_br(item_id))
    }
}
// Item - service
impl ServiceAddCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddService(self.into_ctx_fit_br(fit_id))
    }
}
impl ServiceChangeCmd {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeService(self.into_ctx_item_br(item_id))
    }
}
// Item - ship
impl ShipSetCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::SetShip(self.into_ctx_fit_br(fit_id))
    }
}
impl ShipChangeCmd {
    pub fn into_sol_via_fit(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeShip(self.into_ctx_br_via_fit(fit_id))
    }
    pub fn into_sol_via_item(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeShip(self.into_ctx_br_via_item(item_id))
    }
}
impl ShipUnsetCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::UnsetShip(self.into_ctx_fit_br(fit_id))
    }
}
// Item - skill
impl SkillAddCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddSkill(self.into_ctx_fit_br(fit_id))
    }
}
impl SkillChangeCmd {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeSkill(self.into_ctx_item_br(item_id))
    }
}
// Item - stance
impl StanceSetCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::SetStance(self.into_ctx_fit_br(fit_id))
    }
}
impl StanceChangeCmd {
    pub fn into_sol_via_fit(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeStance(self.into_ctx_br_via_fit(fit_id))
    }
    pub fn into_sol_via_item(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeStance(self.into_ctx_br_via_item(item_id))
    }
}
impl StanceUnsetCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::UnsetStance(self.into_ctx_fit_br(fit_id))
    }
}
// Item - subsystem
impl SubsystemAddCmd {
    pub fn into_sol(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddSubsystem(self.into_ctx_fit_br(fit_id))
    }
}
impl SubsystemChangeCmd {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeSubsystem(self.into_ctx_item_br(item_id))
    }
}
// Item - system-wide effect
impl SwEffectAddCmd {
    pub fn into_sol(self) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AddSwEffect(self)
    }
}
impl SwEffectChangeCmd {
    pub fn into_sol(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChangeSwEffect(self.into_ctx_item_br(item_id))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolChangeEnumCmd {
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
            Self::SetCharacter(cmd) => SolCtlCmdRendered::SetCharacter(cmd.render(resps)?),
            Self::ChangeCharacter(cmd) => SolCtlCmdRendered::ChangeCharacter(cmd.render(resps)?),
            Self::UnsetCharacter(cmd) => SolCtlCmdRendered::UnsetCharacter(cmd.render(resps)?),
            // Item - charge
            Self::ChangeCharge(cmd) => SolCtlCmdRendered::ChangeCharge(cmd.render(resps)?),
            // Item - drone
            Self::AddDrone(cmd) => SolCtlCmdRendered::AddDrone(cmd.render(resps)?),
            Self::ChangeDrone(cmd) => SolCtlCmdRendered::ChangeDrone(cmd.render(resps)?),
            // Item - fighter
            Self::AddFighter(cmd) => SolCtlCmdRendered::AddFighter(cmd.render(resps)?),
            Self::ChangeFighter(cmd) => SolCtlCmdRendered::ChangeFighter(cmd.render(resps)?),
            // Item - fit-wide effect
            Self::AddFwEffect(cmd) => SolCtlCmdRendered::AddFwEffect(cmd.render(resps)?),
            Self::ChangeFwEffect(cmd) => SolCtlCmdRendered::ChangeFwEffect(cmd.render(resps)?),
            // Item - implant
            Self::AddImplant(cmd) => SolCtlCmdRendered::AddImplant(cmd.render(resps)?),
            Self::ChangeImplant(cmd) => SolCtlCmdRendered::ChangeImplant(cmd.render(resps)?),
            // Item - module
            Self::AddModule(cmd) => SolCtlCmdRendered::AddModule(cmd.render(resps)?),
            Self::ChangeModule(cmd) => SolCtlCmdRendered::ChangeModule(cmd.render(resps)?),
            // Item - projected effect
            Self::AddProjEffect(cmd) => SolCtlCmdRendered::AddProjEffect(cmd.render(resps)?),
            Self::ChangeProjEffect(cmd) => SolCtlCmdRendered::ChangeProjEffect(cmd.render(resps)?),
            // Item - rig
            Self::AddRig(cmd) => SolCtlCmdRendered::AddRig(cmd.render(resps)?),
            Self::ChangeRig(cmd) => SolCtlCmdRendered::ChangeRig(cmd.render(resps)?),
            // Item - service
            Self::AddService(cmd) => SolCtlCmdRendered::AddService(cmd.render(resps)?),
            Self::ChangeService(cmd) => SolCtlCmdRendered::ChangeService(cmd.render(resps)?),
            // Item - ship
            Self::SetShip(cmd) => SolCtlCmdRendered::SetShip(cmd.render(resps)?),
            Self::ChangeShip(cmd) => SolCtlCmdRendered::ChangeShip(cmd.render(resps)?),
            Self::UnsetShip(cmd) => SolCtlCmdRendered::UnsetShip(cmd.render(resps)?),
            // Item - skill
            Self::AddSkill(cmd) => SolCtlCmdRendered::AddSkill(cmd.render(resps)?),
            Self::ChangeSkill(cmd) => SolCtlCmdRendered::ChangeSkill(cmd.render(resps)?),
            // Item - stance
            Self::SetStance(cmd) => SolCtlCmdRendered::SetStance(cmd.render(resps)?),
            Self::ChangeStance(cmd) => SolCtlCmdRendered::ChangeStance(cmd.render(resps)?),
            Self::UnsetStance(cmd) => SolCtlCmdRendered::UnsetStance(cmd.render(resps)?),
            // Item - subsystem
            Self::AddSubsystem(cmd) => SolCtlCmdRendered::AddSubsystem(cmd.render(resps)?),
            Self::ChangeSubsystem(cmd) => SolCtlCmdRendered::ChangeSubsystem(cmd.render(resps)?),
            // Item - system-wide effect
            Self::AddSwEffect(cmd) => SolCtlCmdRendered::AddSwEffect(cmd),
            Self::ChangeSwEffect(cmd) => SolCtlCmdRendered::ChangeSwEffect(cmd.render(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCtlCmdRendered {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<CtlCmdResp, SolChangeEnumError> {
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
pub enum SolChangeEnumError {
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
    CharacterSet(#[from] FitGetCharacterSetError),
    #[error("failed to change character")]
    CharacterChange(#[from] CharacterChangeError),
    #[error("failed to unset character")]
    CharacterUnset(#[from] FitGetCharacterUnsetError),
    // Item - charge
    #[error("failed to change charge")]
    ChargeChange(#[from] ItemGetChargeChangeError),
    // Item - drone
    #[error("failed to add drone")]
    DroneAdd(#[from] FitGetDroneAddError),
    #[error("failed to change drone")]
    DroneChange(#[from] ItemGetDroneChangeError),
    // Item - fighter
    #[error("failed to add fighter")]
    FighterAdd(#[from] FitGetFighterAddError),
    #[error("failed to change fighter")]
    FighterChange(#[from] ItemGetFighterChangeError),
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
    ModuleAdd(#[from] FitGetModuleAddError),
    #[error("failed to change module")]
    ModuleChange(#[from] ItemGetModuleChangeError),
    // Item - projected effect
    #[error("failed to add projected effect")]
    ProjEffectAdd(#[from] ProjEffectAddError),
    #[error("failed to change projected effect")]
    ProjEffectChange(#[from] ItemGetProjEffectChangeError),
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
    ShipSet(#[from] FitGetShipSetError),
    #[error("failed to change ship")]
    ShipChange(#[from] ShipChangeError),
    #[error("failed to unset ship")]
    ShipUnset(#[from] FitGetShipUnsetError),
    // Item - skill
    #[error("failed to add skill")]
    SkillAdd(#[from] FitGetSkillAddError),
    #[error("failed to change skill")]
    SkillChange(#[from] ItemGetSkillChangeError),
    // Item - stance
    #[error("failed to set stance")]
    StanceSet(#[from] FitGetStanceSetError),
    #[error("failed to change stance")]
    StanceChange(#[from] StanceChangeError),
    #[error("failed to unset stance")]
    StanceUnset(#[from] FitGetStanceUnsetError),
    // Item - subsystem
    #[error("failed to add subsystem")]
    SubsystemAdd(#[from] FitGetSubsystemAddError),
    #[error("failed to change subsystem")]
    SubsystemChange(#[from] ItemGetSubsystemChangeError),
    // Item - system-wide effect
    #[error("failed to change system-wide effect")]
    SwEffectChange(#[from] ItemGetSwEffectChangeError),
}
