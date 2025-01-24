use crate::{
    defs::{Count, Idx, SolItemId},
    sol::{
        svc::{calc::SolCalc, vast::SolVastFitData},
        uad::{fit::SolFit, SolUad},
    },
};

pub struct SolSlotValFail {
    pub used: Count,
    pub total: Option<Count>,
    pub users: Vec<SolItemId>,
}
impl SolSlotValFail {
    fn new(used: Count, total: Option<Count>, users: Vec<SolItemId>) -> Self {
        Self { used, total, users }
    }
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_rig_slots_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_rig_slots(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slots_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_subsystem_slots(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_drones_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launched_drones(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_fighters_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launched_fighters(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_support_fighters_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launched_support_fighters(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_light_fighters_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launched_light_fighters(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_heavy_fighters_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launched_heavy_fighters(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_support_fighters_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launched_standup_support_fighters(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_light_fighters_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launched_standup_light_fighters(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_heavy_fighters_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launched_standup_heavy_fighters(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_turret_slots_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_turret_slots(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_launcher_slots_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_launcher_slots(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_high_slots_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_high_slots(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_mid_slots_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_mid_slots(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    pub(in crate::sol::svc::vast) fn validate_low_slots_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_low_slots(uad, calc, fit);
        stats.used <= stats.total.unwrap_or(0)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_rig_slots_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolSlotValFail> {
        let stats = self.get_stats_rig_slots(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = fit.rigs.iter().map(|v| *v).collect();
        Some(SolSlotValFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slots_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolSlotValFail> {
        let stats = self.get_stats_subsystem_slots(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = fit.subsystems.iter().map(|v| *v).collect();
        Some(SolSlotValFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launched_drones_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolSlotValFail> {
        let stats = self.get_stats_launched_drones(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.drones_online_bandwidth.keys().map(|v| *v).collect();
        Some(SolSlotValFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launched_fighters_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolSlotValFail> {
        let stats = self.get_stats_launched_fighters(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.fighters_online.iter().map(|v| *v).collect();
        Some(SolSlotValFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launched_support_fighters_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolSlotValFail> {
        let stats = self.get_stats_launched_support_fighters(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.support_fighters_online.iter().map(|v| *v).collect();
        Some(SolSlotValFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launched_light_fighters_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolSlotValFail> {
        let stats = self.get_stats_launched_light_fighters(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.light_fighters_online.iter().map(|v| *v).collect();
        Some(SolSlotValFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launched_heavy_fighters_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolSlotValFail> {
        let stats = self.get_stats_launched_heavy_fighters(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.heavy_fighters_online.iter().map(|v| *v).collect();
        Some(SolSlotValFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_support_fighters_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolSlotValFail> {
        let stats = self.get_stats_launched_standup_support_fighters(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.standup_support_fighters_online.iter().map(|v| *v).collect();
        Some(SolSlotValFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_light_fighters_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolSlotValFail> {
        let stats = self.get_stats_launched_standup_light_fighters(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.standup_light_fighters_online.iter().map(|v| *v).collect();
        Some(SolSlotValFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_heavy_fighters_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolSlotValFail> {
        let stats = self.get_stats_launched_standup_heavy_fighters(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.standup_heavy_fighters_online.iter().map(|v| *v).collect();
        Some(SolSlotValFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_turret_slots_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolSlotValFail> {
        let stats = self.get_stats_turret_slots(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.mods_turret.iter().map(|v| *v).collect();
        Some(SolSlotValFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_launcher_slots_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolSlotValFail> {
        let stats = self.get_stats_launcher_slots(uad, calc, fit);
        if stats.used <= stats.total.unwrap_or(0) {
            return None;
        }
        let users = self.mods_launcher.iter().map(|v| *v).collect();
        Some(SolSlotValFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_high_slots_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolSlotValFail> {
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
        Some(SolSlotValFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_mid_slots_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolSlotValFail> {
        let stats = self.get_stats_mid_slots(uad, calc, fit);
        let total = stats.total.unwrap_or(0);
        if stats.used <= total {
            return None;
        }
        let users = match total >= fit.mods_mid.len() as Count {
            true => Vec::new(),
            false => fit.mods_mid.inner()[total as Idx..].iter().filter_map(|v| *v).collect(),
        };
        Some(SolSlotValFail::new(stats.used, stats.total, users))
    }
    pub(in crate::sol::svc::vast) fn validate_low_slots_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolSlotValFail> {
        let stats = self.get_stats_low_slots(uad, calc, fit);
        let total = stats.total.unwrap_or(0);
        if stats.used <= total {
            return None;
        }
        let users = match total >= fit.mods_low.len() as Count {
            true => Vec::new(),
            false => fit.mods_low.inner()[total as Idx..].iter().filter_map(|v| *v).collect(),
        };
        Some(SolSlotValFail::new(stats.used, stats.total, users))
    }
}
