use rc::Lender;

use crate::{
    DpsProfile, FitId, FitInfo, FitInfoMode, FleetId, FleetInfo, FleetInfoMode, ItemId, ItemInfoMode, ProjEffectInfo,
    SecZone, SolId, SolInfoMode, Spool, SrcAlias, SwEffectInfo, shared::OvrdMapLight,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct SolInfo {
    pub id: SolId,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    pub extended: Option<SolInfoExt>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct SolInfoExt {
    pub src_alias: SrcAlias,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub fleets: Vec<FleetInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub fits: Vec<FitInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub sw_effects: Vec<SwEffectInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub proj_effects: Vec<ProjEffectInfo>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub sec_zone: Option<SecZone>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub default_spool: Option<Spool>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub default_incoming_dps: Option<DpsProfile>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolInfo {
    pub(in crate::info::sol) fn from_ids_and_core(
        sol_id: SolId,
        src_alias: SrcAlias,
        core_sol: &mut rc::Sol,
        sol_info_mode: SolInfoMode,
        fleet_info_modes: &OvrdMapLight<FleetId, FleetInfoMode>,
        fit_info_modes: &OvrdMapLight<FitId, FitInfoMode>,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Self {
        Self {
            id: sol_id,
            extended: SolInfoExt::try_from_core(
                src_alias,
                core_sol,
                sol_info_mode,
                fleet_info_modes,
                fit_info_modes,
                item_info_modes,
            ),
        }
    }
    pub(crate) fn from_id_and_ext(sol_id: SolId, sol_info_ext: Option<SolInfoExt>) -> Self {
        Self {
            id: sol_id,
            extended: sol_info_ext,
        }
    }
}

impl SolInfoExt {
    pub(crate) fn from_src_alias(src_alias: SrcAlias) -> Self {
        Self {
            src_alias,
            fleets: Vec::new(),
            fits: Vec::new(),
            sw_effects: Vec::new(),
            proj_effects: Vec::new(),
            sec_zone: None,
            default_spool: None,
            default_incoming_dps: None,
        }
    }
    pub(in crate::info) fn try_from_core(
        src_alias: SrcAlias,
        core_sol: &mut rc::Sol,
        sol_info_mode: SolInfoMode,
        fleet_info_modes: &OvrdMapLight<FleetId, FleetInfoMode>,
        fit_info_modes: &OvrdMapLight<FitId, FitInfoMode>,
        item_info_modes: &OvrdMapLight<ItemId, ItemInfoMode>,
    ) -> Option<Self> {
        match sol_info_mode {
            SolInfoMode::Id => None,
            SolInfoMode::Partial => Some(Self::from_src_alias(src_alias)),
            SolInfoMode::Full => Some(Self {
                src_alias,
                // Force fits in fleets to use ID mode regardless of passed preference. The
                // preference will be used for fits themselves
                fleets: core_sol
                    .iter_fleets()
                    .map_into_iter(|mut core_fleet| {
                        FleetInfo::from_core(
                            &mut core_fleet,
                            fleet_info_modes,
                            &OvrdMapLight::from_default(FitInfoMode::Id),
                            &OvrdMapLight::from_default(ItemInfoMode::Id),
                        )
                    })
                    .collect(),
                fits: core_sol
                    .iter_fits()
                    .map_into_iter(|mut core_fit| FitInfo::from_core(&mut core_fit, fit_info_modes, item_info_modes))
                    .collect(),
                sw_effects: core_sol
                    .iter_sw_effects()
                    .map_into_iter(|mut core_sw_effect| SwEffectInfo::from_core(&mut core_sw_effect, item_info_modes))
                    .collect(),
                proj_effects: core_sol
                    .iter_proj_effects()
                    .map_into_iter(|mut proj_effect| ProjEffectInfo::from_core(&mut proj_effect, item_info_modes))
                    .collect(),
                sec_zone: Some(core_sol.get_sec_zone()),
                default_spool: Some(core_sol.get_default_spool()),
                default_incoming_dps: Some(core_sol.get_default_incoming_dps()),
            }),
        }
    }
}
