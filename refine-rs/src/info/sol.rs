use rc::Lender;

use crate::{
    info::{FitInfo, FitInfoMode, FleetInfo, FleetInfoMode, ItemInfoMode, ProjEffectInfo, SolInfoMode, SwEffectInfo},
    sol::SolarSystemId,
};

pub struct SolInfo {
    pub id: SolarSystemId,
    pub extended: Option<SolInfoExt>,
}

pub struct SolInfoExt {
    pub fleets: Vec<FleetInfo>,
    pub fits: Vec<FitInfo>,
    pub sw_effects: Vec<SwEffectInfo>,
    pub proj_effects: Vec<ProjEffectInfo>,
    pub sec_zone: rc::SecZone,
    pub default_spool: rc::Spool,
    pub default_incoming_dps: rc::DpsProfile,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfo {
    pub(crate) fn from_id_and_core(
        sol_id: SolarSystemId,
        core_sol: &mut rc::SolarSystem,
        sol_mode: SolInfoMode,
        fleet_mode: FleetInfoMode,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> Self {
        Self {
            id: sol_id,
            extended: match sol_mode {
                SolInfoMode::Id => None,
                SolInfoMode::Full => SolInfoExt::try_from_core(core_sol, sol_mode, fleet_mode, fit_mode, item_mode),
            },
        }
    }
    pub(crate) fn from_id_and_ext(sol_id: SolarSystemId, sol_info_ext: Option<SolInfoExt>) -> Self {
        Self {
            id: sol_id,
            extended: sol_info_ext,
        }
    }
}

impl SolInfoExt {
    pub(crate) fn try_from_core(
        core_sol: &mut rc::SolarSystem,
        sol_mode: SolInfoMode,
        fleet_mode: FleetInfoMode,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> Option<Self> {
        match sol_mode {
            SolInfoMode::Id => None,
            SolInfoMode::Full => Some(Self {
                fleets: core_sol
                    .iter_fleets_mut()
                    .map_into_iter(|mut core_fleet| FleetInfo::from_core(&mut core_fleet, fleet_mode))
                    .collect(),
                fits: core_sol
                    .iter_fits_mut()
                    .map_into_iter(|mut core_fit| FitInfo::from_core(&mut core_fit, fit_mode, item_mode))
                    .collect(),
                sw_effects: core_sol
                    .iter_sw_effects_mut()
                    .map_into_iter(|mut core_sw_effect| SwEffectInfo::from_core(&mut core_sw_effect, item_mode))
                    .collect(),
                proj_effects: core_sol
                    .iter_proj_effects_mut()
                    .map_into_iter(|mut proj_effect| ProjEffectInfo::from_core(&mut proj_effect, item_mode))
                    .collect(),
                sec_zone: core_sol.get_sec_zone(),
                default_spool: core_sol.get_default_spool(),
                default_incoming_dps: core_sol.get_default_incoming_dps(),
            }),
        }
    }
}
