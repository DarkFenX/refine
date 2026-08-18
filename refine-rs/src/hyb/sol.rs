use crate::{
    AutochargeChangeCmd, BoosterAddCmd, BoosterChangeCmd, CharacterChangeCmd, CharacterSetCmd, CharacterUnsetCmd,
    ChargeChangeCmd, CmdResp, CmdResps, DroneAddCmd, DroneAddCmdBr, DroneChangeCmd, DroneChangeCmdBr, FighterAddCmd,
    FighterAddCmdBr, FighterChangeCmd, FighterChangeCmdBr, FitAddCmd, FitAddCmdBr, FitChangeCmd, FitChangeCmdBr,
    FitIdBr, FitInfoCmd, FitInfoCmdBr, FitRemoveCmd, FleetAddCmd, FleetAddCmdBr, FleetChangeCmd, FleetChangeCmdBr,
    FleetIdBr, FleetInfoCmd, FleetRemoveCmd, FwEffectAddCmd, FwEffectChangeCmd, ImplantAddCmd, ImplantChangeCmd,
    ItemIdBr, ItemInfoCmd, ItemInfoCmdBr, ItemRemoveCmd, ModuleAddCmd, ModuleAddCmdBr, ModuleChangeCmd,
    ModuleChangeCmdBr, ProjEffectAddCmd, ProjEffectAddCmdBr, ProjEffectChangeCmd, ProjEffectChangeCmdBr, RigAddCmd,
    RigChangeCmd, ServiceAddCmd, ServiceChangeCmd, ShipChangeCmd, ShipSetCmd, ShipUnsetCmd, SkillAddCmd,
    SkillChangeCmd, SolChangeCmd, SolChangeEnumCmd, SolChangeEnumCmdBr, SolInfoCmd, SolInfoCmdBr, SolInfoEnumCmdBr,
    StanceChangeCmd, StanceSetCmd, StanceUnsetCmd, SubsystemAddCmd, SubsystemChangeCmd, SwEffectAddCmd,
    SwEffectChangeCmd,
    err::{BrResolveError, SolChangeEnumError, SolInfoEnumError},
    info::SolInfoEnumCmd,
    svc::SolCtx,
};

#[derive(Clone)]
pub(crate) enum SolHybridCmd {
    Ctl(SolChangeEnumCmd),
    Info(SolInfoEnumCmd),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(untagged))]
#[derive(Clone)]
pub enum SolHybridCmdBr {
    Ctl(SolChangeEnumCmdBr),
    Info(SolInfoEnumCmdBr),
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
impl FleetAddCmd {
    pub fn into_sol_hyb_br(self) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br())
    }
}
impl FleetAddCmdBr {
    pub fn into_sol_hyb_br(self) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br())
    }
}
impl FleetChangeCmd {
    pub fn into_sol_hyb_br(self, fleet_id: impl Into<FleetIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fleet_id))
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
impl FitAddCmd {
    pub fn into_sol_hyb_br(self) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br())
    }
}
impl FitAddCmdBr {
    pub fn into_sol_hyb_br(self) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br())
    }
}
impl FitChangeCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
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
impl DroneAddCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl DroneAddCmdBr {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl DroneChangeCmd {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
impl DroneChangeCmdBr {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
// Item - fighter
impl FighterAddCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl FighterAddCmdBr {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl FighterChangeCmd {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
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
impl ModuleAddCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl ModuleAddCmdBr {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(fit_id))
    }
}
impl ModuleChangeCmd {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
impl ModuleChangeCmdBr {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
    }
}
// Item - projected effect
impl ProjEffectAddCmd {
    pub fn into_sol_hyb_br(self) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br())
    }
}
impl ProjEffectAddCmdBr {
    pub fn into_sol_hyb_br(self) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br())
    }
}
impl ProjEffectChangeCmd {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Ctl(self.into_sol_ctl_br(item_id))
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
// Info - solar system
impl SolInfoCmd {
    pub fn into_sol_hyb_br(self) -> SolHybridCmdBr {
        SolHybridCmdBr::Info(self.into_sol_inf_br())
    }
}
impl SolInfoCmdBr {
    pub fn into_sol_hyb_br(self) -> SolHybridCmdBr {
        SolHybridCmdBr::Info(self.into_sol_inf_br())
    }
}
// Info - fleet
impl FleetInfoCmd {
    pub fn into_sol_hyb_br(self, fleet_id: impl Into<FleetIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Info(self.into_sol_inf_br(fleet_id))
    }
}
// Info - fit
impl FitInfoCmd {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Info(self.into_sol_inf_br(fit_id))
    }
}
impl FitInfoCmdBr {
    pub fn into_sol_hyb_br(self, fit_id: impl Into<FitIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Info(self.into_sol_inf_br(fit_id))
    }
}
// Info - item
impl ItemInfoCmd {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Info(self.into_sol_inf_br(item_id))
    }
}
impl ItemInfoCmdBr {
    pub fn into_sol_hyb_br(self, item_id: impl Into<ItemIdBr>) -> SolHybridCmdBr {
        SolHybridCmdBr::Info(self.into_sol_inf_br(item_id))
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
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SolHybridError {
    #[error(transparent)]
    Ctl(#[from] SolChangeEnumError),
    #[error(transparent)]
    Info(#[from] SolInfoEnumError),
}
