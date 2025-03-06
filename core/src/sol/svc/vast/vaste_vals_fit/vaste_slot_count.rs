use crate::{
    defs::{Count, Idx, SolItemId},
    sol::{
        svc::{calc::SolCalc, vast::SolVastFitData},
        uad::{SolUad, fit::SolFit},
    },
};

pub struct SolValSlotCountFail {
    pub used: Count,
    pub total: Option<Count>,
    pub users: Vec<SolItemId>,
}
impl SolValSlotCountFail {
    fn new(used: Count, total: Option<Count>, users: Vec<SolItemId>) -> Self {
        Self { used, total, users }
    }
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_rig_slot_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_rig_slots(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_subsystem_slots(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_drone_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launched_drones(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_fighter_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launched_fighters(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_support_fighter_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launched_support_fighters(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_light_fighter_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launched_light_fighters(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_heavy_fighter_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launched_heavy_fighters(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_support_fighter_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launched_standup_support_fighters(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_light_fighter_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launched_standup_light_fighters(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_heavy_fighter_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launched_standup_heavy_fighters(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_turret_slot_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_turret_slots(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launcher_slot_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launcher_slots(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_high_slot_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_high_slots(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_mid_slot_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_mid_slots(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_low_slot_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_low_slots(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_rig_slot_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_rig_slots(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = fit.rigs.iter().copied().collect();
        Some(SolValSlotCountFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_subsystem_slots(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = fit.subsystems.iter().copied().collect();
        Some(SolValSlotCountFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launched_drone_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launched_drones(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.drones_online_bandwidth.keys().copied().collect();
        Some(SolValSlotCountFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launched_fighter_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launched_fighters(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.fighters_online.iter().copied().collect();
        Some(SolValSlotCountFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launched_support_fighter_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launched_support_fighters(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.support_fighters_online.iter().copied().collect();
        Some(SolValSlotCountFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launched_light_fighter_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launched_light_fighters(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.light_fighters_online.iter().copied().collect();
        Some(SolValSlotCountFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launched_heavy_fighter_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launched_heavy_fighters(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.heavy_fighters_online.iter().copied().collect();
        Some(SolValSlotCountFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_support_fighter_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launched_standup_support_fighters(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.standup_support_fighters_online.iter().copied().collect();
        Some(SolValSlotCountFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_light_fighter_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launched_standup_light_fighters(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.standup_light_fighters_online.iter().copied().collect();
        Some(SolValSlotCountFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_heavy_fighter_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launched_standup_heavy_fighters(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.standup_heavy_fighters_online.iter().copied().collect();
        Some(SolValSlotCountFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_turret_slot_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_turret_slots(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.mods_turret.iter().copied().collect();
        Some(SolValSlotCountFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launcher_slot_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launcher_slots(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.mods_launcher.iter().copied().collect();
        Some(SolValSlotCountFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_high_slot_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_high_slots(uad, calc, fit);
        let total = stats.total.unwrap_or(0);
        if stats.used <= total {
            return None;
        }
        let users = match total >= fit.mods_high.len() as Count {
            true => Vec::new(),
            false => fit.mods_high.inner()[total as Idx..]
                .iter()
                .filter_map(|v| *v)
                .collect(),
        };
        Some(SolValSlotCountFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_mid_slot_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_mid_slots(uad, calc, fit);
        let total = stats.total.unwrap_or(0);
        if stats.used <= total {
            return None;
        }
        let users = match total >= fit.mods_mid.len() as Count {
            true => Vec::new(),
            false => fit.mods_mid.inner()[total as Idx..].iter().filter_map(|v| *v).collect(),
        };
        Some(SolValSlotCountFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_low_slot_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_low_slots(uad, calc, fit);
        let total = stats.total.unwrap_or(0);
        if stats.used <= total {
            return None;
        }
        let users = match total >= fit.mods_low.len() as Count {
            true => Vec::new(),
            false => fit.mods_low.inner()[total as Idx..].iter().filter_map(|v| *v).collect(),
        };
        Some(SolValSlotCountFail::new(stats.used, stats.total, users))
    }
}
