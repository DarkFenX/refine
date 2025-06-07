use crate::{
    ac,
    sol::{
        Count,
        svc::{
            calc::Calc,
            vast::{VastFitData, shared::get_attr_as_count},
        },
        uad::{Uad, fit::UadFit},
    },
};

pub struct StatSlot {
    pub used: Count,
    pub total: Option<Count>,
}

impl VastFitData {
    // Public methods
    pub(in crate::sol) fn get_stat_rig_slots(&self, uad: &Uad, calc: &mut Calc, fit: &UadFit) -> StatSlot {
        let used = fit.rigs.len() as Count;
        let total = get_attr_as_count(uad, calc, fit.ship, &ac::attrs::UPGRADE_SLOTS_LEFT);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stat_service_slots(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> StatSlot {
        let used = fit.services.len() as Count;
        let total = get_attr_as_count(uad, calc, fit.ship, &ac::attrs::SERVICE_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stat_subsystem_slots(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> StatSlot {
        let used = fit.subsystems.len() as Count;
        let total = get_attr_as_count(uad, calc, fit.ship, &ac::attrs::MAX_SUBSYSTEMS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stat_launched_drones(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> StatSlot {
        let used = self.drones_online_bandwidth.len() as Count;
        let total = get_attr_as_count(uad, calc, fit.character, &ac::attrs::MAX_ACTIVE_DRONES);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stat_launched_fighters(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> StatSlot {
        let used = self.fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, fit.ship, &ac::attrs::FTR_TUBES);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stat_launched_support_fighters(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> StatSlot {
        let used = self.support_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, fit.ship, &ac::attrs::FTR_SUPPORT_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stat_launched_light_fighters(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> StatSlot {
        let used = self.light_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, fit.ship, &ac::attrs::FTR_LIGHT_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stat_launched_heavy_fighters(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> StatSlot {
        let used = self.heavy_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, fit.ship, &ac::attrs::FTR_HEAVY_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stat_launched_standup_support_fighters(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> StatSlot {
        let used = self.standup_support_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, fit.ship, &ac::attrs::FTR_STANDUP_SUPPORT_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stat_launched_standup_light_fighters(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> StatSlot {
        let used = self.standup_light_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, fit.ship, &ac::attrs::FTR_STANDUP_LIGHT_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stat_launched_standup_heavy_fighters(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> StatSlot {
        let used = self.standup_heavy_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, fit.ship, &ac::attrs::FTR_STANDUP_HEAVY_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stat_turret_slots(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> StatSlot {
        let used = self.mods_turret.len() as Count;
        let total = get_attr_as_count(uad, calc, fit.ship, &ac::attrs::TURRET_SLOTS_LEFT);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stat_launcher_slots(
        &self,
        uad: &Uad,
        calc: &mut Calc,
        fit: &UadFit,
    ) -> StatSlot {
        let used = self.mods_launcher.len() as Count;
        let total = get_attr_as_count(uad, calc, fit.ship, &ac::attrs::LAUNCHER_SLOTS_LEFT);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stat_high_slots(&self, uad: &Uad, calc: &mut Calc, fit: &UadFit) -> StatSlot {
        let used = fit.mods_high.len() as Count;
        let total = get_attr_as_count(uad, calc, fit.ship, &ac::attrs::HI_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stat_mid_slots(&self, uad: &Uad, calc: &mut Calc, fit: &UadFit) -> StatSlot {
        let used = fit.mods_mid.len() as Count;
        let total = get_attr_as_count(uad, calc, fit.ship, &ac::attrs::MED_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stat_low_slots(&self, uad: &Uad, calc: &mut Calc, fit: &UadFit) -> StatSlot {
        let used = fit.mods_low.len() as Count;
        let total = get_attr_as_count(uad, calc, fit.ship, &ac::attrs::LOW_SLOTS);
        StatSlot { used, total }
    }
}
