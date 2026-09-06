use crate::{
    AutochargeChangeCmd, BoosterAddCmd, BoosterChangeCmd, CharacterChangeCmd, CharacterSetCmd, CharacterUnsetCmd,
    ChargeChangeCmd, CmdResp, CmdResps, DroneAddCmd, DroneAddCmdBr, DroneChangeCmd, DroneChangeCmdBr, FighterAddCmd,
    FighterAddCmdBr, FighterChangeCmd, FighterChangeCmdBr, FitAddCmd, FitAddCmdBr, FitAddCmdGen, FitChangeCmd,
    FitChangeCmdBr, FitId, FitIdBr, FitRemoveCmd, FleetAddCmd, FleetAddCmdBr, FleetAddCmdGen, FleetChangeCmd,
    FleetChangeCmdBr, FleetId, FleetIdBr, FleetRemoveCmd, FwEffectAddCmd, FwEffectChangeCmd, ImplantAddCmd,
    ImplantChangeCmd, ItemAutodetectAddCmd, ItemId, ItemIdBr, ItemRemoveCmd, ModuleAddCmd, ModuleAddCmdBr,
    ModuleChangeCmd, ModuleChangeCmdBr, ProjEffectAddCmd, ProjEffectAddCmdBr, ProjEffectAddCmdGen, ProjEffectChangeCmd,
    ProjEffectChangeCmdBr, RigAddCmd, RigChangeCmd, ServiceAddCmd, ServiceChangeCmd, ShipChangeCmd, ShipSetCmd,
    ShipUnsetCmd, SkillAddCmd, SkillChangeCmd, SolChangeCmd, StanceChangeCmd, StanceSetCmd, StanceUnsetCmd,
    SubsystemAddCmd, SubsystemChangeCmd, SwEffectAddCmd, SwEffectChangeCmd,
    ctl::core::{
        AutochargeChangeCmdCtxItemGen, BoosterAddCmdCtxFitGen, BoosterChangeCmdCtxItemGen, CharacterChangeCmdCtxAnyGen,
        CharacterSetCmdCtxFitGen, CharacterUnsetCmdCtxFitGen, ChargeChangeCmdCtxItemGen, DroneAddCmdCtxFitGen,
        DroneChangeCmdCtxItemGen, FighterAddCmdCtxFitGen, FighterChangeCmdCtxItemGen, FitChangeCmdCtxFitGen,
        FitRemoveCmdCtxFitGen, FleetChangeCmdCtxFleetGen, FleetRemoveCmdCtxFleetGen, FwEffectAddCmdCtxFitGen,
        FwEffectChangeCmdCtxItemGen, ImplantAddCmdCtxFitGen, ImplantChangeCmdCtxItemGen, ItemAutodetectAddCmdCtxFitGen,
        ItemRemoveCmdCtxItemGen, ModuleAddCmdCtxFitGen, ModuleChangeCmdCtxItemGen, ProjEffectChangeCmdCtxItemGen,
        RigAddCmdCtxFitGen, RigChangeCmdCtxItemGen, ServiceAddCmdCtxFitGen, ServiceChangeCmdCtxItemGen,
        ShipChangeCmdCtxAnyGen, ShipSetCmdCtxFitGen, ShipUnsetCmdCtxFitGen, SkillAddCmdCtxFitGen,
        SkillChangeCmdCtxItemGen, StanceChangeCmdCtxAnyGen, StanceSetCmdCtxFitGen, StanceUnsetCmdCtxFitGen,
        SubsystemAddCmdCtxFitGen, SubsystemChangeCmdCtxItemGen, SwEffectChangeCmdCtxItemGen,
    },
    err::{
        BrResolveError, CharacterChangeError, FitAddError, FitGetBoosterAddError, FitGetCharacterSetError,
        FitGetCharacterUnsetError, FitGetDroneAddError, FitGetFighterAddError, FitGetFitChangeError,
        FitGetFitRemoveError, FitGetFwEffectAddError, FitGetImplantAddError, FitGetItemAutodetectAddError,
        FitGetModuleAddError, FitGetRigAddError, FitGetServiceAddError, FitGetShipSetError, FitGetShipUnsetError,
        FitGetSkillAddError, FitGetStanceSetError, FitGetStanceUnsetError, FitGetSubsystemAddError, FleetAddError,
        FleetGetFleetChangeError, FleetGetFleetRemoveError, ItemGetAutochargeChangeError, ItemGetBoosterChangeError,
        ItemGetChargeChangeError, ItemGetDroneChangeError, ItemGetFighterChangeError, ItemGetFwEffectChangeError,
        ItemGetImplantChangeError, ItemGetItemRemoveError, ItemGetModuleChangeError, ItemGetProjEffectChangeError,
        ItemGetRigChangeError, ItemGetServiceChangeError, ItemGetSkillChangeError, ItemGetSubsystemChangeError,
        ItemGetSwEffectChangeError, ProjEffectAddError, ShipChangeError, StanceChangeError,
    },
    shared::CmdResidue,
};

pub type SolChangeEnumCmd = SolChangeEnumCmdGen<FleetId, FitId, ItemId>;
pub type SolChangeEnumCmdBr = SolChangeEnumCmdGen<FleetIdBr, FitIdBr, ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case"),
    serde(bound(deserialize = "L: serde::Deserialize<'de>, F: serde::Deserialize<'de>, I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub enum SolChangeEnumCmdGen<L, F, I> {
    // Solar system
    SolChange(SolChangeCmd),
    // Fleet
    FleetAdd(FleetAddCmdGen<F>),
    FleetChange(FleetChangeCmdCtxFleetGen<L, F>),
    FleetRemove(FleetRemoveCmdCtxFleetGen<L>),
    // Fit
    FitAdd(FitAddCmdGen<L>),
    FitChange(FitChangeCmdCtxFitGen<L, F>),
    FitRemove(FitRemoveCmdCtxFitGen<F>),
    // Item
    ItemAutodetectAdd(ItemAutodetectAddCmdCtxFitGen<F>),
    ItemRemove(ItemRemoveCmdCtxItemGen<I>),
    // Item - autocharge
    AutochargeChange(AutochargeChangeCmdCtxItemGen<I>),
    // Item - booster
    BoosterAdd(BoosterAddCmdCtxFitGen<F>),
    BoosterChange(BoosterChangeCmdCtxItemGen<I>),
    // Item - character
    CharacterSet(CharacterSetCmdCtxFitGen<F>),
    CharacterChange(CharacterChangeCmdCtxAnyGen<F, I>),
    CharacterUnset(CharacterUnsetCmdCtxFitGen<F>),
    // Item - charge
    ChargeChange(ChargeChangeCmdCtxItemGen<I>),
    // Item - drone
    DroneAdd(DroneAddCmdCtxFitGen<F, I>),
    DroneChange(DroneChangeCmdCtxItemGen<I>),
    // Item - fighter
    FighterAdd(FighterAddCmdCtxFitGen<F, I>),
    FighterChange(FighterChangeCmdCtxItemGen<I>),
    // Item - fit-wide effect
    FwEffectAdd(FwEffectAddCmdCtxFitGen<F>),
    FwEffectChange(FwEffectChangeCmdCtxItemGen<I>),
    // Item - implant
    ImplantAdd(ImplantAddCmdCtxFitGen<F>),
    ImplantChange(ImplantChangeCmdCtxItemGen<I>),
    // Item - module
    ModuleAdd(ModuleAddCmdCtxFitGen<F, I>),
    ModuleChange(ModuleChangeCmdCtxItemGen<I>),
    // Item - projected effect
    ProjEffectAdd(ProjEffectAddCmdGen<I>),
    ProjEffectChange(ProjEffectChangeCmdCtxItemGen<I>),
    // Item - rig
    RigAdd(RigAddCmdCtxFitGen<F>),
    RigChange(RigChangeCmdCtxItemGen<I>),
    // Item - service
    ServiceAdd(ServiceAddCmdCtxFitGen<F>),
    ServiceChange(ServiceChangeCmdCtxItemGen<I>),
    // Item - ship
    ShipSet(ShipSetCmdCtxFitGen<F>),
    ShipChange(ShipChangeCmdCtxAnyGen<F, I>),
    ShipUnset(ShipUnsetCmdCtxFitGen<F>),
    // Item - skill
    SkillAdd(SkillAddCmdCtxFitGen<F>),
    SkillChange(SkillChangeCmdCtxItemGen<I>),
    // Item - stance
    StanceSet(StanceSetCmdCtxFitGen<F>),
    StanceChange(StanceChangeCmdCtxAnyGen<F, I>),
    StanceUnset(StanceUnsetCmdCtxFitGen<F>),
    // Item - subsystem
    SubsystemAdd(SubsystemAddCmdCtxFitGen<F>),
    SubsystemChange(SubsystemChangeCmdCtxItemGen<I>),
    // Item - system-wide effect
    SwEffectAdd(SwEffectAddCmd),
    SwEffectChange(SwEffectChangeCmdCtxItemGen<I>),
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
impl ItemAutodetectAddCmd {
    pub fn into_sol_ctl(self, fit_id: FitId) -> SolChangeEnumCmd {
        SolChangeEnumCmd::ItemAutodetectAdd(self.into_ctx_fit(fit_id))
    }
    pub fn into_sol_ctl_br(self, fit_id: impl Into<FitIdBr>) -> SolChangeEnumCmdBr {
        SolChangeEnumCmdBr::ItemAutodetectAdd(self.into_ctx_fit_br(fit_id))
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
            Self::ItemAutodetectAdd(cmd) => SolChangeEnumCmd::ItemAutodetectAdd(cmd.br_resolve(resps)?),
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
impl<L, F, I> SolChangeEnumCmdGen<L, F, I> {
    pub(crate) fn exec_residue(&self) -> CmdResidue {
        match self {
            // Solar system
            Self::SolChange(cmd) => cmd.exec_residue(),
            // Fleet
            Self::FleetAdd(cmd) => cmd.exec_residue(),
            Self::FleetChange(cmd) => cmd.exec_residue(),
            Self::FleetRemove(cmd) => cmd.exec_residue(),
            // Fit
            Self::FitAdd(cmd) => cmd.exec_residue(),
            Self::FitChange(cmd) => cmd.exec_residue(),
            Self::FitRemove(cmd) => cmd.exec_residue(),
            // Item
            Self::ItemAutodetectAdd(cmd) => cmd.exec_residue(),
            Self::ItemRemove(cmd) => cmd.exec_residue(),
            // Item - autocharge
            Self::AutochargeChange(cmd) => cmd.exec_residue(),
            // Item - booster
            Self::BoosterAdd(cmd) => cmd.exec_residue(),
            Self::BoosterChange(cmd) => cmd.exec_residue(),
            // Item - character
            Self::CharacterSet(cmd) => cmd.exec_residue(),
            Self::CharacterChange(cmd) => cmd.exec_residue(),
            Self::CharacterUnset(cmd) => cmd.exec_residue(),
            // Item - charge
            Self::ChargeChange(cmd) => cmd.exec_residue(),
            // Item - drone
            Self::DroneAdd(cmd) => cmd.exec_residue(),
            Self::DroneChange(cmd) => cmd.exec_residue(),
            // Item - fighter
            Self::FighterAdd(cmd) => cmd.exec_residue(),
            Self::FighterChange(cmd) => cmd.exec_residue(),
            // Item - fit-wide effect
            Self::FwEffectAdd(cmd) => cmd.exec_residue(),
            Self::FwEffectChange(cmd) => cmd.exec_residue(),
            // Item - implant
            Self::ImplantAdd(cmd) => cmd.exec_residue(),
            Self::ImplantChange(cmd) => cmd.exec_residue(),
            // Item - module
            Self::ModuleAdd(cmd) => cmd.exec_residue(),
            Self::ModuleChange(cmd) => cmd.exec_residue(),
            // Item - projected effect
            Self::ProjEffectAdd(cmd) => cmd.exec_residue(),
            Self::ProjEffectChange(cmd) => cmd.exec_residue(),
            // Item - rig
            Self::RigAdd(cmd) => cmd.exec_residue(),
            Self::RigChange(cmd) => cmd.exec_residue(),
            // Item - service
            Self::ServiceAdd(cmd) => cmd.exec_residue(),
            Self::ServiceChange(cmd) => cmd.exec_residue(),
            // Item - ship
            Self::ShipSet(cmd) => cmd.exec_residue(),
            Self::ShipChange(cmd) => cmd.exec_residue(),
            Self::ShipUnset(cmd) => cmd.exec_residue(),
            // Item - skill
            Self::SkillAdd(cmd) => cmd.exec_residue(),
            Self::SkillChange(cmd) => cmd.exec_residue(),
            // Item - stance
            Self::StanceSet(cmd) => cmd.exec_residue(),
            Self::StanceChange(cmd) => cmd.exec_residue(),
            Self::StanceUnset(cmd) => cmd.exec_residue(),
            // Item - subsystem
            Self::SubsystemAdd(cmd) => cmd.exec_residue(),
            Self::SubsystemChange(cmd) => cmd.exec_residue(),
            // Item - system-wide effect
            Self::SwEffectAdd(cmd) => cmd.exec_residue(),
            Self::SwEffectChange(cmd) => cmd.exec_residue(),
        }
    }
}

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
            Self::ItemAutodetectAdd(cmd) => cmd.execute(core_sol)?.into(),
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
    #[error("failed to add autodetected item")]
    ItemAutodetectAdd(#[from] FitGetItemAutodetectAddError),
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
