use crate::{
    def::DefCount,
    svc::{
        SvcCtx,
        calc::Calc,
        vast::{VastFitData, shared::get_attr_as_count},
    },
    ud::UFit,
};

pub struct StatSlot {
    pub used: DefCount,
    pub total: Option<DefCount>,
}

impl VastFitData {
    // Modules
    pub(in crate::svc) fn get_stat_high_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = fit.mods_high.len() as DefCount;
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().hi_slots);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_mid_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = fit.mods_mid.len() as DefCount;
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().med_slots);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_low_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = fit.mods_low.len() as DefCount;
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().low_slots);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_turret_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = self.mods_turret.len() as DefCount;
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().turret_slots_left);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launcher_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = self.mods_launcher.len() as DefCount;
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().launcher_slots_left);
        StatSlot { used, total }
    }
    // Rigs
    pub(in crate::svc) fn get_stat_rig_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = fit.rigs.len() as DefCount;
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().upgrade_slots_left);
        StatSlot { used, total }
    }
    // Service
    pub(in crate::svc) fn get_stat_service_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = fit.services.len() as DefCount;
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().service_slots);
        StatSlot { used, total }
    }
    // Subsystems
    pub(in crate::svc) fn get_stat_subsystem_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = fit.subsystems.len() as DefCount;
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().max_subsystems);
        StatSlot { used, total }
    }
    // Drones
    pub(in crate::svc) fn get_stat_launched_drones(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = self.get_launched_drone_count();
        let total = get_attr_as_count(ctx, calc, fit.character, ctx.ac().max_active_drones);
        StatSlot { used, total }
    }
    pub(in crate::svc::vast) fn get_launched_drone_count(&self) -> DefCount {
        self.drones_online_bandwidth.len() as DefCount
    }
    // Fighters
    pub(in crate::svc) fn get_stat_launched_fighters(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = self.fighters_online.len() as DefCount;
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().ftr_tubes);
        StatSlot { used, total }
    }
    pub(in crate::svc::vast) fn get_launched_fighter_count(&self) -> DefCount {
        self.fighters_online.len() as DefCount
    }
    pub(in crate::svc) fn get_stat_launched_light_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = self.light_fighters_online.len() as DefCount;
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().ftr_light_slots);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launched_heavy_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = self.heavy_fighters_online.len() as DefCount;
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().ftr_heavy_slots);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launched_support_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = self.support_fighters_online.len() as DefCount;
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().ftr_support_slots);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launched_st_light_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = self.st_light_fighters_online.len() as DefCount;
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().ftr_st_light_slots);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launched_st_heavy_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = self.st_heavy_fighters_online.len() as DefCount;
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().ftr_st_heavy_slots);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launched_st_support_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = self.st_support_fighters_online.len() as DefCount;
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().ftr_st_support_slots);
        StatSlot { used, total }
    }
}
