use crate::{
    AutochargeChangeCmd, BoosterAddCmd, BoosterChangeCmd, CharacterChangeCmd, CharacterSetCmd, CharacterUnsetCmd,
    ChargeChangeCmd, CmdResp, CmdResps, DroneAddCmdBr, DroneChangeCmdBr, FighterAddCmdBr, FighterChangeCmdBr,
    FitChangeCmd, FitChangeEnumCmd, FitChangeEnumCmdBr, FitInfoCmdBr, FitInfoEnumCmdBr, FwEffectAddCmd,
    FwEffectChangeCmd, ImplantAddCmd, ImplantChangeCmd, ItemIdBr, ItemInfoCmdBr, ItemRemoveCmd, ModuleAddCmdBr,
    ModuleChangeCmdBr, RigAddCmd, RigChangeCmd, ServiceAddCmd, ServiceChangeCmd, ShipChangeCmd, ShipSetCmd,
    ShipUnsetCmd, SkillAddCmd, SkillChangeCmd, StanceChangeCmd, StanceSetCmd, StanceUnsetCmd, SubsystemAddCmd,
    SubsystemChangeCmd,
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
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
// Item
impl ItemRemoveCmd {
    pub fn into_fit_hyb_br(self, item_id: impl Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - autocharge
impl AutochargeChangeCmd {
    pub fn into_fit_hyb_br(self, item_id: impl Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - booster
impl BoosterAddCmd {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl BoosterChangeCmd {
    pub fn into_fit_hyb_br(self, item_id: impl Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - character
impl CharacterSetCmd {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl CharacterChangeCmd {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl CharacterUnsetCmd {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
// Item - charge
impl ChargeChangeCmd {
    pub fn into_fit_hyb_br(self, item_id: impl Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - drone
impl DroneAddCmdBr {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl DroneChangeCmdBr {
    pub fn into_fit_hyb_br(self, item_id: impl Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - fighter
impl FighterAddCmdBr {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl FighterChangeCmdBr {
    pub fn into_fit_hyb_br(self, item_id: impl Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - fit-wide effect
impl FwEffectAddCmd {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl FwEffectChangeCmd {
    pub fn into_fit_hyb_br(self, item_id: impl Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - implant
impl ImplantAddCmd {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl ImplantChangeCmd {
    pub fn into_fit_hyb_br(self, item_id: impl Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - module
impl ModuleAddCmdBr {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl ModuleChangeCmdBr {
    pub fn into_fit_hyb_br(self, item_id: impl Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - rig
impl RigAddCmd {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl RigChangeCmd {
    pub fn into_fit_hyb_br(self, item_id: impl Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - service
impl ServiceAddCmd {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl ServiceChangeCmd {
    pub fn into_fit_hyb_br(self, item_id: impl Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - ship
impl ShipSetCmd {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl ShipChangeCmd {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl ShipUnsetCmd {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
// Item - skill
impl SkillAddCmd {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl SkillChangeCmd {
    pub fn into_fit_hyb_br(self, item_id: impl Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Item - stance
impl StanceSetCmd {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl StanceChangeCmd {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl StanceUnsetCmd {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
// Item - subsystem
impl SubsystemAddCmd {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br())
    }
}
impl SubsystemChangeCmd {
    pub fn into_fit_hyb_br(self, item_id: impl Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Ctl(self.into_fit_ctl_br(item_id))
    }
}
// Info
impl FitInfoCmdBr {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Info(self.into_fit_info_br())
    }
}
impl ItemInfoCmdBr {
    pub fn into_fit_hyb_br(self, item_id: impl Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Info(self.into_fit_info_br(item_id))
    }
}
// Stats
impl FitStatsCmdBr {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Stats(self.into_fit_stats_br())
    }
}
impl ItemStatsCmdBr {
    pub fn into_fit_hyb_br(self, item_id: impl Into<ItemIdBr>) -> FitHybridCmdBr {
        FitHybridCmdBr::Stats(self.into_fit_stats_br(item_id))
    }
}
// Validation
impl FitValCmdBr {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
        FitHybridCmdBr::Val(self.into_fit_val_br())
    }
}
// Try items
impl FitTryItemsCmdBr {
    pub fn into_fit_hyb_br(self) -> FitHybridCmdBr {
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
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> Result<CmdResp, FitHybridError> {
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
