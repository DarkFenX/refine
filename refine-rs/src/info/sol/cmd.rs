use crate::{
    CmdResp, CmdResps, FitIdBr, FitInfoCmd, FitInfoCmdBr, FleetIdBr, FleetInfoCmd, ItemIdBr, ItemInfoCmd,
    ItemInfoCmdBr, SolInfoCmd, SolInfoCmdBr,
    err::BrResolveError,
    info::cmd_core::{
        FitInfoCmdCtxFit, FitInfoCmdCtxFitBr, FleetInfoCmdCtxFleet, FleetInfoCmdCtxFleetBr, ItemInfoCmdCtxItem,
        ItemInfoCmdCtxItemBr,
    },
};

#[derive(Clone)]
pub(crate) enum SolInfoEnumCmd {
    SolInfo(SolInfoCmd),
    FleetInfo(FleetInfoCmdCtxFleet),
    FitInfo(FitInfoCmdCtxFit),
    ItemInfo(ItemInfoCmdCtxItem),
}

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(tag = "type", rename_all = "snake_case")
)]
#[derive(Clone)]
pub enum SolInfoEnumCmdBr {
    SolInfo(SolInfoCmdBr),
    FleetInfo(FleetInfoCmdCtxFleetBr),
    FitInfo(FitInfoCmdCtxFitBr),
    ItemInfo(ItemInfoCmdCtxItemBr),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfoCmd {
    pub fn into_sol_inf_br(self) -> SolInfoEnumCmdBr {
        SolInfoEnumCmdBr::SolInfo(self.into_br())
    }
}
impl SolInfoCmdBr {
    pub fn into_sol_inf_br(self) -> SolInfoEnumCmdBr {
        SolInfoEnumCmdBr::SolInfo(self)
    }
}
impl FleetInfoCmd {
    pub fn into_sol_inf_br(self, fleet_id: impl Into<FleetIdBr>) -> SolInfoEnumCmdBr {
        SolInfoEnumCmdBr::FleetInfo(self.into_ctx_item_br(fleet_id))
    }
}
impl FitInfoCmd {
    pub fn into_sol_inf_br(self, fit_id: impl Into<FitIdBr>) -> SolInfoEnumCmdBr {
        SolInfoEnumCmdBr::FitInfo(self.into_ctx_item_br(fit_id))
    }
}
impl FitInfoCmdBr {
    pub fn into_sol_inf_br(self, fit_id: impl Into<FitIdBr>) -> SolInfoEnumCmdBr {
        SolInfoEnumCmdBr::FitInfo(self.into_ctx_item_br(fit_id))
    }
}
impl ItemInfoCmd {
    pub fn into_sol_inf_br(self, item_id: impl Into<ItemIdBr>) -> SolInfoEnumCmdBr {
        SolInfoEnumCmdBr::ItemInfo(self.into_ctx_item_br(item_id))
    }
}
impl ItemInfoCmdBr {
    pub fn into_sol_inf_br(self, item_id: impl Into<ItemIdBr>) -> SolInfoEnumCmdBr {
        SolInfoEnumCmdBr::ItemInfo(self.into_ctx_item_br(item_id))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfoEnumCmdBr {
    pub(crate) fn br_resolve(self, resps: &CmdResps) -> Result<SolInfoEnumCmd, BrResolveError> {
        Ok(match self {
            Self::SolInfo(cmd) => SolInfoEnumCmd::SolInfo(cmd.br_resolve(resps)?),
            Self::FleetInfo(cmd) => SolInfoEnumCmd::FleetInfo(cmd.br_resolve(resps)?),
            Self::FitInfo(cmd) => SolInfoEnumCmd::FitInfo(cmd.br_resolve(resps)?),
            Self::ItemInfo(cmd) => SolInfoEnumCmd::ItemInfo(cmd.br_resolve(resps)?),
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
// impl SolInfoEnumCmd {
//     pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<CmdResp,
// SolInfoEnumError> {         Ok(match self {
//             Self::SolInfo(cmd) => cmd.execute(core_sol).into(),
//             Self::FleetInfo(cmd) => cmd.execute(core_sol).into(),
//             Self::FitInfo(cmd) => cmd.execute(core_sol).into(),
//             Self::ItemInfo(cmd) => cmd.execute(core_sol).into(),
//         })
//     }
// }

#[derive(thiserror::Error, Debug)]
pub enum SolInfoEnumError {}
