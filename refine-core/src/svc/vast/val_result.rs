use crate::{
    svc::vast::{
        ValActivationBlockedFail, ValCapitalModFail, ValChargeGroupFail, ValChargeParentGroupFail, ValChargeSizeFail,
        ValChargeVolumeFail, ValCloakingBlockedFail, ValDroneGroupFail, ValEffectSecZoneFail, ValEffectStopperFail,
        ValFighterSquadSizeFail, ValItemKindFail, ValItemSecZoneFail, ValItemVsShipKindFail, ValMaxGroupFail,
        ValMaxTypeFail, ValModuleStateFail, ValNotLoadedItemFail, ValOverloadSkillFail, ValProjFilterFail,
        ValProjImmunityFail, ValResourceFail, ValRigSizeFail, ValShipLimitFail, ValShipStanceFail, ValSlotCountFail,
        ValSlotIndexFail, ValSrqFail, ValUnusableCapFail, ValUnusableResFail, ValUnusableSlotFail,
    },
    ud::FitId,
};

/// Validation result for a solar system.
///
/// Contains per-fit failures, and failures for items not belonging to any fit.
#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
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
        self.fits.is_empty() && self.not_loaded_item.is_none()
    }
}

/// Validation result for a fit.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
        // Generic
        self.not_loaded_item.is_none()
            && self.item_kind.is_none()
            && self.skill_reqs.is_none()
            // Implants/boosters
            && self.implant_slot_index.is_none()
            && self.booster_slot_index.is_none()
            // Shared between mod-alike items
            && self.cpu.is_none()
            && self.powergrid.is_none()
            && self.ship_limit.is_none()
            && self.max_group_fitted.is_none()
            && self.max_group_online.is_none()
            && self.max_group_active.is_none()
            && self.max_type_fitted.is_none()
            && self.item_vs_ship_kind.is_none()
            // Modules
            && self.high_slot_count.is_none()
            && self.mid_slot_count.is_none()
            && self.low_slot_count.is_none()
            && self.turret_slot_count.is_none()
            && self.launcher_slot_count.is_none()
            && self.module_state.is_none()
            && self.capital_module.is_none()
            && self.overload_skill.is_none()
            && self.unusable_cap.is_none()
            // Charges
            && self.charge_group.is_none()
            && self.charge_parent_group.is_none()
            && self.charge_size.is_none()
            && self.charge_volume.is_none()
            // Rigs
            && self.rig_slot_count.is_none()
            && self.calibration.is_none()
            && self.rig_size.is_none()
            // Services
            && self.service_slot_count.is_none()
            // T3 subsystems/stances
            && self.subsystem_slot_count.is_none()
            && self.subsystem_slot_index.is_none()
            && self.ship_stance.is_none()
            // Drones
            && self.drone_bay_volume.is_none()
            && self.launched_drone_count.is_none()
            && self.drone_bandwidth.is_none()
            && self.unlaunchable_drone_slot.is_none()
            && self.unlaunchable_drone_bandwidth.is_none()
            && self.drone_group.is_none()
            // Fighters
            && self.fighter_bay_volume.is_none()
            && self.launched_fighter_count.is_none()
            && self.launched_light_fighter_count.is_none()
            && self.launched_heavy_fighter_count.is_none()
            && self.launched_support_fighter_count.is_none()
            && self.launched_st_light_fighter_count.is_none()
            && self.launched_st_heavy_fighter_count.is_none()
            && self.launched_st_support_fighter_count.is_none()
            && self.unlaunchable_fighter.is_none()
            && self.unlaunchable_light_fighter.is_none()
            && self.unlaunchable_heavy_fighter.is_none()
            && self.unlaunchable_support_fighter.is_none()
            && self.unlaunchable_st_light_fighter.is_none()
            && self.unlaunchable_st_heavy_fighter.is_none()
            && self.unlaunchable_st_support_fighter.is_none()
            && self.fighter_squad_size.is_none()
            // Projection, destination side
            && self.activation_blocked.is_none()
            && self.effect_stopper.is_none()
            && self.cloaking_blocked.is_none()
            // Projection, source side
            && self.projectee_filter.is_none()
            && self.assist_immunity.is_none()
            && self.offense_immunity.is_none()
            && self.resist_immunity.is_none()
            // Sec zone
            && self.sec_zone_fitted.is_none()
            && self.sec_zone_online.is_none()
            && self.sec_zone_active.is_none()
            && self.sec_zone_unonlineable.is_none()
            && self.sec_zone_unactivable.is_none()
            && self.sec_zone_effect.is_none()
    }
}
