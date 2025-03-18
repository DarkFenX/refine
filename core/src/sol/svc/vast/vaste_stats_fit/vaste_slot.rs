use crate::{
    defs::{Count, EAttrId, SolItemId},
    ec,
    sol::{
        svc::{calc::SolCalc, vast::SolVastFitData},
        uad::{SolUad, fit::SolFit},
    },
};

pub struct SolStatSlot {
    pub used: Count,
    pub total: Option<Count>,
}

impl SolVastFitData {
    // Public methods
    pub(in crate::sol::svc::vast) fn get_stats_rig_slots(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        let used = fit.rigs.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ec::attrs::UPGRADE_SLOTS_LEFT);
        SolStatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_service_slots(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        let used = fit.services.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ec::attrs::SERVICE_SLOTS);
        SolStatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_subsystem_slots(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        let used = fit.subsystems.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ec::attrs::MAX_SUBSYSTEMS);
        SolStatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_drones(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        let used = self.drones_online_bandwidth.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.character, &ec::attrs::MAX_ACTIVE_DRONES);
        SolStatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_fighters(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        let used = self.fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ec::attrs::FTR_TUBES);
        SolStatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_support_fighters(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        let used = self.support_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ec::attrs::FTR_SUPPORT_SLOTS);
        SolStatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_light_fighters(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        let used = self.light_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ec::attrs::FTR_LIGHT_SLOTS);
        SolStatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_heavy_fighters(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        let used = self.heavy_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ec::attrs::FTR_HEAVY_SLOTS);
        SolStatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_standup_support_fighters(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        let used = self.standup_support_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ec::attrs::FTR_STANDUP_SUPPORT_SLOTS);
        SolStatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_standup_light_fighters(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        let used = self.standup_light_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ec::attrs::FTR_STANDUP_LIGHT_SLOTS);
        SolStatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launched_standup_heavy_fighters(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        let used = self.standup_heavy_fighters_online.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ec::attrs::FTR_STANDUP_HEAVY_SLOTS);
        SolStatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_turret_slots(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        let used = self.mods_turret.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ec::attrs::TURRET_SLOTS_LEFT);
        SolStatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_launcher_slots(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        let used = self.mods_launcher.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ec::attrs::LAUNCHER_SLOTS_LEFT);
        SolStatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_high_slots(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        let used = fit.mods_high.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ec::attrs::HI_SLOTS);
        SolStatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_mid_slots(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        let used = fit.mods_mid.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ec::attrs::MED_SLOTS);
        SolStatSlot { used, total }
    }
    pub(in crate::sol::svc::vast) fn get_stats_low_slots(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> SolStatSlot {
        let used = fit.mods_low.len() as Count;
        let total = get_attr_as_count(uad, calc, &fit.ship, &ec::attrs::LOW_SLOTS);
        SolStatSlot { used, total }
    }
}

fn get_attr_as_count(
    uad: &SolUad,
    calc: &mut SolCalc,
    output_item_id: &Option<SolItemId>,
    output_attr_id: &EAttrId,
) -> Option<Count> {
    calc.get_item_attr_val_simple_opt(uad, output_item_id, output_attr_id)
        .map(|v| v.round() as Count)
}
