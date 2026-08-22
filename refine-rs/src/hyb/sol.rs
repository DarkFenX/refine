use crate::{
    AutochargeChangeCmd, BoosterAddCmd, BoosterChangeCmd, CharacterChangeCmd, CharacterSetCmd, CharacterUnsetCmd,
    ChargeChangeCmd, CmdResp, CmdResps, DroneAddCmdBr, DroneChangeCmdBr, FighterAddCmdBr, FighterChangeCmdBr,
    FitAddCmdBr, FitChangeCmdBr, FitIdBr, FitInfoCmdBr, FitRemoveCmd, FleetAddCmdBr, FleetChangeCmdBr, FleetIdBr,
    FleetInfoCmdBr, FleetRemoveCmd, FwEffectAddCmd, FwEffectChangeCmd, ImplantAddCmd, ImplantChangeCmd, ItemIdBr,
    ItemInfoCmdBr, ItemRemoveCmd, ModuleAddCmdBr, ModuleChangeCmdBr, ProjEffectAddCmdBr, ProjEffectChangeCmdBr,
    RigAddCmd, RigChangeCmd, ServiceAddCmd, ServiceChangeCmd, ShipChangeCmd, ShipSetCmd, ShipUnsetCmd, SkillAddCmd,
    SkillChangeCmd, SolChangeCmd, SolChangeEnumCmd, SolChangeEnumCmdBr, SolInfoCmdBr, SolInfoEnumCmdBr,
    StanceChangeCmd, StanceSetCmd, StanceUnsetCmd, SubsystemAddCmd, SubsystemChangeCmd, SwEffectAddCmd,
    SwEffectChangeCmd,
    err::{BrResolveError, SolChangeEnumError, SolInfoEnumError},
    info::SolInfoEnumCmd,
    stats::{
        FitStatsCmdBr, FleetStatsCmdBr, ItemStatsCmdBr, SolStatsCmdBr, SolStatsEnumCmd, SolStatsEnumCmdBr,
        err::SolStatsEnumError,
    },
    svc::SolCtx,
    trial::{FitTryItemsCmdBr, SolTryItemsEnumCmd, SolTryItemsEnumCmdBr, err::SolTryItemsEnumError},
    val::{FitValCmdBr, SolValCmdBr, SolValEnumCmd, SolValEnumCmdBr, err::SolValEnumError},
};

#[derive(Clone)]
pub(crate) enum SolHybridCmd {
    Ctl(SolChangeEnumCmd),
    Info(SolInfoEnumCmd),
    Stats(SolStatsEnumCmd),
    Val(SolValEnumCmd),
    TryItems(SolTryItemsEnumCmd),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(untagged))]
#[derive(Clone)]
pub enum SolHybridCmdBr {
    Ctl(SolChangeEnumCmdBr),
    Info(SolInfoEnumCmdBr),
    Stats(SolStatsEnumCmdBr),
    Val(SolValEnumCmdBr),
    TryItems(SolTryItemsEnumCmdBr),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
// Solar system
impl SolChangeCmd {
    pub fn into_sol_hyb_br(self) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br())
    }
}
// Fleet
impl FleetAddCmdBr {
    pub fn into_sol_hyb_br(self) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br())
    }
}
impl FleetChangeCmdBr {
    pub fn into_sol_hyb_br(self, fleet_id: impl Into<FleetIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fleet_id))
    }
}
impl FleetRemoveCmd {
    pub fn into_sol_hyb_br(self, fleet_id: impl Into<FleetIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fleet_id))
    }
}
// Fit
impl FitAddCmdBr {
    pub fn into_sol_hyb_br(self) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br())
    }
}
impl FitChangeCmdBr {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl FitRemoveCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
// Item
impl ItemRemoveCmd {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
// Item - autocharge
impl AutochargeChangeCmd {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
// Item - booster
impl BoosterAddCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl BoosterChangeCmd {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
// Item - character
impl CharacterSetCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl CharacterChangeCmd {
    pub fn into_sol_hyb_br_via_fit(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br_via_fit(fit_id))
    }
    pub fn into_sol_hyb_br_via_item(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br_via_item(item_id))
    }
}
impl CharacterUnsetCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
// Item - charge
impl ChargeChangeCmd {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
// Item - drone
impl DroneAddCmdBr {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl DroneChangeCmdBr {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
// Item - fighter
impl FighterAddCmdBr {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl FighterChangeCmdBr {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
// Item - fit-wide effect
impl FwEffectAddCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl FwEffectChangeCmd {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
// Item - implant
impl ImplantAddCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl ImplantChangeCmd {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
// Item - module
impl ModuleAddCmdBr {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl ModuleChangeCmdBr {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
// Item - projected effect
impl ProjEffectAddCmdBr {
    pub fn into_sol_hyb_br(self) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br())
    }
}
impl ProjEffectChangeCmdBr {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
// Item - rig
impl RigAddCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl RigChangeCmd {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
// Item - service
impl ServiceAddCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl ServiceChangeCmd {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
// Item - ship
impl ShipSetCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl ShipChangeCmd {
    pub fn into_sol_hyb_br_via_fit(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br_via_fit(fit_id))
    }
    pub fn into_sol_hyb_br_via_item(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br_via_item(item_id))
    }
}
impl ShipUnsetCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
// Item - skill
impl SkillAddCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl SkillChangeCmd {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
// Item - stance
impl StanceSetCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl StanceChangeCmd {
    pub fn into_sol_hyb_br_via_fit(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br_via_fit(fit_id))
    }
    pub fn into_sol_hyb_br_via_item(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br_via_item(item_id))
    }
}
impl StanceUnsetCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
// Item - subsystem
impl SubsystemAddCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl SubsystemChangeCmd {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
// Item - system-wide effect
impl SwEffectAddCmd {
    pub fn into_sol_hyb_br(self) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br())
    }
}
impl SwEffectChangeCmd {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
// Info
impl SolInfoCmdBr {
    pub fn into_sol_hyb_br(self) -> SolHybridCmdBr {
        SolHybridCmdBr::Info(self.into_sol_info_br())
    }
}
impl FleetInfoCmdBr {
    pub fn into_sol_hyb_br(self, fleet_id: impl Into<FleetIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Info(self.into_sol_info_br(fleet_id))
    }
}
impl FitInfoCmdBr {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Info(self.into_sol_info_br(fit_id))
    }
}
impl ItemInfoCmdBr {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Info(self.into_sol_info_br(item_id))
    }
}
// Stats
impl SolStatsCmdBr {
    pub fn into_sol_hyb_br(self) -> SolHybridCmdBr {
        SolHybridCmdBr::Stats(self.into_sol_stats_br())
    }
}
impl FleetStatsCmdBr {
    pub fn into_sol_hyb_br(self, fleet_id: impl Into<FleetIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Stats(self.into_sol_stats_br(fleet_id))
    }
}
impl FitStatsCmdBr {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Stats(self.into_sol_stats_br(fit_id))
    }
}
impl ItemStatsCmdBr {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Stats(self.into_sol_stats_br(item_id))
    }
}
// Validation
impl SolValCmdBr {
    pub fn into_sol_hyb_br(self) -> SolHybridCmdBr {
        SolHybridCmdBr::Val(self.into_sol_val_br())
    }
}
impl FitValCmdBr {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Val(self.into_sol_val_br(fit_id))
    }
}
// Try items
impl FitTryItemsCmdBr {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::TryItems(self.into_sol_try_br(fit_id))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolHybridCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> Result<SolHybridCmd, BrResolveError> {
        Ok(match self {
            Self::Ctl(ctl_cmd) => SolHybridCmd::Ctl(ctl_cmd.br_resolve(resps)?),
            Self::Info(info_cmd) => SolHybridCmd::Info(info_cmd.br_resolve(resps)?),
            Self::Stats(stats_cmd) => SolHybridCmd::Stats(stats_cmd.br_resolve(resps)?),
            Self::Val(val_cmd) => SolHybridCmd::Val(val_cmd.br_resolve(resps)?),
            Self::TryItems(try_cmd) => SolHybridCmd::TryItems(try_cmd.br_resolve(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolHybridCmd {
    pub(crate) fn execute(self, ctx: SolCtx, core_sol: &mut rc::SolarSystem) -> Result<CmdResp, SolHybridError> {
        Ok(match self {
            Self::Ctl(ctl_cmd) => ctl_cmd.execute(core_sol)?,
            Self::Info(info_cmd) => info_cmd.execute(ctx, core_sol)?,
            Self::Stats(stats_cmd) => stats_cmd.execute(core_sol)?,
            Self::Val(val_cmd) => val_cmd.execute(core_sol)?,
            Self::TryItems(try_cmd) => try_cmd.execute(core_sol)?,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolHybridError {
    #[error(transparent)]
    Ctl(#[from] SolChangeEnumError),
    #[error(transparent)]
    Info(#[from] SolInfoEnumError),
    #[error(transparent)]
    Stats(#[from] SolStatsEnumError),
    #[error(transparent)]
    Val(#[from] SolValEnumError),
    #[error(transparent)]
    TryItems(#[from] SolTryItemsEnumError),
}
