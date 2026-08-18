use rc::Lender;

use crate::{
    DpsProfile, FitId, FitInfo, FitInfoMode, FleetId, FleetInfo, FleetInfoMode, ItemId, ItemInfoMode, ProjEffectInfo,
    SecZone, SolInfoMode, SolarSystemId, Spool, SwEffectInfo, shared::OverridableMap, src::SrcAlias,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct SolInfo {
    pub id: SolarSystemId,
    pub src_alias: SrcAlias,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<SolInfoExt>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct SolInfoExt {
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub fleets: Vec<FleetInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub fits: Vec<FitInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub sw_effects: Vec<SwEffectInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub proj_effects: Vec<ProjEffectInfo>,
    pub sec_zone: SecZone,
    pub default_spool: Spool,
    pub default_incoming_dps: DpsProfile,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfo {
    pub(in crate::info) fn from_ids_and_core(
        sol_id: SolarSystemId,
        src_alias: SrcAlias,
        core_sol: &mut rc::SolarSystem,
        sol_info_mode: SolInfoMode,
        fleet_info_modes: &OverridableMap<FleetId, FleetInfoMode>,
        fit_info_modes: &OverridableMap<FitId, FitInfoMode>,
        item_info_modes: &OverridableMap<ItemId, ItemInfoMode>,
    ) -> Self {
        Self {
            id: sol_id,
            src_alias,
            extended: SolInfoExt::try_from_core(
                core_sol,
                sol_info_mode,
                fleet_info_modes,
                fit_info_modes,
                item_info_modes,
            ),
        }
    }
    pub(crate) fn from_ids_and_ext(
        sol_id: SolarSystemId,
        src_alias: SrcAlias,
        sol_info_ext: Option<SolInfoExt>,
    ) -> Self {
        Self {
            id: sol_id,
            src_alias,
            extended: sol_info_ext,
        }
    }
}

impl SolInfoExt {
    pub(in crate::info) fn try_from_core(
        core_sol: &mut rc::SolarSystem,
        sol_info_mode: SolInfoMode,
        fleet_info_modes: &OverridableMap<FleetId, FleetInfoMode>,
        fit_info_modes: &OverridableMap<FitId, FitInfoMode>,
        item_info_modes: &OverridableMap<ItemId, ItemInfoMode>,
    ) -> Option<Self> {
        match sol_info_mode {
            SolInfoMode::Id => None,
            SolInfoMode::Full => Some(Self {
                fleets: core_sol
                    .iter_fleets_mut()
                    .map_into_iter(|mut core_fleet| FleetInfo::from_core(&mut core_fleet, fleet_info_modes))
                    .collect(),
                fits: core_sol
                    .iter_fits_mut()
                    .map_into_iter(|mut core_fit| FitInfo::from_core(&mut core_fit, fit_info_modes, item_info_modes))
                    .collect(),
                sw_effects: core_sol
                    .iter_sw_effects_mut()
                    .map_into_iter(|mut core_sw_effect| SwEffectInfo::from_core(&mut core_sw_effect, item_info_modes))
                    .collect(),
                proj_effects: core_sol
                    .iter_proj_effects_mut()
                    .map_into_iter(|mut proj_effect| ProjEffectInfo::from_core(&mut proj_effect, item_info_modes))
                    .collect(),
                sec_zone: core_sol.get_sec_zone(),
                default_spool: core_sol.get_default_spool(),
                default_incoming_dps: core_sol.get_default_incoming_dps(),
            }),
        }
    }
}
