use crate::{
    CtlCmdResps, FitId, FitIdBr, FitInfoMode, FleetId, FleetIdBr, FleetInfoMode, ItemId, ItemIdBr, ItemInfoMode,
    SolInfo, SolInfoExt, SolInfoMode, SolarSystemId, SrcAlias,
    info::{InfoModes, InfoModesInt},
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct SolInfoCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet: InfoModes<FleetInfoMode, FleetId>,
    #[cfg_attr(feature = "serde", serde(default))]
    fit: InfoModes<FitInfoMode, FitId>,
    #[cfg_attr(feature = "serde", serde(default))]
    item: InfoModes<ItemInfoMode, ItemId>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: SolInfoCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct SolInfoCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    fleet: InfoModes<FleetInfoMode, FleetIdBr>,
    #[cfg_attr(feature = "serde", serde(default))]
    fit: InfoModes<FitInfoMode, FitIdBr>,
    #[cfg_attr(feature = "serde", serde(default))]
    item: InfoModes<ItemInfoMode, ItemIdBr>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: SolInfoCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
struct SolInfoCmdShared {
    #[cfg_attr(feature = "serde", serde(default))]
    sol: SolInfoMode,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfoCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_sol(mut self, mode: SolInfoMode) -> Self {
        self.shared.sol = mode;
        self
    }
    pub fn with_fleet_default(mut self, mode: FleetInfoMode) -> Self {
        self.fleet.default = mode;
        self
    }
    pub fn with_fleet_overrides(mut self, mode: FleetInfoMode, item_ids: impl Iterator<Item = FleetId>) -> Self {
        self.fleet.overrides.push((mode, item_ids.collect()));
        self
    }
    pub fn with_fit_default(mut self, mode: FitInfoMode) -> Self {
        self.fit.default = mode;
        self
    }
    pub fn with_fit_overrides(mut self, mode: FitInfoMode, item_ids: impl Iterator<Item = FitId>) -> Self {
        self.fit.overrides.push((mode, item_ids.collect()));
        self
    }
    pub fn with_item_default(mut self, mode: ItemInfoMode) -> Self {
        self.item.default = mode;
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.item.overrides.push((mode, item_ids.collect()));
        self
    }
}

impl SolInfoCmdBr {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_sol(mut self, mode: SolInfoMode) -> Self {
        self.shared.sol = mode;
        self
    }
    pub fn with_fleet_default(mut self, mode: FleetInfoMode) -> Self {
        self.fleet.default = mode;
        self
    }
    pub fn with_fleet_overrides(mut self, mode: FleetInfoMode, item_ids: impl Iterator<Item = FleetIdBr>) -> Self {
        self.fleet.overrides.push((mode, item_ids.collect()));
        self
    }
    pub fn with_fit_default(mut self, mode: FitInfoMode) -> Self {
        self.fit.default = mode;
        self
    }
    pub fn with_fit_overrides(mut self, mode: FitInfoMode, item_ids: impl Iterator<Item = FitIdBr>) -> Self {
        self.fit.overrides.push((mode, item_ids.collect()));
        self
    }
    pub fn with_item_default(mut self, mode: ItemInfoMode) -> Self {
        self.item.default = mode;
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl Iterator<Item = ItemIdBr>) -> Self {
        self.item.overrides.push((mode, item_ids.collect()));
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfoCmd {
    pub(crate) fn execute_into_info(
        self,
        sol_id: SolarSystemId,
        src_alias: SrcAlias,
        core_sol: &mut rc::SolarSystem,
    ) -> SolInfo {
        SolInfo::from_ids_and_core(
            sol_id,
            src_alias,
            core_sol,
            self.shared.sol,
            &InfoModesInt::from_pub_modes(self.fleet),
            &InfoModesInt::from_pub_modes(self.fit),
            &InfoModesInt::from_pub_modes(self.item),
        )
    }

    pub(crate) fn execute_into_info_ext(self, core_sol: &mut rc::SolarSystem) -> Option<SolInfoExt> {
        SolInfoExt::try_from_core(
            core_sol,
            self.shared.sol,
            &InfoModesInt::from_pub_modes(self.fleet),
            &InfoModesInt::from_pub_modes(self.fit),
            &InfoModesInt::from_pub_modes(self.item),
        )
    }
}

impl SolInfoCmdBr {
    pub(crate) fn execute(
        self,
        sol_id: SolarSystemId,
        src_alias: SrcAlias,
        core_sol: &mut rc::SolarSystem,
        ctl_cmd_resps: &CtlCmdResps,
    ) -> SolInfo {
        SolInfo::from_ids_and_core(
            sol_id,
            src_alias,
            core_sol,
            self.shared.sol,
            &InfoModesInt::from_pub_modes_br(self.fleet, ctl_cmd_resps),
            &InfoModesInt::from_pub_modes_br(self.fit, ctl_cmd_resps),
            &InfoModesInt::from_pub_modes_br(self.item, ctl_cmd_resps),
        )
    }
}
