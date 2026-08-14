use rc::Lender;

use crate::{
    DpsProfile, FitInfo, FleetInfo, ProjEffectInfo, SecZone, SolInfoMode, SolInfoModes, SolarSystemId, Spool,
    SwEffectInfo, src::SrcAlias,
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
    pub(crate) fn from_ids_and_core(
        sol_id: SolarSystemId,
        src_alias: SrcAlias,
        core_sol: &mut rc::SolarSystem,
        modes: SolInfoModes,
    ) -> Self {
        Self {
            id: sol_id,
            src_alias,
            extended: match modes.sol {
                SolInfoMode::Id => None,
                SolInfoMode::Full => SolInfoExt::try_from_core(core_sol, modes),
            },
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
    pub(crate) fn try_from_core(core_sol: &mut rc::SolarSystem, modes: SolInfoModes) -> Option<Self> {
        match modes.sol {
            SolInfoMode::Id => None,
            SolInfoMode::Full => Some(Self {
                fleets: core_sol
                    .iter_fleets_mut()
                    .map_into_iter(|mut core_fleet| FleetInfo::from_core(&mut core_fleet, modes.get_fleet_modes()))
                    .collect(),
                fits: core_sol
                    .iter_fits_mut()
                    .map_into_iter(|mut core_fit| FitInfo::from_core(&mut core_fit, modes.get_fit_modes()))
                    .collect(),
                sw_effects: core_sol
                    .iter_sw_effects_mut()
                    .map_into_iter(|mut core_sw_effect| {
                        SwEffectInfo::from_core(&mut core_sw_effect, modes.get_item_modes())
                    })
                    .collect(),
                proj_effects: core_sol
                    .iter_proj_effects_mut()
                    .map_into_iter(|mut proj_effect| {
                        ProjEffectInfo::from_core(&mut proj_effect, modes.get_item_modes())
                    })
                    .collect(),
                sec_zone: core_sol.get_sec_zone(),
                default_spool: core_sol.get_default_spool(),
                default_incoming_dps: core_sol.get_default_incoming_dps(),
            }),
        }
    }
}
