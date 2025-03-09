use itertools::Itertools;

use crate::{
    defs::{Count, Idx, SolItemId},
    sol::{
        svc::{
            calc::SolCalc,
            vast::{SolStatSlot, SolVastFitData},
        },
        uad::{
            SolUad,
            fit::{SolFit, SolItemVec},
        },
    },
    util::StSet,
};

pub struct SolValSlotCountFail {
    pub used: Count,
    pub total: Option<Count>,
    pub users: Vec<SolItemId>,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_rig_slot_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        if fit.rigs.is_subset(kfs) {
            return true;
        }
        let stats = self.get_stats_rig_slots(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        let stats = self.get_stats_subsystem_slots(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_drone_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        let stats = self.get_stats_launched_drones(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_fighter_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        let stats = self.get_stats_launched_fighters(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_support_fighter_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        let stats = self.get_stats_launched_support_fighters(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_light_fighter_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        let stats = self.get_stats_launched_light_fighters(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_heavy_fighter_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        let stats = self.get_stats_launched_heavy_fighters(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_support_fighter_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        let stats = self.get_stats_launched_standup_support_fighters(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_light_fighter_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        let stats = self.get_stats_launched_standup_light_fighters(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_heavy_fighter_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        let stats = self.get_stats_launched_standup_heavy_fighters(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_turret_slot_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        let stats = self.get_stats_turret_slots(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_launcher_slot_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        let stats = self.get_stats_launcher_slots(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_high_slot_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        let stats = self.get_stats_high_slots(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_mid_slot_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        let stats = self.get_stats_mid_slots(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_low_slot_count_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        let stats = self.get_stats_low_slots(uad, calc, fit);
        validate_fast(stats)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_rig_slot_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_rig_slots(uad, calc, fit);
        validate_verbose_unordered(kfs, stats, &fit.rigs)
    }
    pub(in crate::sol::svc::vast) fn validate_subsystem_slot_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_subsystem_slots(uad, calc, fit);
        validate_verbose_unordered_old(kfs, stats, fit.subsystems.iter())
    }
    pub(in crate::sol::svc::vast) fn validate_launched_drone_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launched_drones(uad, calc, fit);
        validate_verbose_unordered_old(kfs, stats, self.drones_online_bandwidth.keys())
    }
    pub(in crate::sol::svc::vast) fn validate_launched_fighter_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launched_fighters(uad, calc, fit);
        validate_verbose_unordered_old(kfs, stats, self.fighters_online.iter())
    }
    pub(in crate::sol::svc::vast) fn validate_launched_support_fighter_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launched_support_fighters(uad, calc, fit);
        validate_verbose_unordered_old(kfs, stats, self.support_fighters_online.iter())
    }
    pub(in crate::sol::svc::vast) fn validate_launched_light_fighter_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launched_light_fighters(uad, calc, fit);
        validate_verbose_unordered_old(kfs, stats, self.light_fighters_online.iter())
    }
    pub(in crate::sol::svc::vast) fn validate_launched_heavy_fighter_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launched_heavy_fighters(uad, calc, fit);
        validate_verbose_unordered_old(kfs, stats, self.heavy_fighters_online.iter())
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_support_fighter_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launched_standup_support_fighters(uad, calc, fit);
        validate_verbose_unordered_old(kfs, stats, self.standup_support_fighters_online.iter())
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_light_fighter_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launched_standup_light_fighters(uad, calc, fit);
        validate_verbose_unordered_old(kfs, stats, self.standup_light_fighters_online.iter())
    }
    pub(in crate::sol::svc::vast) fn validate_launched_standup_heavy_fighter_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launched_standup_heavy_fighters(uad, calc, fit);
        validate_verbose_unordered_old(kfs, stats, self.standup_heavy_fighters_online.iter())
    }
    pub(in crate::sol::svc::vast) fn validate_turret_slot_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_turret_slots(uad, calc, fit);
        validate_verbose_unordered_old(kfs, stats, self.mods_turret.iter())
    }
    pub(in crate::sol::svc::vast) fn validate_launcher_slot_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_launcher_slots(uad, calc, fit);
        validate_verbose_unordered_old(kfs, stats, self.mods_launcher.iter())
    }
    pub(in crate::sol::svc::vast) fn validate_high_slot_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_high_slots(uad, calc, fit);
        validate_verbose_ordered(kfs, stats, &fit.mods_high)
    }
    pub(in crate::sol::svc::vast) fn validate_mid_slot_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_mid_slots(uad, calc, fit);
        validate_verbose_ordered(kfs, stats, &fit.mods_mid)
    }
    pub(in crate::sol::svc::vast) fn validate_low_slot_count_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValSlotCountFail> {
        let stats = self.get_stats_low_slots(uad, calc, fit);
        validate_verbose_ordered(kfs, stats, &fit.mods_low)
    }
}

fn validate_fast(stats: SolStatSlot) -> bool {
    stats.used <= stats.total.unwrap_or(0)
}
fn validate_verbose_ordered<'a>(
    kfs: &StSet<SolItemId>,
    stats: SolStatSlot,
    users: &SolItemVec,
) -> Option<SolValSlotCountFail> {
    let total = stats.total.unwrap_or(0);
    if stats.used <= total {
        return None;
    }
    let users = match total >= users.len() as Count {
        true => Vec::new(),
        false => users.inner()[total as Idx..].iter().filter_map(|v| *v).collect(),
    };
    Some(SolValSlotCountFail {
        used: stats.used,
        total: stats.total,
        users,
    })
}
fn validate_verbose_unordered<'a>(
    kfs: &StSet<SolItemId>,
    stats: SolStatSlot,
    users: &StSet<SolItemId>,
) -> Option<SolValSlotCountFail> {
    if stats.used <= stats.total.unwrap_or(0) {
        return None;
    }
    let users = users.difference(kfs).copied().collect_vec();
    if users.is_empty() {
        return None;
    }
    Some(SolValSlotCountFail {
        used: stats.used,
        total: stats.total,
        users,
    })
}
fn validate_verbose_unordered_old<'a>(
    kfs: &StSet<SolItemId>,
    stats: SolStatSlot,
    users: impl Iterator<Item = &'a SolItemId>,
) -> Option<SolValSlotCountFail> {
    if stats.used <= stats.total.unwrap_or(0) {
        return None;
    }
    let users = users.filter(|v| !kfs.contains(v)).copied().collect_vec();
    if users.is_empty() {
        return None;
    }
    Some(SolValSlotCountFail {
        used: stats.used,
        total: stats.total,
        users,
    })
}
