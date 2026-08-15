use rc::Lender;

use crate::{
    DpsProfile, FitInfo, FleetInfo, ProjEffectInfo, SecZone, SolInfoArgs, SolInfoMode, SolarSystemId, Spool,
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
        info_args: SolInfoArgs,
    ) -> Self {
        Self {
            id: sol_id,
            src_alias,
            extended: match info_args.sol {
                SolInfoMode::Id => None,
                SolInfoMode::Full => SolInfoExt::try_from_core(core_sol, info_args),
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
    pub(crate) fn try_from_core(core_sol: &mut rc::SolarSystem, info_args: SolInfoArgs) -> Option<Self> {
        match info_args.sol {
            SolInfoMode::Id => None,
            SolInfoMode::Full => Some(Self {
                fleets: core_sol
                    .iter_fleets_mut()
                    .map_into_iter(|mut core_fleet| FleetInfo::from_core(&mut core_fleet, info_args.get_fleet_args()))
                    .collect(),
                fits: core_sol
                    .iter_fits_mut()
                    .map_into_iter(|mut core_fit| FitInfo::from_core(&mut core_fit, info_args.get_fit_args()))
                    .collect(),
                sw_effects: core_sol
                    .iter_sw_effects_mut()
                    .map_into_iter(|mut core_sw_effect| {
                        SwEffectInfo::from_core(&mut core_sw_effect, info_args.get_item_args())
                    })
                    .collect(),
                proj_effects: core_sol
                    .iter_proj_effects_mut()
                    .map_into_iter(|mut proj_effect| {
                        ProjEffectInfo::from_core(&mut proj_effect, info_args.get_item_args())
                    })
                    .collect(),
                sec_zone: core_sol.get_sec_zone(),
                default_spool: core_sol.get_default_spool(),
                default_incoming_dps: core_sol.get_default_incoming_dps(),
            }),
        }
    }
}
