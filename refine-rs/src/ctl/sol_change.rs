use crate::{
    AutochargeChangeCmd, BoosterAddCmd, BoosterChangeCmd, CharacterChangeCmd, CharacterSetCmd, CharacterUnsetCmd,
    ChargeChangeCmd, CmdResp, CmdResps, DroneAddCmd, DroneAddCmdBr, DroneChangeCmd, DroneChangeCmdBr, FighterAddCmd,
    FighterAddCmdBr, FighterChangeCmd, FighterChangeCmdBr, FitAddCmd, FitAddCmdBr, FitChangeCmd, FitChangeCmdBr, FitId,
    FitIdBr, FitRemoveCmd, FleetAddCmd, FleetAddCmdBr, FleetChangeCmd, FleetChangeCmdBr, FleetId, FleetIdBr,
    FleetRemoveCmd, FwEffectAddCmd, FwEffectChangeCmd, ImplantAddCmd, ImplantChangeCmd, ItemId, ItemIdBr,
    ItemRemoveCmd, ModuleAddCmd, ModuleAddCmdBr, ModuleChangeCmd, ModuleChangeCmdBr, ProjEffectAddCmd,
    ProjEffectAddCmdBr, ProjEffectChangeCmd, ProjEffectChangeCmdBr, RigAddCmd, RigChangeCmd, ServiceAddCmd,
    ServiceChangeCmd, ShipChangeCmd, ShipSetCmd, ShipUnsetCmd, SkillAddCmd, SkillChangeCmd, SolChangeCmd,
    StanceChangeCmd, StanceSetCmd, StanceUnsetCmd, SubsystemAddCmd, SubsystemChangeCmd, SwEffectAddCmd,
    SwEffectChangeCmd,
    ctl::core::{
        AutochargeChangeCmdCtxItem, AutochargeChangeCmdCtxItemBr, BoosterAddCmdCtxFit, BoosterAddCmdCtxFitBr,
        BoosterChangeCmdCtxItem, BoosterChangeCmdCtxItemBr, CharacterChangeCmdCtxAny, CharacterChangeCmdCtxAnyBr,
        CharacterSetCmdCtxFit, CharacterSetCmdCtxFitBr, CharacterUnsetCmdCtxFit, CharacterUnsetCmdCtxFitBr,
        ChargeChangeCmdCtxItem, ChargeChangeCmdCtxItemBr, DroneAddCmdCtxFit, DroneAddCmdCtxFitBr,
        DroneChangeCmdCtxItem, DroneChangeCmdCtxItemBr, FighterAddCmdCtxFit, FighterAddCmdCtxFitBr,
        FighterChangeCmdCtxItem, FighterChangeCmdCtxItemBr, FitChangeCmdCtxFit, FitChangeCmdCtxFitBr,
        FitRemoveCmdCtxFit, FitRemoveCmdCtxFitBr, FleetChangeCmdCtxFleet, FleetChangeCmdCtxFleetBr,
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
        BrResolveError, CharacterChangeError, FitAddError, FitGetBoosterAddError, FitGetCharacterSetError,
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
#[derive(Clone)]
pub enum SolChangeEnumCmd {
    // Solar system
    SolChange(SolChangeCmd),
    // Fleet
    FleetAdd(FleetAddCmd),
    FleetChange(FleetChangeCmdCtxFleet),
    FleetRemove(FleetRemoveCmdCtxFleet),
    // Fit
    FitAdd(FitAddCmd),
    FitChange(FitChangeCmdCtxFit),
    FitRemove(FitRemoveCmdCtxFit),
    // Item
    ItemRemove(ItemRemoveCmdCtxItem),
    // Item - autocharge
    AutochargeChange(AutochargeChangeCmdCtxItem),
    // Item - booster
    BoosterAdd(BoosterAddCmdCtxFit),
    BoosterChange(BoosterChangeCmdCtxItem),
    // Item - character
    CharacterSet(CharacterSetCmdCtxFit),
    CharacterChange(CharacterChangeCmdCtxAny),
    CharacterUnset(CharacterUnsetCmdCtxFit),
    // Item - charge
    ChargeChange(ChargeChangeCmdCtxItem),
    // Item - drone
    DroneAdd(DroneAddCmdCtxFit),
    DroneChange(DroneChangeCmdCtxItem),
    // Item - fighter
    FighterAdd(FighterAddCmdCtxFit),
    FighterChange(FighterChangeCmdCtxItem),
    // Item - fit-wide effect
    FwEffectAdd(FwEffectAddCmdCtxFit),
    FwEffectChange(FwEffectChangeCmdCtxItem),
    // Item - implant
    ImplantAdd(ImplantAddCmdCtxFit),
    ImplantChange(ImplantChangeCmdCtxItem),
    // Item - module
    ModuleAdd(ModuleAddCmdCtxFit),
    ModuleChange(ModuleChangeCmdCtxItem),
    // Item - projected effect
    ProjEffectAdd(ProjEffectAddCmd),
    ProjEffectChange(ProjEffectChangeCmdCtxItem),
    // Item - rig
    RigAdd(RigAddCmdCtxFit),
    RigChange(RigChangeCmdCtxItem),
    // Item - service
    ServiceAdd(ServiceAddCmdCtxFit),
    ServiceChange(ServiceChangeCmdCtxItem),
    // Item - ship
    ShipSet(ShipSetCmdCtxFit),
    ShipChange(ShipChangeCmdCtxAny),
    ShipUnset(ShipUnsetCmdCtxFit),
    // Item - skill
    SkillAdd(SkillAddCmdCtxFit),
    SkillChange(SkillChangeCmdCtxItem),
    // Item - stance
    StanceSet(StanceSetCmdCtxFit),
    StanceChange(StanceChangeCmdCtxAny),
    StanceUnset(StanceUnsetCmdCtxFit),
    // Item - subsystem
    SubsystemAdd(SubsystemAddCmdCtxFit),
    SubsystemChange(SubsystemChangeCmdCtxItem),
    // Item - system-wide effect
    SwEffectAdd(SwEffectAddCmd),
    SwEffectChange(SwEffectChangeCmdCtxItem),
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
#[derive(Clone)]
pub enum SolChangeEnumCmdBr {
    // Solar system
    SolChange(SolChangeCmd),
    // Fleet
    FleetAdd(FleetAddCmdBr),
    FleetChange(FleetChangeCmdCtxFleetBr),
    FleetRemove(FleetRemoveCmdCtxFleetBr),
    // Fit
    FitAdd(FitAddCmdBr),
    FitChange(FitChangeCmdCtxFitBr),
    FitRemove(FitRemoveCmdCtxFitBr),
    // Item
    ItemRemove(ItemRemoveCmdCtxItemBr),
    // Item - autocharge
    AutochargeChange(AutochargeChangeCmdCtxItemBr),
    // Item - booster
    BoosterAdd(BoosterAddCmdCtxFitBr),
    BoosterChange(BoosterChangeCmdCtxItemBr),
    // Item - character
    CharacterSet(CharacterSetCmdCtxFitBr),
    CharacterChange(CharacterChangeCmdCtxAnyBr),
    CharacterUnset(CharacterUnsetCmdCtxFitBr),
    // Item - charge
    ChargeChange(ChargeChangeCmdCtxItemBr),
    // Item - drone
    DroneAdd(DroneAddCmdCtxFitBr),
    DroneChange(DroneChangeCmdCtxItemBr),
    // Item - fighter
    FighterAdd(FighterAddCmdCtxFitBr),
    FighterChange(FighterChangeCmdCtxItemBr),
    // Item - fit-wide effect
    FwEffectAdd(FwEffectAddCmdCtxFitBr),
    FwEffectChange(FwEffectChangeCmdCtxItemBr),
    // Item - implant
    ImplantAdd(ImplantAddCmdCtxFitBr),
    ImplantChange(ImplantChangeCmdCtxItemBr),
    // Item - module
    ModuleAdd(ModuleAddCmdCtxFitBr),
    ModuleChange(ModuleChangeCmdCtxItemBr),
    // Item - projected effect
    ProjEffectAdd(ProjEffectAddCmdBr),
    ProjEffectChange(ProjEffectChangeCmdCtxItemBr),
    // Item - rig
    RigAdd(RigAddCmdCtxFitBr),
    RigChange(RigChangeCmdCtxItemBr),
    // Item - service
    ServiceAdd(ServiceAddCmdCtxFitBr),
    ServiceChange(ServiceChangeCmdCtxItemBr),
    // Item - ship
    ShipSet(ShipSetCmdCtxFitBr),
    ShipChange(ShipChangeCmdCtxAnyBr),
    ShipUnset(ShipUnsetCmdCtxFitBr),
    // Item - skill
    SkillAdd(SkillAddCmdCtxFitBr),
    SkillChange(SkillChangeCmdCtxItemBr),
    // Item - stance
    StanceSet(StanceSetCmdCtxFitBr),
    StanceChange(StanceChangeCmdCtxAnyBr),
    StanceUnset(StanceUnsetCmdCtxFitBr),
    // Item - subsystem
    SubsystemAdd(SubsystemAddCmdCtxFitBr),
    SubsystemChange(SubsystemChangeCmdCtxItemBr),
    // Item - system-wide effect
    SwEffectAdd(SwEffectAddCmd),
    SwEffectChange(SwEffectChangeCmdCtxItemBr),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
// Solar system
impl SolChangeCmd {
    pub fn into_sol_ctl(self) -> SolChangeEnumCmd {
        SolChangeEnumCmd::SolChange(self)
    }
    pub fn into_sol_ctl_br(self) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::SolChange(self)
    }
}
// Fleet
impl FleetAddCmd {
    pub fn into_sol_ctl(self) -> SolChangeEnumCmd {
        SolChangeEnumCmd::FleetAdd(self)
    }
    pub fn into_sol_ctl_br(self) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::FleetAdd(self.into_br())
    }
}
impl FleetAddCmdBr {
    pub fn into_sol_ctl_br(self) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::FleetAdd(self)
    }
}
impl FleetChangeCmd {
    pub fn into_sol_ctl(self, fleet_id: FleetId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::FleetChange(self.into_ctx_fleet(fleet_id))
    }
    pub fn into_sol_ctl_br(self, fleet_id: impl Into<FleetIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::FleetChange(self.into_ctx_fleet_br(fleet_id))
    }
}
impl FleetChangeCmdBr {
    pub fn into_sol_ctl_br(self, fleet_id: impl Into<FleetIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::FleetChange(self.into_ctx_fleet_br(fleet_id))
    }
}
impl FleetRemoveCmd {
    pub fn into_sol_ctl(self, fleet_id: FleetId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::FleetRemove(self.into_ctx_fleet(fleet_id))
    }
    pub fn into_sol_ctl_br(self, fleet_id: impl Into<FleetIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::FleetRemove(self.into_ctx_fleet_br(fleet_id))
    }
}
// Fit
impl FitAddCmd {
    pub fn into_sol_ctl(self) -> SolChangeEnumCmd {
        SolChangeEnumCmd::FitAdd(self)
    }
    pub fn into_sol_ctl_br(self) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::FitAdd(self.into_br())
    }
}
impl FitAddCmdBr {
    pub fn into_sol_ctl_br(self) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::FitAdd(self)
    }
}
impl FitChangeCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::FitChange(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::FitChange(self.into_ctx_fit_br(fit_id))
    }
}
impl FitChangeCmdBr {
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::FitChange(self.into_ctx_fit_br(fit_id))
    }
}
impl FitRemoveCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::FitRemove(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::FitRemove(self.into_ctx_fit_br(fit_id))
    }
}
// Item
impl ItemRemoveCmd {
    pub fn into_sol_ctl(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ItemRemove(self.into_ctx_item(item_id))
    }
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ItemRemove(self.into_ctx_item_br(item_id))
    }
}
// Item - autocharge
impl AutochargeChangeCmd {
    pub fn into_sol_ctl(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::AutochargeChange(self.into_ctx_item(item_id))
    }
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::AutochargeChange(self.into_ctx_item_br(item_id))
    }
}
// Item - booster
impl BoosterAddCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::BoosterAdd(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::BoosterAdd(self.into_ctx_fit_br(fit_id))
    }
}
impl BoosterChangeCmd {
    pub fn into_sol_ctl(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::BoosterChange(self.into_ctx_item(item_id))
    }
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::BoosterChange(self.into_ctx_item_br(item_id))
    }
}
// Item - character
impl CharacterSetCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::CharacterSet(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::CharacterSet(self.into_ctx_fit_br(fit_id))
    }
}
impl CharacterChangeCmd {
    pub fn into_sol_ctl_via_fit(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::CharacterChange(self.into_ctx_via_fit(fit_id))
    }
    pub fn into_sol_ctl_via_item(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::CharacterChange(self.into_ctx_via_item(item_id))
    }
    pub fn into_sol_ctl_br_via_fit(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::CharacterChange(self.into_ctx_br_via_fit(fit_id))
    }
    pub fn into_sol_ctl_br_via_item(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::CharacterChange(self.into_ctx_br_via_item(item_id))
    }
}
impl CharacterUnsetCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::CharacterUnset(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::CharacterUnset(self.into_ctx_fit_br(fit_id))
    }
}
// Item - charge
impl ChargeChangeCmd {
    pub fn into_sol_ctl(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ChargeChange(self.into_ctx_item(item_id))
    }
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ChargeChange(self.into_ctx_item_br(item_id))
    }
}
// Item - drone
impl DroneAddCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::DroneAdd(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::DroneAdd(self.into_ctx_fit_br(fit_id))
    }
}
impl DroneAddCmdBr {
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::DroneAdd(self.into_ctx_fit_br(fit_id))
    }
}
impl DroneChangeCmd {
    pub fn into_sol_ctl(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::DroneChange(self.into_ctx_item(item_id))
    }
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::DroneChange(self.into_ctx_item_br(item_id))
    }
}
impl DroneChangeCmdBr {
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::DroneChange(self.into_ctx_item_br(item_id))
    }
}
// Item - fighter
impl FighterAddCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::FighterAdd(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::FighterAdd(self.into_ctx_fit_br(fit_id))
    }
}
impl FighterAddCmdBr {
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::FighterAdd(self.into_ctx_fit_br(fit_id))
    }
}
impl FighterChangeCmd {
    pub fn into_sol_ctl(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::FighterChange(self.into_ctx_item(item_id))
    }
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::FighterChange(self.into_ctx_item_br(item_id))
    }
}
impl FighterChangeCmdBr {
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::FighterChange(self.into_ctx_item_br(item_id))
    }
}
// Item - fit-wide effect
impl FwEffectAddCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::FwEffectAdd(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::FwEffectAdd(self.into_ctx_fit_br(fit_id))
    }
}
impl FwEffectChangeCmd {
    pub fn into_sol_ctl(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::FwEffectChange(self.into_ctx_item(item_id))
    }
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::FwEffectChange(self.into_ctx_item_br(item_id))
    }
}
// Item - implant
impl ImplantAddCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ImplantAdd(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ImplantAdd(self.into_ctx_fit_br(fit_id))
    }
}
impl ImplantChangeCmd {
    pub fn into_sol_ctl(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ImplantChange(self.into_ctx_item(item_id))
    }
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ImplantChange(self.into_ctx_item_br(item_id))
    }
}
// Item - module
impl ModuleAddCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ModuleAdd(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ModuleAdd(self.into_ctx_fit_br(fit_id))
    }
}
impl ModuleAddCmdBr {
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ModuleAdd(self.into_ctx_fit_br(fit_id))
    }
}
impl ModuleChangeCmd {
    pub fn into_sol_ctl(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ModuleChange(self.into_ctx_item(item_id))
    }
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ModuleChange(self.into_ctx_item_br(item_id))
    }
}
impl ModuleChangeCmdBr {
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ModuleChange(self.into_ctx_item_br(item_id))
    }
}
// Item - projected effect
impl ProjEffectAddCmd {
    pub fn into_sol_ctl(self) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ProjEffectAdd(self)
    }
    pub fn into_sol_ctl_br(self) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ProjEffectAdd(self.into_br())
    }
}
impl ProjEffectAddCmdBr {
    pub fn into_sol_ctl_br(self) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ProjEffectAdd(self)
    }
}
impl ProjEffectChangeCmd {
    pub fn into_sol_ctl(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ProjEffectChange(self.into_ctx_item(item_id))
    }
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ProjEffectChange(self.into_ctx_item_br(item_id))
    }
}
impl ProjEffectChangeCmdBr {
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ProjEffectChange(self.into_ctx_item_br(item_id))
    }
}
// Item - rig
impl RigAddCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::RigAdd(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::RigAdd(self.into_ctx_fit_br(fit_id))
    }
}
impl RigChangeCmd {
    pub fn into_sol_ctl(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::RigChange(self.into_ctx_item(item_id))
    }
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::RigChange(self.into_ctx_item_br(item_id))
    }
}
// Item - service
impl ServiceAddCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ServiceAdd(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ServiceAdd(self.into_ctx_fit_br(fit_id))
    }
}
impl ServiceChangeCmd {
    pub fn into_sol_ctl(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ServiceChange(self.into_ctx_item(item_id))
    }
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ServiceChange(self.into_ctx_item_br(item_id))
    }
}
// Item - ship
impl ShipSetCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ShipSet(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ShipSet(self.into_ctx_fit_br(fit_id))
    }
}
impl ShipChangeCmd {
    pub fn into_sol_ctl_via_fit(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ShipChange(self.into_ctx_via_fit(fit_id))
    }
    pub fn into_sol_ctl_via_item(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ShipChange(self.into_ctx_via_item(item_id))
    }
    pub fn into_sol_ctl_br_via_fit(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ShipChange(self.into_ctx_br_via_fit(fit_id))
    }
    pub fn into_sol_ctl_br_via_item(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ShipChange(self.into_ctx_br_via_item(item_id))
    }
}
impl ShipUnsetCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ShipUnset(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ShipUnset(self.into_ctx_fit_br(fit_id))
    }
}
// Item - skill
impl SkillAddCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::SkillAdd(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::SkillAdd(self.into_ctx_fit_br(fit_id))
    }
}
impl SkillChangeCmd {
    pub fn into_sol_ctl(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::SkillChange(self.into_ctx_item(item_id))
    }
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::SkillChange(self.into_ctx_item_br(item_id))
    }
}
// Item - stance
impl StanceSetCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::StanceSet(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::StanceSet(self.into_ctx_fit_br(fit_id))
    }
}
impl StanceChangeCmd {
    pub fn into_sol_ctl_via_fit(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::StanceChange(self.into_ctx_via_fit(fit_id))
    }
    pub fn into_sol_ctl_via_item(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::StanceChange(self.into_ctx_via_item(item_id))
    }
    pub fn into_sol_ctl_br_via_fit(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::StanceChange(self.into_ctx_br_via_fit(fit_id))
    }
    pub fn into_sol_ctl_br_via_item(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::StanceChange(self.into_ctx_br_via_item(item_id))
    }
}
impl StanceUnsetCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::StanceUnset(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::StanceUnset(self.into_ctx_fit_br(fit_id))
    }
}
// Item - subsystem
impl SubsystemAddCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::SubsystemAdd(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::SubsystemAdd(self.into_ctx_fit_br(fit_id))
    }
}
impl SubsystemChangeCmd {
    pub fn into_sol_ctl(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::SubsystemChange(self.into_ctx_item(item_id))
    }
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::SubsystemChange(self.into_ctx_item_br(item_id))
    }
}
// Item - system-wide effect
impl SwEffectAddCmd {
    pub fn into_sol_ctl(self) -> SolChangeEnumCmd {
        SolChangeEnumCmd::SwEffectAdd(self)
    }
    pub fn into_sol_ctl_br(self) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::SwEffectAdd(self)
    }
}
impl SwEffectChangeCmd {
    pub fn into_sol_ctl(self, item_id: ItemId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::SwEffectChange(self.into_ctx_item(item_id))
    }
    pub fn into_sol_ctl_br(self, item_id: impl Into<ItemIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::SwEffectChange(self.into_ctx_item_br(item_id))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolChangeEnumCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> Result<SolChangeEnumCmd, BrResolveError> {
        Ok(match self {
            // Solar system
            Self::SolChange(cmd) => SolChangeEnumCmd::SolChange(cmd),
            // Fleet
            Self::FleetAdd(cmd) => SolChangeEnumCmd::FleetAdd(cmd.br_resolve(resps)?),
            Self::FleetChange(cmd) => SolChangeEnumCmd::FleetChange(cmd.br_resolve(resps)?),
            Self::FleetRemove(cmd) => SolChangeEnumCmd::FleetRemove(cmd.br_resolve(resps)?),
            // Fit
            Self::FitAdd(cmd) => SolChangeEnumCmd::FitAdd(cmd.br_resolve(resps)?),
            Self::FitChange(cmd) => SolChangeEnumCmd::FitChange(cmd.br_resolve(resps)?),
            Self::FitRemove(cmd) => SolChangeEnumCmd::FitRemove(cmd.br_resolve(resps)?),
            // Item
            Self::ItemRemove(cmd) => SolChangeEnumCmd::ItemRemove(cmd.br_resolve(resps)?),
            // Item - autocharge
            Self::AutochargeChange(cmd) => SolChangeEnumCmd::AutochargeChange(cmd.br_resolve(resps)?),
            // Item - booster
            Self::BoosterAdd(cmd) => SolChangeEnumCmd::BoosterAdd(cmd.br_resolve(resps)?),
            Self::BoosterChange(cmd) => SolChangeEnumCmd::BoosterChange(cmd.br_resolve(resps)?),
            // Item - character
            Self::CharacterSet(cmd) => SolChangeEnumCmd::CharacterSet(cmd.br_resolve(resps)?),
            Self::CharacterChange(cmd) => SolChangeEnumCmd::CharacterChange(cmd.br_resolve(resps)?),
            Self::CharacterUnset(cmd) => SolChangeEnumCmd::CharacterUnset(cmd.br_resolve(resps)?),
            // Item - charge
            Self::ChargeChange(cmd) => SolChangeEnumCmd::ChargeChange(cmd.br_resolve(resps)?),
            // Item - drone
            Self::DroneAdd(cmd) => SolChangeEnumCmd::DroneAdd(cmd.br_resolve(resps)?),
            Self::DroneChange(cmd) => SolChangeEnumCmd::DroneChange(cmd.br_resolve(resps)?),
            // Item - fighter
            Self::FighterAdd(cmd) => SolChangeEnumCmd::FighterAdd(cmd.br_resolve(resps)?),
            Self::FighterChange(cmd) => SolChangeEnumCmd::FighterChange(cmd.br_resolve(resps)?),
            // Item - fit-wide effect
            Self::FwEffectAdd(cmd) => SolChangeEnumCmd::FwEffectAdd(cmd.br_resolve(resps)?),
            Self::FwEffectChange(cmd) => SolChangeEnumCmd::FwEffectChange(cmd.br_resolve(resps)?),
            // Item - implant
            Self::ImplantAdd(cmd) => SolChangeEnumCmd::ImplantAdd(cmd.br_resolve(resps)?),
            Self::ImplantChange(cmd) => SolChangeEnumCmd::ImplantChange(cmd.br_resolve(resps)?),
            // Item - module
            Self::ModuleAdd(cmd) => SolChangeEnumCmd::ModuleAdd(cmd.br_resolve(resps)?),
            Self::ModuleChange(cmd) => SolChangeEnumCmd::ModuleChange(cmd.br_resolve(resps)?),
            // Item - projected effect
            Self::ProjEffectAdd(cmd) => SolChangeEnumCmd::ProjEffectAdd(cmd.br_resolve(resps)?),
            Self::ProjEffectChange(cmd) => SolChangeEnumCmd::ProjEffectChange(cmd.br_resolve(resps)?),
            // Item - rig
            Self::RigAdd(cmd) => SolChangeEnumCmd::RigAdd(cmd.br_resolve(resps)?),
            Self::RigChange(cmd) => SolChangeEnumCmd::RigChange(cmd.br_resolve(resps)?),
            // Item - service
            Self::ServiceAdd(cmd) => SolChangeEnumCmd::ServiceAdd(cmd.br_resolve(resps)?),
            Self::ServiceChange(cmd) => SolChangeEnumCmd::ServiceChange(cmd.br_resolve(resps)?),
            // Item - ship
            Self::ShipSet(cmd) => SolChangeEnumCmd::ShipSet(cmd.br_resolve(resps)?),
            Self::ShipChange(cmd) => SolChangeEnumCmd::ShipChange(cmd.br_resolve(resps)?),
            Self::ShipUnset(cmd) => SolChangeEnumCmd::ShipUnset(cmd.br_resolve(resps)?),
            // Item - skill
            Self::SkillAdd(cmd) => SolChangeEnumCmd::SkillAdd(cmd.br_resolve(resps)?),
            Self::SkillChange(cmd) => SolChangeEnumCmd::SkillChange(cmd.br_resolve(resps)?),
            // Item - stance
            Self::StanceSet(cmd) => SolChangeEnumCmd::StanceSet(cmd.br_resolve(resps)?),
            Self::StanceChange(cmd) => SolChangeEnumCmd::StanceChange(cmd.br_resolve(resps)?),
            Self::StanceUnset(cmd) => SolChangeEnumCmd::StanceUnset(cmd.br_resolve(resps)?),
            // Item - subsystem
            Self::SubsystemAdd(cmd) => SolChangeEnumCmd::SubsystemAdd(cmd.br_resolve(resps)?),
            Self::SubsystemChange(cmd) => SolChangeEnumCmd::SubsystemChange(cmd.br_resolve(resps)?),
            // Item - system-wide effect
            Self::SwEffectAdd(cmd) => SolChangeEnumCmd::SwEffectAdd(cmd),
            Self::SwEffectChange(cmd) => SolChangeEnumCmd::SwEffectChange(cmd.br_resolve(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolChangeEnumCmd {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<CmdResp, SolChangeEnumError> {
        Ok(match self {
            // Solar system
            #[expect(clippy::unit_arg)]
            Self::SolChange(cmd) => cmd.execute(core_sol).into(),
            // Fleet
            Self::FleetAdd(cmd) => cmd.execute(core_sol)?.into(),
            Self::FleetChange(cmd) => cmd.execute(core_sol)?.into(),
            Self::FleetRemove(cmd) => cmd.execute(core_sol)?.into(),
            // Fit
            Self::FitAdd(cmd) => cmd.execute(core_sol)?.into(),
            Self::FitChange(cmd) => cmd.execute(core_sol)?.into(),
            Self::FitRemove(cmd) => cmd.execute(core_sol)?.into(),
            // Item
            Self::ItemRemove(cmd) => cmd.execute(core_sol)?.into(),
            // Item - autocharge
            Self::AutochargeChange(cmd) => cmd.execute(core_sol)?.into(),
            // Item - booster
            Self::BoosterAdd(cmd) => cmd.execute(core_sol)?.into(),
            Self::BoosterChange(cmd) => cmd.execute(core_sol)?.into(),
            // Item - character
            Self::CharacterSet(cmd) => cmd.execute(core_sol)?.into(),
            Self::CharacterChange(cmd) => cmd.execute(core_sol)?.into(),
            Self::CharacterUnset(cmd) => cmd.execute(core_sol)?.into(),
            // Item - charge
            Self::ChargeChange(cmd) => cmd.execute(core_sol)?.into(),
            // Item - drone
            Self::DroneAdd(cmd) => cmd.execute(core_sol)?.into(),
            Self::DroneChange(cmd) => cmd.execute(core_sol)?.into(),
            // Item - fighter
            Self::FighterAdd(cmd) => cmd.execute(core_sol)?.into(),
            Self::FighterChange(cmd) => cmd.execute(core_sol)?.into(),
            // Item - fit-wide effect
            Self::FwEffectAdd(cmd) => cmd.execute(core_sol)?.into(),
            Self::FwEffectChange(cmd) => cmd.execute(core_sol)?.into(),
            // Item - implant
            Self::ImplantAdd(cmd) => cmd.execute(core_sol)?.into(),
            Self::ImplantChange(cmd) => cmd.execute(core_sol)?.into(),
            // Item - module
            Self::ModuleAdd(cmd) => cmd.execute(core_sol)?.into(),
            Self::ModuleChange(cmd) => cmd.execute(core_sol)?.into(),
            // Item - projected effect
            Self::ProjEffectAdd(cmd) => cmd.execute(core_sol)?.into(),
            Self::ProjEffectChange(cmd) => cmd.execute(core_sol)?.into(),
            // Item - rig
            Self::RigAdd(cmd) => cmd.execute(core_sol)?.into(),
            Self::RigChange(cmd) => cmd.execute(core_sol)?.into(),
            // Item - service
            Self::ServiceAdd(cmd) => cmd.execute(core_sol)?.into(),
            Self::ServiceChange(cmd) => cmd.execute(core_sol)?.into(),
            // Item - ship
            Self::ShipSet(cmd) => cmd.execute(core_sol)?.into(),
            Self::ShipChange(cmd) => cmd.execute(core_sol)?.into(),
            Self::ShipUnset(cmd) => cmd.execute(core_sol)?.into(),
            // Item - skill
            Self::SkillAdd(cmd) => cmd.execute(core_sol)?.into(),
            Self::SkillChange(cmd) => cmd.execute(core_sol)?.into(),
            // Item - stance
            Self::StanceSet(cmd) => cmd.execute(core_sol)?.into(),
            Self::StanceChange(cmd) => cmd.execute(core_sol)?.into(),
            Self::StanceUnset(cmd) => cmd.execute(core_sol)?.into(),
            // Item - subsystem
            Self::SubsystemAdd(cmd) => cmd.execute(core_sol)?.into(),
            Self::SubsystemChange(cmd) => cmd.execute(core_sol)?.into(),
            // Item - system-wide effect
            Self::SwEffectAdd(cmd) => cmd.execute(core_sol).into(),
            Self::SwEffectChange(cmd) => cmd.execute(core_sol)?.into(),
        })
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
