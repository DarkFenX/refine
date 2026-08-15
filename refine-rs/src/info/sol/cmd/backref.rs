use crate::{
    CtlCmdResps, FitIdBackref, FitInfoMode, FitInfoModesBackref, FleetIdBackref, FleetInfoMode, FleetInfoModesBackref,
    ItemIdBackref, ItemInfoMode, ItemInfoModesBackref, SolInfo, SolInfoMode, SolarSystemId, SrcAlias,
    info::{FitInfoModesInt, FleetInfoModesInt, ItemInfoModesInt},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct SolInfoCmdBackref {
    #[cfg_attr(feature = "serde", serde(default))]
    sol: SolInfoMode = SolInfoMode::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    fleet: FleetInfoModesBackref = FleetInfoModesBackref::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    fit: FitInfoModesBackref = FitInfoModesBackref::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    item: ItemInfoModesBackref = ItemInfoModesBackref::default(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfoCmdBackref {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_sol(mut self, mode: SolInfoMode) -> Self {
        self.sol = mode;
        self
    }
    pub fn with_fleet_default(mut self, mode: FleetInfoMode) -> Self {
        self.fleet.default = mode;
        self
    }
    pub fn with_fleet_overrides(mut self, mode: FleetInfoMode, item_ids: impl Iterator<Item = FleetIdBackref>) -> Self {
        self.fleet.overrides.push((mode, item_ids.collect()));
        self
    }
    pub fn with_fit_default(mut self, mode: FitInfoMode) -> Self {
        self.fit.default = mode;
        self
    }
    pub fn with_fit_overrides(mut self, mode: FitInfoMode, item_ids: impl Iterator<Item = FitIdBackref>) -> Self {
        self.fit.overrides.push((mode, item_ids.collect()));
        self
    }
    pub fn with_item_default(mut self, mode: ItemInfoMode) -> Self {
        self.item.default = mode;
        self
    }
    pub fn with_item_overrides(mut self, mode: ItemInfoMode, item_ids: impl Iterator<Item = ItemIdBackref>) -> Self {
        self.item.overrides.push((mode, item_ids.collect()));
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfoCmdBackref {
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
            self.sol,
            &FleetInfoModesInt::from_pub_modes_backref(self.fleet, ctl_cmd_resps),
            &FitInfoModesInt::from_pub_modes_backref(self.fit, ctl_cmd_resps),
            &ItemInfoModesInt::from_pub_modes_backref(self.item, ctl_cmd_resps),
        )
    }
}
