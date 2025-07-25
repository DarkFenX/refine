use crate::{
    ac,
    def::Count,
    svc::{
        SvcCtx,
        calc::Calc,
        vast::{VastFitData, shared::get_attr_as_count},
    },
    ud::UFit,
};

pub struct StatSlot {
    pub used: Count,
    pub total: Option<Count>,
}

impl VastFitData {
    // Modules
    pub(in crate::svc) fn get_stat_high_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = fit.mods_high.len() as Count;
        let total = get_attr_as_count(ctx, calc, fit.ship, &ac::attrs::HI_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_mid_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = fit.mods_mid.len() as Count;
        let total = get_attr_as_count(ctx, calc, fit.ship, &ac::attrs::MED_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_low_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = fit.mods_low.len() as Count;
        let total = get_attr_as_count(ctx, calc, fit.ship, &ac::attrs::LOW_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_turret_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = self.mods_turret.len() as Count;
        let total = get_attr_as_count(ctx, calc, fit.ship, &ac::attrs::TURRET_SLOTS_LEFT);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launcher_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = self.mods_launcher.len() as Count;
        let total = get_attr_as_count(ctx, calc, fit.ship, &ac::attrs::LAUNCHER_SLOTS_LEFT);
        StatSlot { used, total }
    }
    // Rigs
    pub(in crate::svc) fn get_stat_rig_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = fit.rigs.len() as Count;
        let total = get_attr_as_count(ctx, calc, fit.ship, &ac::attrs::UPGRADE_SLOTS_LEFT);
        StatSlot { used, total }
    }
    // Service
    pub(in crate::svc) fn get_stat_service_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = fit.services.len() as Count;
        let total = get_attr_as_count(ctx, calc, fit.ship, &ac::attrs::SERVICE_SLOTS);
        StatSlot { used, total }
    }
    // Subsystems
    pub(in crate::svc) fn get_stat_subsystem_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = fit.subsystems.len() as Count;
        let total = get_attr_as_count(ctx, calc, fit.ship, &ac::attrs::MAX_SUBSYSTEMS);
        StatSlot { used, total }
    }
    // Drones
    pub(in crate::svc) fn get_stat_launched_drones(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = self.drones_online_bandwidth.len() as Count;
        let total = get_attr_as_count(ctx, calc, fit.character, &ac::attrs::MAX_ACTIVE_DRONES);
        StatSlot { used, total }
    }
    // Fighters
    pub(in crate::svc) fn get_stat_launched_fighters(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = self.fighters_online.len() as Count;
        let total = get_attr_as_count(ctx, calc, fit.ship, &ac::attrs::FTR_TUBES);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launched_light_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = self.light_fighters_online.len() as Count;
        let total = get_attr_as_count(ctx, calc, fit.ship, &ac::attrs::FTR_LIGHT_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launched_heavy_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = self.heavy_fighters_online.len() as Count;
        let total = get_attr_as_count(ctx, calc, fit.ship, &ac::attrs::FTR_HEAVY_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launched_support_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = self.support_fighters_online.len() as Count;
        let total = get_attr_as_count(ctx, calc, fit.ship, &ac::attrs::FTR_SUPPORT_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launched_st_light_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = self.st_light_fighters_online.len() as Count;
        let total = get_attr_as_count(ctx, calc, fit.ship, &ac::attrs::FTR_ST_LIGHT_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launched_st_heavy_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = self.st_heavy_fighters_online.len() as Count;
        let total = get_attr_as_count(ctx, calc, fit.ship, &ac::attrs::FTR_ST_HEAVY_SLOTS);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launched_st_support_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = self.st_support_fighters_online.len() as Count;
        let total = get_attr_as_count(ctx, calc, fit.ship, &ac::attrs::FTR_ST_SUPPORT_SLOTS);
        StatSlot { used, total }
    }
}
