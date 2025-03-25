use crate::{
    ac, ad,
    sol::{
        Count, ItemId,
        svc::{calc::Calc, vast::VastFitData},
        uad::{Uad, fit::Fit},
    },
};

pub struct StatSlot {
    pub used: Count,
    pub total: Option<Count>,
}

impl VastFitData {
    // Public methods
    pub(in crate::sol::svc::vast) fn get_stats_rig_slots(&self, uad: &Uad, calc: &mut Calc, fit: &Fit) -> StatSlot {
        let used = fit.rigs.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ac::attrs::UPGRADE_SLOTS_LEFT);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_service_slots(&self, uad: &Uad, calc: &mut Calc, fit: &Fit) -> StatSlot {
        let used = fit.services.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ac::attrs::SERVICE_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_subsystem_slots(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> StatSlot {
        let used = fit.subsystems.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ac::attrs::MAX_SUBSYSTEMS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_drones(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> StatSlot {
        let used = self.drones_online_bandwidth.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.character, &ac::attrs::MAX_ACTIVE_DRONES);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_fighters(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> StatSlot {
        let used = self.fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ac::attrs::FTR_TUBES);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_support_fighters(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> StatSlot {
        let used = self.support_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ac::attrs::FTR_SUPPORT_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_light_fighters(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> StatSlot {
        let used = self.light_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ac::attrs::FTR_LIGHT_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_heavy_fighters(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> StatSlot {
        let used = self.heavy_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ac::attrs::FTR_HEAVY_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_standup_support_fighters(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> StatSlot {
        let used = self.standup_support_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ac::attrs::FTR_STANDUP_SUPPORT_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_standup_light_fighters(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> StatSlot {
        let used = self.standup_light_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ac::attrs::FTR_STANDUP_LIGHT_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_standup_heavy_fighters(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> StatSlot {
        let used = self.standup_heavy_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ac::attrs::FTR_STANDUP_HEAVY_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_turret_slots(&self, uad: &Uad, calc: &mut Calc, fit: &Fit) -> StatSlot {
        let used = self.mods_turret.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ac::attrs::TURRET_SLOTS_LEFT);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launcher_slots(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &Fit,
    ) -> StatSlot {
        let used = self.mods_launcher.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ac::attrs::LAUNCHER_SLOTS_LEFT);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_high_slots(&self, uad: &Uad, calc: &mut Calc, fit: &Fit) -> StatSlot {
        let used = fit.mods_high.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ac::attrs::HI_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_mid_slots(&self, uad: &Uad, calc: &mut Calc, fit: &Fit) -> StatSlot {
        let used = fit.mods_mid.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ac::attrs::MED_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_low_slots(&self, uad: &Uad, calc: &mut Calc, fit: &Fit) -> StatSlot {
        let used = fit.mods_low.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ac::attrs::LOW_SLOTS);
        StatSlot { used, total }
    }
}

fn get_attr_as_count(
    uad: &Uad,
    calc: &mut Calc,
    output_item_id: &Option<ItemId>,
    output_a_attr_id: &ad::AAttrId,
) -> Option<Count> {
    calc.get_item_attr_val_simple_opt(uad, output_item_id, output_a_attr_id)
        .map(|v| v.round() as Count)
}
