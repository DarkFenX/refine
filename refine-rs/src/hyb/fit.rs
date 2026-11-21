use crate::{
    AutochargeChangeCmd, BoosterAddCmd, BoosterChangeCmd, CharacterChangeCmd, CharacterSetCmd, CharacterUnsetCmd,
    ChargeChangeCmd, CmdResp, CmdResps, DroneAddCmdBr, DroneChangeCmdBr, FighterAddCmdBr, FighterChangeCmdBr,
    FitChangeCmd, FitChangeEnumCmd, FitChangeEnumCmdBr, FitInfoCmdBr, FitInfoEnumCmdBr, FwEffectAddCmd,
    FwEffectChangeCmd, ImplantAddCmd, ImplantChangeCmd, ItemAutodetectAddCmd, ItemIdBr, ItemInfoCmdBr, ItemRemoveCmd,
    ModuleAddCmdBr, ModuleChangeCmdBr, RigAddCmd, RigChangeCmd, ServiceAddCmd, ServiceChangeCmd, ShipChangeCmd,
    ShipSetCmd, ShipUnsetCmd, SkillAddCmd, SkillChangeCmd, StanceChangeCmd, StanceSetCmd, StanceUnsetCmd,
    SubsystemAddCmd, SubsystemChangeCmd,
    err::{BrResolveError, FitChangeEnumError, FitInfoEnumError},
    info::FitInfoEnumCmd,
    stats::{FitStatsCmdBr, FitStatsEnumCmd, FitStatsEnumCmdBr, ItemStatsCmdBr, err::FitStatsEnumError},
    trial::{FitTryItemsCmdBr, FitTryItemsEnumCmd, FitTryItemsEnumCmdBr},
    val::{FitValCmdBr, FitValEnumCmd, FitValEnumCmdBr},
};

#[derive(Clone)]
pub(crate) enum FitHybridCmd {
    Ctl(FitChangeEnumCmd),
    Info(FitInfoEnumCmd),
    Stats(FitStatsEnumCmd),
    Val(FitValEnumCmd),
    TryItems(FitTryItemsEnumCmd),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(untagged))]
#[derive(Clone)]
pub enum FitHybridCmdBr {
    Ctl(FitChangeEnumCmdBr),
    Info(FitInfoEnumCmdBr),
    Stats(FitStatsEnumCmdBr),
    Val(FitValEnumCmdBr),
    TryItems(FitTryItemsEnumCmdBr),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
// Fit
impl FitChangeCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
// Item
impl ItemAutodetectAddCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl ItemRemoveCmd {
    pub const fn into_fit_hyb_br(self, item_id: impl [const] Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - autocharge
impl AutochargeChangeCmd {
    pub const fn into_fit_hyb_br(self, item_id: impl [const] Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - booster
impl BoosterAddCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl BoosterChangeCmd {
    pub const fn into_fit_hyb_br(self, item_id: impl [const] Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - character
impl CharacterSetCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl CharacterChangeCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl CharacterUnsetCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
// Item - charge
impl ChargeChangeCmd {
    pub const fn into_fit_hyb_br(self, item_id: impl [const] Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - drone
impl DroneAddCmdBr {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl DroneChangeCmdBr {
    pub const fn into_fit_hyb_br(self, item_id: impl [const] Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - fighter
impl FighterAddCmdBr {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl FighterChangeCmdBr {
    pub const fn into_fit_hyb_br(self, item_id: impl [const] Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - fit-wide effect
impl FwEffectAddCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl FwEffectChangeCmd {
    pub const fn into_fit_hyb_br(self, item_id: impl [const] Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - implant
impl ImplantAddCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl ImplantChangeCmd {
    pub const fn into_fit_hyb_br(self, item_id: impl [const] Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - module
impl ModuleAddCmdBr {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl ModuleChangeCmdBr {
    pub const fn into_fit_hyb_br(self, item_id: impl [const] Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - rig
impl RigAddCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl RigChangeCmd {
    pub const fn into_fit_hyb_br(self, item_id: impl [const] Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - service
impl ServiceAddCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl ServiceChangeCmd {
    pub const fn into_fit_hyb_br(self, item_id: impl [const] Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - ship
impl ShipSetCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl ShipChangeCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl ShipUnsetCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
// Item - skill
impl SkillAddCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl SkillChangeCmd {
    pub const fn into_fit_hyb_br(self, item_id: impl [const] Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - stance
impl StanceSetCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl StanceChangeCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl StanceUnsetCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
// Item - subsystem
impl SubsystemAddCmd {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl SubsystemChangeCmd {
    pub const fn into_fit_hyb_br(self, item_id: impl [const] Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Info
impl FitInfoCmdBr {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Info(self.into_fit_info_br())
    }
}
impl ItemInfoCmdBr {
    pub const fn into_fit_hyb_br(self, item_id: impl [const] Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Info(self.into_fit_info_br(item_id))
    }
}
// Stats
impl FitStatsCmdBr {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Stats(self.into_fit_stats_br())
    }
}
impl ItemStatsCmdBr {
    pub const fn into_fit_hyb_br(self, item_id: impl [const] Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Stats(self.into_fit_stats_br(item_id))
    }
}
// Validation
impl FitValCmdBr {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Val(self.into_fit_val_br())
    }
}
// Try items
impl FitTryItemsCmdBr {
    pub const fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::TryItems(self.into_fit_try_br())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitHybridCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> Result<FitHybridCmd, BrResolveError> {
        Ok(match self {
            Self::Ctl(ctl_cmd) => FitHybridCmd::Ctl(ctl_cmd.br_resolve(resps)?),
            Self::Info(info_cmd) => FitHybridCmd::Info(info_cmd.br_resolve(resps)?),
            Self::Stats(stats_cmd) => FitHybridCmd::Stats(stats_cmd.br_resolve(resps)?),
            Self::Val(val_cmd) => FitHybridCmd::Val(val_cmd.br_resolve(resps)),
            Self::TryItems(try_cmd) => FitHybridCmd::TryItems(try_cmd.br_resolve(resps)),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FitHybridCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::Fit) -> Result<CmdResp, FitHybridError> {
        Ok(match self {
            Self::Ctl(ctl_cmd) => ctl_cmd.execute(core_fit)?,
            Self::Info(info_cmd) => info_cmd.execute(core_fit)?,
            Self::Stats(stats_cmd) => stats_cmd.execute(core_fit)?,
            Self::Val(val_cmd) => val_cmd.execute(core_fit),
            Self::TryItems(try_cmd) => try_cmd.execute(core_fit),
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitHybridError {
    #[error(transparent)]
    Ctl(#[from] FitChangeEnumError),
    #[error(transparent)]
    Info(#[from] FitInfoEnumError),
    #[error(transparent)]
    Stats(#[from] FitStatsEnumError),
}
