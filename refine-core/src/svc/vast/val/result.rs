use crate::{
    FitId,
    val::{
        ValActivationBlockedFail, ValCapitalModFail, ValChargeGroupFail, ValChargeParentGroupFail, ValChargeSizeFail,
        ValChargeVolumeFail, ValCloakingBlockedFail, ValDroneGroupFail, ValEffectSecZoneFail, ValEffectStopperFail,
        ValFighterSquadSizeFail, ValItemKindFail, ValItemSecZoneFail, ValItemVsShipKindFail, ValMaxGroupFail,
        ValMaxTypeFail, ValModuleStateFail, ValNotLoadedItemFail, ValOverloadSkillFail, ValProjFilterFail,
        ValProjImmunityFail, ValResourceFail, ValRigSizeFail, ValShipLimitFail, ValShipStanceFail, ValSlotCountFail,
        ValSlotIndexFail, ValSrqFail, ValUnusableCapFail, ValUnusableResFail, ValUnusableSlotFail,
    },
};

/// Validation result for a solar system.
///
/// Contains per-fit failures, and failures for items not belonging to any fit.
#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct ValResultSol {
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub fits: Vec<(FitId, ValResultFit)>,
    /// Not loaded stand-alone items - i.e. items which do not belong to any fits.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub not_loaded_item: Option<ValNotLoadedItemFail>,
}
impl ValResultSol {
    pub(in crate::svc::vast) fn new() -> Self {
        Self {
            fits: Vec::new(),
            not_loaded_item: None,
        }
    }
    pub fn all_passed(&self) -> bool {
        let Self { fits, not_loaded_item } = self;
        fits.is_empty() && not_loaded_item.is_none()
    }
}

/// Validation result for a fit.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct ValResultFit {
    // Generic
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub not_loaded_item: Option<ValNotLoadedItemFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub item_kind: Option<ValItemKindFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub skill_reqs: Option<ValSrqFail> = None,
    // Implants/boosters
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub implant_slot_index: Option<ValSlotIndexFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub booster_slot_index: Option<ValSlotIndexFail> = None,
    // Shared between mod-alike items
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub cpu: Option<ValResourceFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub powergrid: Option<ValResourceFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ship_limit: Option<ValShipLimitFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub max_group_fitted: Option<ValMaxGroupFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub max_group_online: Option<ValMaxGroupFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub max_group_active: Option<ValMaxGroupFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub max_type_fitted: Option<ValMaxTypeFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub item_vs_ship_kind: Option<ValItemVsShipKindFail> = None,
    // Modules
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub high_slot_count: Option<ValSlotCountFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub mid_slot_count: Option<ValSlotCountFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub low_slot_count: Option<ValSlotCountFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub turret_slot_count: Option<ValSlotCountFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub launcher_slot_count: Option<ValSlotCountFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub module_state: Option<ValModuleStateFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub capital_module: Option<ValCapitalModFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub overload_skill: Option<ValOverloadSkillFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub unusable_cap: Option<ValUnusableCapFail> = None,
    // Charges
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub charge_group: Option<ValChargeGroupFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub charge_parent_group: Option<ValChargeParentGroupFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub charge_size: Option<ValChargeSizeFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub charge_volume: Option<ValChargeVolumeFail> = None,
    // Rigs
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub rig_slot_count: Option<ValSlotCountFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub calibration: Option<ValResourceFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub rig_size: Option<ValRigSizeFail> = None,
    // Services
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub service_slot_count: Option<ValSlotCountFail> = None,
    // T3 subsystems/stances
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub subsystem_slot_count: Option<ValSlotCountFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub subsystem_slot_index: Option<ValSlotIndexFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ship_stance: Option<ValShipStanceFail> = None,
    // Drones
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub drone_bay_volume: Option<ValResourceFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub launched_drone_count: Option<ValSlotCountFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub drone_bandwidth: Option<ValResourceFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub unlaunchable_drone_slot: Option<ValUnusableSlotFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub unlaunchable_drone_bandwidth: Option<ValUnusableResFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub drone_group: Option<ValDroneGroupFail> = None,
    // Fighters
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fighter_bay_volume: Option<ValResourceFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub launched_fighter_count: Option<ValSlotCountFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub launched_light_fighter_count: Option<ValSlotCountFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub launched_heavy_fighter_count: Option<ValSlotCountFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub launched_support_fighter_count: Option<ValSlotCountFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub launched_st_light_fighter_count: Option<ValSlotCountFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub launched_st_heavy_fighter_count: Option<ValSlotCountFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub launched_st_support_fighter_count: Option<ValSlotCountFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub unlaunchable_fighter: Option<ValUnusableSlotFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub unlaunchable_light_fighter: Option<ValUnusableSlotFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub unlaunchable_heavy_fighter: Option<ValUnusableSlotFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub unlaunchable_support_fighter: Option<ValUnusableSlotFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub unlaunchable_st_light_fighter: Option<ValUnusableSlotFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub unlaunchable_st_heavy_fighter: Option<ValUnusableSlotFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub unlaunchable_st_support_fighter: Option<ValUnusableSlotFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fighter_squad_size: Option<ValFighterSquadSizeFail> = None,
    // Projection, destination side
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub activation_blocked: Option<ValActivationBlockedFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub effect_stopper: Option<ValEffectStopperFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub cloaking_blocked: Option<ValCloakingBlockedFail> = None,
    // Projection, source side
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub projectee_filter: Option<ValProjFilterFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub assist_immunity: Option<ValProjImmunityFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub offense_immunity: Option<ValProjImmunityFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub resist_immunity: Option<ValProjImmunityFail> = None,
    // Sec zone
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub sec_zone_fitted: Option<ValItemSecZoneFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub sec_zone_online: Option<ValItemSecZoneFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub sec_zone_active: Option<ValItemSecZoneFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub sec_zone_unonlineable: Option<ValItemSecZoneFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub sec_zone_unactivable: Option<ValItemSecZoneFail> = None,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub sec_zone_effect: Option<ValEffectSecZoneFail> = None,
}
impl ValResultFit {
    pub fn all_passed(&self) -> bool {
        let Self {
            not_loaded_item,
            item_kind,
            skill_reqs,
            implant_slot_index,
            booster_slot_index,
            cpu,
            powergrid,
            ship_limit,
            max_group_fitted,
            max_group_online,
            max_group_active,
            max_type_fitted,
            item_vs_ship_kind,
            high_slot_count,
            mid_slot_count,
            low_slot_count,
            turret_slot_count,
            launcher_slot_count,
            module_state,
            capital_module,
            overload_skill,
            unusable_cap,
            charge_group,
            charge_parent_group,
            charge_size,
            charge_volume,
            rig_slot_count,
            calibration,
            rig_size,
            service_slot_count,
            subsystem_slot_count,
            subsystem_slot_index,
            ship_stance,
            drone_bay_volume,
            launched_drone_count,
            drone_bandwidth,
            unlaunchable_drone_slot,
            unlaunchable_drone_bandwidth,
            drone_group,
            fighter_bay_volume,
            launched_fighter_count,
            launched_light_fighter_count,
            launched_heavy_fighter_count,
            launched_support_fighter_count,
            launched_st_light_fighter_count,
            launched_st_heavy_fighter_count,
            launched_st_support_fighter_count,
            unlaunchable_fighter,
            unlaunchable_light_fighter,
            unlaunchable_heavy_fighter,
            unlaunchable_support_fighter,
            unlaunchable_st_light_fighter,
            unlaunchable_st_heavy_fighter,
            unlaunchable_st_support_fighter,
            fighter_squad_size,
            activation_blocked,
            effect_stopper,
            cloaking_blocked,
            projectee_filter,
            assist_immunity,
            offense_immunity,
            resist_immunity,
            sec_zone_fitted,
            sec_zone_online,
            sec_zone_active,
            sec_zone_unonlineable,
            sec_zone_unactivable,
            sec_zone_effect,
        } = self;
        // Generic
        not_loaded_item.is_none()
            && item_kind.is_none()
            && skill_reqs.is_none()
            // Implants/boosters
            && implant_slot_index.is_none()
            && booster_slot_index.is_none()
            // Shared between mod-alike items
            && cpu.is_none()
            && powergrid.is_none()
            && ship_limit.is_none()
            && max_group_fitted.is_none()
            && max_group_online.is_none()
            && max_group_active.is_none()
            && max_type_fitted.is_none()
            && item_vs_ship_kind.is_none()
            // Modules
            && high_slot_count.is_none()
            && mid_slot_count.is_none()
            && low_slot_count.is_none()
            && turret_slot_count.is_none()
            && launcher_slot_count.is_none()
            && module_state.is_none()
            && capital_module.is_none()
            && overload_skill.is_none()
            && unusable_cap.is_none()
            // Charges
            && charge_group.is_none()
            && charge_parent_group.is_none()
            && charge_size.is_none()
            && charge_volume.is_none()
            // Rigs
            && rig_slot_count.is_none()
            && calibration.is_none()
            && rig_size.is_none()
            // Services
            && service_slot_count.is_none()
            // T3 subsystems/stances
            && subsystem_slot_count.is_none()
            && subsystem_slot_index.is_none()
            && ship_stance.is_none()
            // Drones
            && drone_bay_volume.is_none()
            && launched_drone_count.is_none()
            && drone_bandwidth.is_none()
            && unlaunchable_drone_slot.is_none()
            && unlaunchable_drone_bandwidth.is_none()
            && drone_group.is_none()
            // Fighters
            && fighter_bay_volume.is_none()
            && launched_fighter_count.is_none()
            && launched_light_fighter_count.is_none()
            && launched_heavy_fighter_count.is_none()
            && launched_support_fighter_count.is_none()
            && launched_st_light_fighter_count.is_none()
            && launched_st_heavy_fighter_count.is_none()
            && launched_st_support_fighter_count.is_none()
            && unlaunchable_fighter.is_none()
            && unlaunchable_light_fighter.is_none()
            && unlaunchable_heavy_fighter.is_none()
            && unlaunchable_support_fighter.is_none()
            && unlaunchable_st_light_fighter.is_none()
            && unlaunchable_st_heavy_fighter.is_none()
            && unlaunchable_st_support_fighter.is_none()
            && fighter_squad_size.is_none()
            // Projection, destination side
            && activation_blocked.is_none()
            && effect_stopper.is_none()
            && cloaking_blocked.is_none()
            // Projection, source side
            && projectee_filter.is_none()
            && assist_immunity.is_none()
            && offense_immunity.is_none()
            && resist_immunity.is_none()
            // Sec zone
            && sec_zone_fitted.is_none()
            && sec_zone_online.is_none()
            && sec_zone_active.is_none()
            && sec_zone_unonlineable.is_none()
            && sec_zone_unactivable.is_none()
            && sec_zone_effect.is_none()
    }
}
