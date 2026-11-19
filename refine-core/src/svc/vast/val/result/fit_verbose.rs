use crate::val::{
    ValActivationBlockedFail, ValCapitalModFail, ValChargeGroupFail, ValChargeSizeFail, ValChargeVolumeFail,
    ValCloakingBlockedFail, ValDroneGroupFail, ValEffectSecZoneFail, ValEffectStopperFail, ValFighterSquadSizeFail,
    ValItemKindFail, ValItemSecZoneFail, ValItemVsShipKindFail, ValMaxGroupFail, ValMaxTypeFail, ValModuleStateFail,
    ValNotLoadedItemFail, ValOverloadSkillFail, ValProjFilterFail, ValProjImmunityFail, ValResourceFail, ValResult,
    ValRigSizeFail, ValShipLimitFail, ValShipStanceFail, ValSlotCountFail, ValSlotIndexFail, ValSrqFail,
    ValUnusableCapFail, ValUnusableResFail, ValUnusableSlotFail,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Validation result for a fit.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone)]
pub struct ValResultFit {
    // Generic
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub not_loaded_item: ValResult<ValNotLoadedItemFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub item_kind: ValResult<ValItemKindFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub skill_reqs: ValResult<ValSrqFail> = ValResult::NotRequested,
    // Implants/boosters
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub implant_slot_index: ValResult<ValSlotIndexFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub booster_slot_index: ValResult<ValSlotIndexFail> = ValResult::NotRequested,
    // Shared between mod-alike items
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub cpu: ValResult<ValResourceFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub powergrid: ValResult<ValResourceFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub ship_limit: ValResult<ValShipLimitFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub max_group_fitted: ValResult<ValMaxGroupFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub max_group_online: ValResult<ValMaxGroupFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub max_group_active: ValResult<ValMaxGroupFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub max_type_fitted: ValResult<ValMaxTypeFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub item_vs_ship_kind: ValResult<ValItemVsShipKindFail> = ValResult::NotRequested,
    // Modules
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub high_slot_count: ValResult<ValSlotCountFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub mid_slot_count: ValResult<ValSlotCountFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub low_slot_count: ValResult<ValSlotCountFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub turret_slot_count: ValResult<ValSlotCountFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub launcher_slot_count: ValResult<ValSlotCountFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub module_state: ValResult<ValModuleStateFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub capital_module: ValResult<ValCapitalModFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub overload_skill: ValResult<ValOverloadSkillFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub unusable_cap: ValResult<ValUnusableCapFail> = ValResult::NotRequested,
    // Charges
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub charge_group: ValResult<ValChargeGroupFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub charge_size: ValResult<ValChargeSizeFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub charge_volume: ValResult<ValChargeVolumeFail> = ValResult::NotRequested,
    // Rigs
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub rig_slot_count: ValResult<ValSlotCountFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub calibration: ValResult<ValResourceFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub rig_size: ValResult<ValRigSizeFail> = ValResult::NotRequested,
    // Services
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub service_slot_count: ValResult<ValSlotCountFail> = ValResult::NotRequested,
    // T3 subsystems/stances
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub subsystem_slot_count: ValResult<ValSlotCountFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub subsystem_slot_index: ValResult<ValSlotIndexFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub ship_stance: ValResult<ValShipStanceFail> = ValResult::NotRequested,
    // Drones
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub drone_bay_volume: ValResult<ValResourceFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub launched_drone_count: ValResult<ValSlotCountFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub drone_bandwidth_current: ValResult<ValResourceFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub drone_bandwidth_potential: ValResult<ValResourceFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub unlaunchable_drone_slot: ValResult<ValUnusableSlotFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub unlaunchable_drone_bandwidth: ValResult<ValUnusableResFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub drone_group: ValResult<ValDroneGroupFail> = ValResult::NotRequested,
    // Fighters
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub fighter_bay_volume_all: ValResult<ValResourceFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub fighter_bay_volume_inbay: ValResult<ValResourceFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub launched_fighter_count: ValResult<ValSlotCountFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub launched_light_fighter_count: ValResult<ValSlotCountFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub launched_heavy_fighter_count: ValResult<ValSlotCountFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub launched_support_fighter_count: ValResult<ValSlotCountFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub launched_st_light_fighter_count: ValResult<ValSlotCountFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub launched_st_heavy_fighter_count: ValResult<ValSlotCountFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub launched_st_support_fighter_count: ValResult<ValSlotCountFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub unlaunchable_fighter: ValResult<ValUnusableSlotFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub unlaunchable_light_fighter: ValResult<ValUnusableSlotFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub unlaunchable_heavy_fighter: ValResult<ValUnusableSlotFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub unlaunchable_support_fighter: ValResult<ValUnusableSlotFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub unlaunchable_st_light_fighter: ValResult<ValUnusableSlotFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub unlaunchable_st_heavy_fighter: ValResult<ValUnusableSlotFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub unlaunchable_st_support_fighter: ValResult<ValUnusableSlotFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub fighter_squad_size: ValResult<ValFighterSquadSizeFail> = ValResult::NotRequested,
    // Projection, destination side
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub activation_blocked: ValResult<ValActivationBlockedFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub effect_stopper: ValResult<ValEffectStopperFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub cloaking_blocked: ValResult<ValCloakingBlockedFail> = ValResult::NotRequested,
    // Projection, source side
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub projectee_filter: ValResult<ValProjFilterFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub assist_immunity: ValResult<ValProjImmunityFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub offense_immunity: ValResult<ValProjImmunityFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub resist_immunity: ValResult<ValProjImmunityFail> = ValResult::NotRequested,
    // Sec zone
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub sec_zone_fitted: ValResult<ValItemSecZoneFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub sec_zone_online: ValResult<ValItemSecZoneFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub sec_zone_active: ValResult<ValItemSecZoneFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub sec_zone_unonlineable: ValResult<ValItemSecZoneFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub sec_zone_unactivable: ValResult<ValItemSecZoneFail> = ValResult::NotRequested,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "ValResult::is_not_failed"))]
    pub sec_zone_effect: ValResult<ValEffectSecZoneFail> = ValResult::NotRequested,
}
impl ValResultFit {
    pub fn all_passed(&self) -> bool {
        let Self {
            // Generic
            not_loaded_item,
            item_kind,
            skill_reqs,
            // Implants/boosters
            implant_slot_index,
            booster_slot_index,
            // Shared between mod-alike items
            cpu,
            powergrid,
            ship_limit,
            max_group_fitted,
            max_group_online,
            max_group_active,
            max_type_fitted,
            item_vs_ship_kind,
            // Modules
            high_slot_count,
            mid_slot_count,
            low_slot_count,
            turret_slot_count,
            launcher_slot_count,
            module_state,
            capital_module,
            overload_skill,
            unusable_cap,
            // Charges
            charge_group,
            charge_size,
            charge_volume,
            // Rigs
            rig_slot_count,
            calibration,
            rig_size,
            // Services
            service_slot_count,
            // T3 subsystems/stances
            subsystem_slot_count,
            subsystem_slot_index,
            ship_stance,
            // Drones
            drone_bay_volume,
            launched_drone_count,
            drone_bandwidth_current,
            drone_bandwidth_potential,
            unlaunchable_drone_slot,
            unlaunchable_drone_bandwidth,
            drone_group,
            // Fighters
            fighter_bay_volume_all,
            fighter_bay_volume_inbay,
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
            // Projection, destination side
            activation_blocked,
            effect_stopper,
            cloaking_blocked,
            // Projection, source side
            projectee_filter,
            assist_immunity,
            offense_immunity,
            resist_immunity,
            // Sec zone
            sec_zone_fitted,
            sec_zone_online,
            sec_zone_active,
            sec_zone_unonlineable,
            sec_zone_unactivable,
            sec_zone_effect,
        } = self;
        // Generic
        !not_loaded_item.is_failed()
            && !item_kind.is_failed()
            && !skill_reqs.is_failed()
            // Implants/boosters
            && !implant_slot_index.is_failed()
            && !booster_slot_index.is_failed()
            // Shared between mod-alike items
            && !cpu.is_failed()
            && !powergrid.is_failed()
            && !ship_limit.is_failed()
            && !max_group_fitted.is_failed()
            && !max_group_online.is_failed()
            && !max_group_active.is_failed()
            && !max_type_fitted.is_failed()
            && !item_vs_ship_kind.is_failed()
            // Modules
            && !high_slot_count.is_failed()
            && !mid_slot_count.is_failed()
            && !low_slot_count.is_failed()
            && !turret_slot_count.is_failed()
            && !launcher_slot_count.is_failed()
            && !module_state.is_failed()
            && !capital_module.is_failed()
            && !overload_skill.is_failed()
            && !unusable_cap.is_failed()
            // Charges
            && !charge_group.is_failed()
            && !charge_size.is_failed()
            && !charge_volume.is_failed()
            // Rigs
            && !rig_slot_count.is_failed()
            && !calibration.is_failed()
            && !rig_size.is_failed()
            // Services
            && !service_slot_count.is_failed()
            // T3 subsystems/stances
            && !subsystem_slot_count.is_failed()
            && !subsystem_slot_index.is_failed()
            && !ship_stance.is_failed()
            // Drones
            && !drone_bay_volume.is_failed()
            && !launched_drone_count.is_failed()
            && !drone_bandwidth_current.is_failed()
            && !drone_bandwidth_potential.is_failed()
            && !unlaunchable_drone_slot.is_failed()
            && !unlaunchable_drone_bandwidth.is_failed()
            && !drone_group.is_failed()
            // Fighters
            && !fighter_bay_volume_all.is_failed()
            && !fighter_bay_volume_inbay.is_failed()
            && !launched_fighter_count.is_failed()
            && !launched_light_fighter_count.is_failed()
            && !launched_heavy_fighter_count.is_failed()
            && !launched_support_fighter_count.is_failed()
            && !launched_st_light_fighter_count.is_failed()
            && !launched_st_heavy_fighter_count.is_failed()
            && !launched_st_support_fighter_count.is_failed()
            && !unlaunchable_fighter.is_failed()
            && !unlaunchable_light_fighter.is_failed()
            && !unlaunchable_heavy_fighter.is_failed()
            && !unlaunchable_support_fighter.is_failed()
            && !unlaunchable_st_light_fighter.is_failed()
            && !unlaunchable_st_heavy_fighter.is_failed()
            && !unlaunchable_st_support_fighter.is_failed()
            && !fighter_squad_size.is_failed()
            // Projection, destination side
            && !activation_blocked.is_failed()
            && !effect_stopper.is_failed()
            && !cloaking_blocked.is_failed()
            // Projection, source side
            && !projectee_filter.is_failed()
            && !assist_immunity.is_failed()
            && !offense_immunity.is_failed()
            && !resist_immunity.is_failed()
            // Sec zone
            && !sec_zone_fitted.is_failed()
            && !sec_zone_online.is_failed()
            && !sec_zone_active.is_failed()
            && !sec_zone_unonlineable.is_failed()
            && !sec_zone_unactivable.is_failed()
            && !sec_zone_effect.is_failed()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ValResultFit {
    pub(crate) fn potential_drone_bandwidth_passed(&self) -> bool {
        !self.drone_bandwidth_potential.is_failed() && !self.unlaunchable_drone_bandwidth.is_failed()
    }
}
