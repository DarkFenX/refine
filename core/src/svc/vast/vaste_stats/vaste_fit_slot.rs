use crate::{
    misc::Count,
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
        let used = Count::from_usize(fit.mods_high.len());
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().hi_slots);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_mid_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = Count::from_usize(fit.mods_mid.len());
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().med_slots);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_low_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = Count::from_usize(fit.mods_low.len());
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().low_slots);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_turret_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = Count::from_usize(self.mods_turret.len());
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().turret_slots_left);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launcher_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = Count::from_usize(self.mods_launcher.len());
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().launcher_slots_left);
        StatSlot { used, total }
    }
    // Rigs
    pub(in crate::svc) fn get_stat_rig_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = Count::from_usize(fit.rigs.len());
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().upgrade_slots_left);
        StatSlot { used, total }
    }
    // Service
    pub(in crate::svc) fn get_stat_service_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = Count::from_usize(fit.services.len());
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().service_slots);
        StatSlot { used, total }
    }
    // Subsystems
    pub(in crate::svc) fn get_stat_subsystem_slots(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = Count::from_usize(fit.subsystems.len());
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().max_subsystems);
        StatSlot { used, total }
    }
    // Drones
    pub(in crate::svc) fn get_stat_launched_drones(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = self.get_launched_drone_count();
        let total = get_attr_as_count(ctx, calc, fit.character, ctx.ac().max_active_drones);
        StatSlot { used, total }
    }
    pub(in crate::svc::vast) fn get_launched_drone_count(&self) -> Count {
        Count::from_usize(self.drones_online_bandwidth.len())
    }
    // Fighters
    pub(in crate::svc) fn get_stat_launched_fighters(&self, ctx: SvcCtx, calc: &mut Calc, fit: &UFit) -> StatSlot {
        let used = Count::from_usize(self.fighters_online.len());
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().ftr_tubes);
        StatSlot { used, total }
    }
    pub(in crate::svc::vast) fn get_launched_fighter_count(&self) -> Count {
        Count::from_usize(self.fighters_online.len())
    }
    pub(in crate::svc) fn get_stat_launched_light_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = Count::from_usize(self.light_fighters_online.len());
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().ftr_light_slots);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launched_heavy_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = Count::from_usize(self.heavy_fighters_online.len());
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().ftr_heavy_slots);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launched_support_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = Count::from_usize(self.support_fighters_online.len());
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().ftr_support_slots);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launched_st_light_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = Count::from_usize(self.st_light_fighters_online.len());
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().ftr_st_light_slots);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launched_st_heavy_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = Count::from_usize(self.st_heavy_fighters_online.len());
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().ftr_st_heavy_slots);
        StatSlot { used, total }
    }
    pub(in crate::svc) fn get_stat_launched_st_support_fighters(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit: &UFit,
    ) -> StatSlot {
        let used = Count::from_usize(self.st_support_fighters_online.len());
        let total = get_attr_as_count(ctx, calc, fit.ship, ctx.ac().ftr_st_support_slots);
        StatSlot { used, total }
    }
}
