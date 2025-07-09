use crate::{
    def::FitKey,
    sol::REffs,
    svc::{
        SvcCtx,
        calc::Calc,
        vast::{ValOptionsInt, ValOptionsSolInt, ValResultFit, ValResultSol, Vast},
    },
};

impl Vast {
    pub(in crate::svc) fn validate_sol_fast(
        &mut self,
        ctx: SvcCtx,
        calc: &mut Calc,
        reffs: &REffs,
        options: &ValOptionsSolInt,
    ) -> bool {
        for &fit_key in options.fit_keys.iter() {
            if !self.validate_fit_fast(ctx, calc, reffs, fit_key, &options.options) {
                return false;
            }
        }
        if options.options.not_loaded_item.enabled
            && !self.validate_not_loaded_item_fast(&options.options.not_loaded_item.kfs)
        {
            return false;
        }
        true
    }
    pub(in crate::svc) fn validate_sol_verbose(
        &mut self,
        ctx: SvcCtx,
        calc: &mut Calc,
        reffs: &REffs,
        options: &ValOptionsSolInt,
    ) -> ValResultSol {
        let mut sol_result = ValResultSol::new();
        for &fit_key in options.fit_keys.iter() {
            let fit_result = self.validate_fit_verbose(ctx, calc, reffs, fit_key, &options.options);
            if !fit_result.all_passed() {
                let fit_id = ctx.uad.fits.id_by_key(fit_key);
                sol_result.fits.insert(fit_id, fit_result);
            }
        }
        if options.options.not_loaded_item.enabled {
            sol_result.not_loaded_item =
                self.validate_not_loaded_item_verbose(&options.options.not_loaded_item.kfs, ctx);
        }
        sol_result
    }
    pub(in crate::svc) fn validate_fit_fast(
        &mut self,
        ctx: SvcCtx,
        calc: &mut Calc,
        reffs: &REffs,
        fit_key: FitKey,
        options: &ValOptionsInt,
    ) -> bool {
        let fit = ctx.uad.fits.get(fit_key);
        let fit_data = self.get_fit_data_mut(&fit_key);
        let ship = fit.ship.map(|v| ctx.uad.items.get(v).get_ship().unwrap());
        // Order of validations matters here; the faster validation and the more likely it is to
        // fail, the closer to top it should be. This order was chosen to optimize for market
        // filtering capabilities, which takes into account following item distribution:
        // - modules 3249
        // - implants 834
        // - rigs 817
        // - boosters 144
        // - drones 125
        // - fighters 94
        // - subsystems 48
        // - services 16
        // Cheap generic check which applies to various item types, even if not universally
        // applicable.
        if options.skill_reqs.enabled && !fit_data.validate_skill_reqs_fast(&options.skill_reqs.kfs) {
            return false;
        }
        // Very cheap check which prevents using big groups of modules/rigs on wrong kind of ship
        if options.item_vs_ship_kind.enabled
            && !fit_data.validate_item_vs_ship_kind_fast(&options.item_vs_ship_kind.kfs)
        {
            return false;
        }
        // Cheap module validations are close to the top as well. The only expensive operation is
        // grabbing modified slot count from ship.
        if options.high_slot_count.enabled
            && !fit_data.validate_high_slot_count_fast(&options.high_slot_count.kfs, ctx, calc, fit)
        {
            return false;
        }
        if options.mid_slot_count.enabled
            && !fit_data.validate_mid_slot_count_fast(&options.mid_slot_count.kfs, ctx, calc, fit)
        {
            return false;
        }
        if options.low_slot_count.enabled
            && !fit_data.validate_low_slot_count_fast(&options.low_slot_count.kfs, ctx, calc, fit)
        {
            return false;
        }
        if options.turret_slot_count.enabled
            && !fit_data.validate_turret_slot_count_fast(&options.turret_slot_count.kfs, ctx, calc, fit)
        {
            return false;
        }
        if options.launcher_slot_count.enabled
            && !fit_data.validate_launcher_slot_count_fast(&options.launcher_slot_count.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Relatively expensive check, but price scales with amount of limited items
        if options.ship_limit.enabled && !fit_data.validate_ship_limit_fast(&options.ship_limit.kfs, ship) {
            return false;
        }
        // A group of checks which isn't too cheap to run, but scales with amount of limited items,
        // and there are quite a few items with those limits.
        if options.max_group_fitted.enabled
            && !fit_data.validate_max_group_fitted_fast(&options.max_group_fitted.kfs, ctx, calc)
        {
            return false;
        }
        if options.max_group_online.enabled
            && !fit_data.validate_max_group_online_fast(&options.max_group_online.kfs, ctx, calc)
        {
            return false;
        }
        if options.max_group_active.enabled
            && !fit_data.validate_max_group_active_fast(&options.max_group_active.kfs, ctx, calc)
        {
            return false;
        }
        // Cheap module check, but only one module uses it at the moment (rorq's PANIC)
        if options.max_type_fitted.enabled && !fit_data.validate_max_type_fitted_fast(&options.max_type_fitted.kfs) {
            return false;
        }
        // Niche but very cheap. Does not allow to fit cap mods to subcaps, filters out some modules
        // before more expensive PG check.
        if options.capital_module.enabled && !fit_data.validate_capital_module_fast(&options.capital_module.kfs, ship) {
            return false;
        }
        // Cheap, but somewhat useless for "try fit" functionality check, since modules are added in
        // online state.
        if options.module_state.enabled && !fit_data.validate_module_state_fast(&options.module_state.kfs) {
            return false;
        }
        // Rigs - cheap slot validation first, then size which is likely to fail (~3/4th of rigs can
        // not be fit to a ship), then calibration which is expensive and not very likely to fail
        if options.rig_slot_count.enabled
            && !fit_data.validate_rig_slot_count_fast(&options.rig_slot_count.kfs, ctx, calc, fit)
        {
            return false;
        }
        if options.rig_size.enabled && !fit_data.validate_rig_size_fast(&options.rig_size.kfs, ship) {
            return false;
        }
        if options.calibration.enabled && !fit_data.validate_calibration_fast(&options.calibration.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Implants - lots of implants, but validation is not likely to fail (need implant slots
        // filled for it to do so), so it's pushed down a bit
        if options.implant_slot_index.enabled
            && !fit_data.validate_implant_slot_index_fast(&options.implant_slot_index.kfs)
        {
            return false;
        }
        // Very expensive resource checks related to modules/services. PG over CPU since it is more
        // likely to break validation (modules of bigger sizes usually instantly take more PG than a
        // ship provides)
        if options.powergrid.enabled && !fit_data.validate_powergrid_fast(&options.powergrid.kfs, ctx, calc, fit) {
            return false;
        }
        if options.cpu.enabled && !fit_data.validate_cpu_fast(&options.cpu.kfs, ctx, calc, fit) {
            return false;
        }
        // Drones
        if options.drone_bay_volume.enabled
            && !fit_data.validate_drone_bay_volume_fast(&options.drone_bay_volume.kfs, ctx, calc, fit)
        {
            return false;
        }
        if options.unlaunchable_drone_bandwidth.enabled
            && !fit_data.validate_unlaunchable_drone_bandwidth_fast(
                &options.unlaunchable_drone_bandwidth.kfs,
                ctx,
                calc,
                fit,
            )
        {
            return false;
        }
        // Unlikely to fail, since drones are not added in in-space+ state
        if options.drone_bandwidth.enabled
            && !fit_data.validate_drone_bandwidth_fast(&options.drone_bandwidth.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Unlikely to fail, since drones are not added in in-space+ state
        if options.launched_drone_count.enabled
            && !fit_data.validate_launched_drone_count_fast(&options.launched_drone_count.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Fighters
        // Volume goes first - since it's as cheap as unlaunchable fighter, but can also fail on a
        // carrier fit.
        if options.fighter_bay_volume.enabled
            && !fit_data.validate_fighter_bay_volume_fast(&options.fighter_bay_volume.kfs, ctx, calc, fit)
        {
            return false;
        }
        if options.unlaunchable_fighter.enabled
            && !fit_data.validate_unlaunchable_fighter_fast(&options.unlaunchable_fighter.kfs, ctx, calc, fit)
        {
            return false;
        }
        if options.unlaunchable_light_fighter.enabled
            && !fit_data.validate_unlaunchable_light_fighter_fast(
                &options.unlaunchable_light_fighter.kfs,
                ctx,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.unlaunchable_heavy_fighter.enabled
            && !fit_data.validate_unlaunchable_heavy_fighter_fast(
                &options.unlaunchable_heavy_fighter.kfs,
                ctx,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.unlaunchable_support_fighter.enabled
            && !fit_data.validate_unlaunchable_support_fighter_fast(
                &options.unlaunchable_support_fighter.kfs,
                ctx,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.unlaunchable_st_light_fighter.enabled
            && !fit_data.validate_unlaunchable_st_light_fighter_fast(
                &options.unlaunchable_st_light_fighter.kfs,
                ctx,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.unlaunchable_st_heavy_fighter.enabled
            && !fit_data.validate_unlaunchable_st_heavy_fighter_fast(
                &options.unlaunchable_st_heavy_fighter.kfs,
                ctx,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.unlaunchable_st_support_fighter.enabled
            && !fit_data.validate_unlaunchable_st_support_fighter_fast(
                &options.unlaunchable_st_support_fighter.kfs,
                ctx,
                calc,
                fit,
            )
        {
            return false;
        }
        // Launched go after launchable, since they are less likely to fail due to fighter state
        // condition.
        if options.launched_fighter_count.enabled
            && !fit_data.validate_launched_fighter_count_fast(&options.launched_fighter_count.kfs, ctx, calc, fit)
        {
            return false;
        }
        if options.launched_light_fighter_count.enabled
            && !fit_data.validate_launched_light_fighter_count_fast(
                &options.launched_light_fighter_count.kfs,
                ctx,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.launched_heavy_fighter_count.enabled
            && !fit_data.validate_launched_heavy_fighter_count_fast(
                &options.launched_heavy_fighter_count.kfs,
                ctx,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.launched_support_fighter_count.enabled
            && !fit_data.validate_launched_support_fighter_count_fast(
                &options.launched_support_fighter_count.kfs,
                ctx,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.launched_st_light_fighter_count.enabled
            && !fit_data.validate_launched_st_light_fighter_count_fast(
                &options.launched_st_light_fighter_count.kfs,
                ctx,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.launched_st_heavy_fighter_count.enabled
            && !fit_data.validate_launched_st_heavy_fighter_count_fast(
                &options.launched_st_heavy_fighter_count.kfs,
                ctx,
                calc,
                fit,
            )
        {
            return false;
        }
        if options.launched_st_support_fighter_count.enabled
            && !fit_data.validate_launched_st_support_fighter_count_fast(
                &options.launched_st_support_fighter_count.kfs,
                ctx,
                calc,
                fit,
            )
        {
            return false;
        }
        // Very niche, since fighter count has to be overridden to a value higher than squad
        // supports.
        if options.fighter_squad_size.enabled
            && !fit_data.validate_fighter_squad_size_fast(&options.fighter_squad_size.kfs)
        {
            return false;
        }
        // Boosters are below drones and fighters because they are not likely to fail, despite being
        // more numerous item category
        if options.booster_slot_index.enabled
            && !fit_data.validate_booster_slot_index_fast(&options.booster_slot_index.kfs)
        {
            return false;
        }
        // Depends on some incoming projections or system/fit-wide effects, but can fail for some
        // modules in those conditions (e.g. MWD under ESS bubble effect).
        if options.activation_blocked.enabled
            && !fit_data.validate_activation_blocked_fast(&options.activation_blocked.kfs, ctx, calc)
        {
            return false;
        }
        // Subsystems - very few subsystems, unlikely to fail
        if options.subsystem_slot_index.enabled
            && !fit_data.validate_subsystem_slot_index_fast(&options.subsystem_slot_index.kfs)
        {
            return false;
        }
        if options.subsystem_slot_count.enabled
            && !fit_data.validate_subsystem_slot_count_fast(&options.subsystem_slot_count.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Services - very few services, applicable only to citadels, which usually do not have all
        // slots filled anyway
        if options.service_slot_count.enabled
            && !fit_data.validate_service_slot_count_fast(&options.service_slot_count.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Security zone-specific checks. Usually should pass, since expectation is to have fit in
        // nullsec, which has no sec zone limits, at least for now.
        if options.sec_zone_fitted.enabled
            && !fit_data.validate_sec_zone_fitted_fast(&options.sec_zone_fitted.kfs, ctx, calc)
        {
            return false;
        }
        if options.sec_zone_online.enabled && !fit_data.validate_sec_zone_online_fast(&options.sec_zone_online.kfs, ctx)
        {
            return false;
        }
        if options.sec_zone_active.enabled
            && !fit_data.validate_sec_zone_active_fast(&options.sec_zone_active.kfs, ctx, calc)
        {
            return false;
        }
        if options.sec_zone_unonlineable.enabled
            && !fit_data.validate_sec_zone_unonlineable_fast(&options.sec_zone_unonlineable.kfs, ctx)
        {
            return false;
        }
        if options.sec_zone_unactivable.enabled
            && !fit_data.validate_sec_zone_unactivable_fast(&options.sec_zone_unactivable.kfs, ctx, calc)
        {
            return false;
        }
        // Incoming projection - effect stopper shouldn't fail for tried items, since there are no
        // indirect ways to stop item effects for now.
        if options.effect_stopper.enabled
            && !fit_data.validate_effect_stopper_fast(&options.effect_stopper.kfs, ctx, calc, reffs)
        {
            return false;
        }
        // Outgoing projections - useless for try-fit functionality, since tried items do not get
        // outgoing projections added.
        if options.assist_immunity.enabled
            && !fit_data.validate_assist_immunity_fast(&options.assist_immunity.kfs, ctx, calc)
        {
            return false;
        }
        if options.offense_immunity.enabled
            && !fit_data.validate_offense_immunity_fast(&options.offense_immunity.kfs, ctx, calc)
        {
            return false;
        }
        if options.resist_immunity.enabled
            && !fit_data.validate_resist_immunity_fast(&options.resist_immunity.kfs, ctx, calc)
        {
            return false;
        }
        // Misc checks - rarely used, or unlikely to fail
        // Charge checks are not related to fit optimizations so far
        if options.charge_group.enabled && !fit_data.validate_charge_group_fast(&options.charge_group.kfs) {
            return false;
        }
        if options.charge_size.enabled && !fit_data.validate_charge_size_fast(&options.charge_size.kfs) {
            return false;
        }
        if options.charge_volume.enabled && !fit_data.validate_charge_volume_fast(&options.charge_volume.kfs) {
            return false;
        }
        // Majority of fits are supposed to have thermodynamics 1 trained, and not every fit has
        // overloaded modules.
        if options.overload_skill.enabled && !fit_data.validate_overload_skill_fast(&options.overload_skill.kfs, fit) {
            return false;
        }
        // T3D-specific check which should pass if nothing goes wrong on the app side
        if options.ship_stance.enabled && !fit_data.validate_ship_stance_fast(&options.ship_stance.kfs, fit, ship) {
            return false;
        }
        // Happens only at drone skill 0, which is not something likely to see
        if options.unlaunchable_drone_slot.enabled
            && !fit_data.validate_unlaunchable_drone_slot_fast(&options.unlaunchable_drone_slot.kfs, ctx, calc, fit)
        {
            return false;
        }
        // In regular conditions, items kinds are supposed to match expected ones
        if options.item_kind.enabled && !fit_data.validate_item_kind_fast(&options.item_kind.kfs) {
            return false;
        }
        // In regular conditions, items are supposed to be loaded
        if options.not_loaded_item.enabled && !fit_data.validate_not_loaded_item_fast(&options.not_loaded_item.kfs) {
            return false;
        }
        // No known items use it, only fighter drones used to have it
        if options.drone_group.enabled && !fit_data.validate_drone_group_fast(&options.drone_group.kfs) {
            return false;
        }
        true
    }
    pub(in crate::svc) fn validate_fit_verbose(
        &mut self,
        ctx: SvcCtx,
        calc: &mut Calc,
        reffs: &REffs,
        fit_key: FitKey,
        options: &ValOptionsInt,
    ) -> ValResultFit {
        let fit = ctx.uad.fits.get(fit_key);
        let fit_data = self.get_fit_data_mut(&fit_key);
        let ship = fit.ship.map(|v| ctx.uad.items.get(v).get_ship().unwrap());
        let mut result = ValResultFit::new();
        // Generic
        if options.not_loaded_item.enabled {
            result.not_loaded_item = fit_data.validate_not_loaded_item_verbose(&options.not_loaded_item.kfs, ctx);
        }
        if options.item_kind.enabled {
            result.item_kind = fit_data.validate_item_kind_verbose(&options.item_kind.kfs, ctx);
        }
        if options.skill_reqs.enabled {
            result.skill_reqs = fit_data.validate_skill_reqs_verbose(&options.skill_reqs.kfs, ctx);
        }
        // Implants/boosters
        if options.implant_slot_index.enabled {
            result.implant_slot_index =
                fit_data.validate_implant_slot_index_verbose(&options.implant_slot_index.kfs, ctx);
        }
        if options.booster_slot_index.enabled {
            result.booster_slot_index =
                fit_data.validate_booster_slot_index_verbose(&options.booster_slot_index.kfs, ctx);
        }
        // Shared between mod-alike items
        if options.cpu.enabled {
            result.cpu = fit_data.validate_cpu_verbose(&options.cpu.kfs, ctx, calc, fit);
        }
        if options.powergrid.enabled {
            result.powergrid = fit_data.validate_powergrid_verbose(&options.powergrid.kfs, ctx, calc, fit);
        }
        if options.ship_limit.enabled {
            result.ship_limit = fit_data.validate_ship_limit_verbose(&options.ship_limit.kfs, ctx, ship);
        }
        if options.max_group_fitted.enabled {
            result.max_group_fitted =
                fit_data.validate_max_group_fitted_verbose(&options.max_group_fitted.kfs, ctx, calc);
        }
        if options.max_group_online.enabled {
            result.max_group_online =
                fit_data.validate_max_group_online_verbose(&options.max_group_online.kfs, ctx, calc);
        }
        if options.max_group_active.enabled {
            result.max_group_active =
                fit_data.validate_max_group_active_verbose(&options.max_group_active.kfs, ctx, calc);
        }
        if options.max_type_fitted.enabled {
            result.max_type_fitted = fit_data.validate_max_type_fitted_verbose(&options.max_type_fitted.kfs, ctx);
        }
        if options.item_vs_ship_kind.enabled {
            result.item_vs_ship_kind =
                fit_data.validate_item_vs_ship_kind_verbose(&options.item_vs_ship_kind.kfs, ctx, fit);
        }
        // Modules
        if options.high_slot_count.enabled {
            result.high_slot_count =
                fit_data.validate_high_slot_count_verbose(&options.high_slot_count.kfs, ctx, calc, fit);
        }
        if options.mid_slot_count.enabled {
            result.mid_slot_count =
                fit_data.validate_mid_slot_count_verbose(&options.mid_slot_count.kfs, ctx, calc, fit);
        }
        if options.low_slot_count.enabled {
            result.low_slot_count =
                fit_data.validate_low_slot_count_verbose(&options.low_slot_count.kfs, ctx, calc, fit);
        }
        if options.turret_slot_count.enabled {
            result.turret_slot_count =
                fit_data.validate_turret_slot_count_verbose(&options.turret_slot_count.kfs, ctx, calc, fit);
        }
        if options.launcher_slot_count.enabled {
            result.launcher_slot_count =
                fit_data.validate_launcher_slot_count_verbose(&options.launcher_slot_count.kfs, ctx, calc, fit);
        }
        if options.module_state.enabled {
            result.module_state = fit_data.validate_module_state_verbose(&options.module_state.kfs, ctx);
        }
        if options.capital_module.enabled {
            result.capital_module = fit_data.validate_capital_module_verbose(&options.capital_module.kfs, ctx, ship);
        }
        if options.overload_skill.enabled {
            result.overload_skill = fit_data.validate_overload_skill_verbose(&options.overload_skill.kfs, ctx, fit);
        }
        // Charges
        if options.charge_group.enabled {
            result.charge_group = fit_data.validate_charge_group_verbose(&options.charge_group.kfs, ctx);
        }
        if options.charge_size.enabled {
            result.charge_size = fit_data.validate_charge_size_verbose(&options.charge_size.kfs, ctx);
        }
        if options.charge_volume.enabled {
            result.charge_volume = fit_data.validate_charge_volume_verbose(&options.charge_volume.kfs, ctx);
        }
        // Rigs
        if options.rig_slot_count.enabled {
            result.rig_slot_count =
                fit_data.validate_rig_slot_count_verbose(&options.rig_slot_count.kfs, ctx, calc, fit);
        }
        if options.calibration.enabled {
            result.calibration = fit_data.validate_calibration_verbose(&options.calibration.kfs, ctx, calc, fit);
        }
        if options.rig_size.enabled {
            result.rig_size = fit_data.validate_rig_size_verbose(&options.rig_size.kfs, ctx, ship);
        }
        // Services
        if options.service_slot_count.enabled {
            result.service_slot_count =
                fit_data.validate_service_slot_count_verbose(&options.service_slot_count.kfs, ctx, calc, fit);
        }
        // T3 subsystems/stances
        if options.subsystem_slot_count.enabled {
            result.subsystem_slot_count =
                fit_data.validate_subsystem_slot_count_verbose(&options.subsystem_slot_count.kfs, ctx, calc, fit);
        }
        if options.subsystem_slot_index.enabled {
            result.subsystem_slot_index =
                fit_data.validate_subsystem_slot_index_verbose(&options.subsystem_slot_index.kfs, ctx);
        }
        if options.ship_stance.enabled {
            result.ship_stance = fit_data.validate_ship_stance_verbose(&options.ship_stance.kfs, ctx, fit, ship);
        }
        // Drones
        if options.drone_bay_volume.enabled {
            result.drone_bay_volume =
                fit_data.validate_drone_bay_volume_verbose(&options.drone_bay_volume.kfs, ctx, calc, fit);
        }
        if options.launched_drone_count.enabled {
            result.launched_drone_count =
                fit_data.validate_launched_drone_count_verbose(&options.launched_drone_count.kfs, ctx, calc, fit);
        }
        if options.drone_bandwidth.enabled {
            result.drone_bandwidth =
                fit_data.validate_drone_bandwidth_verbose(&options.drone_bandwidth.kfs, ctx, calc, fit);
        }
        if options.unlaunchable_drone_slot.enabled {
            result.unlaunchable_drone_slot =
                fit_data.validate_unlaunchable_drone_slot_verbose(&options.unlaunchable_drone_slot.kfs, ctx, calc, fit);
        }
        if options.unlaunchable_drone_bandwidth.enabled {
            result.unlaunchable_drone_bandwidth = fit_data.validate_unlaunchable_drone_bandwidth_verbose(
                &options.unlaunchable_drone_bandwidth.kfs,
                ctx,
                calc,
                fit,
            );
        }
        if options.drone_group.enabled {
            result.drone_group = fit_data.validate_drone_group_verbose(&options.drone_group.kfs, ctx);
        }
        // Fighters
        if options.fighter_bay_volume.enabled {
            result.fighter_bay_volume =
                fit_data.validate_fighter_bay_volume_verbose(&options.fighter_bay_volume.kfs, ctx, calc, fit);
        }
        if options.launched_fighter_count.enabled {
            result.launched_fighter_count =
                fit_data.validate_launched_fighter_count_verbose(&options.launched_fighter_count.kfs, ctx, calc, fit);
        }
        if options.launched_light_fighter_count.enabled {
            result.launched_light_fighter_count = fit_data.validate_launched_light_fighter_count_verbose(
                &options.launched_light_fighter_count.kfs,
                ctx,
                calc,
                fit,
            );
        }
        if options.launched_heavy_fighter_count.enabled {
            result.launched_heavy_fighter_count = fit_data.validate_launched_heavy_fighter_count_verbose(
                &options.launched_heavy_fighter_count.kfs,
                ctx,
                calc,
                fit,
            );
        }
        if options.launched_support_fighter_count.enabled {
            result.launched_support_fighter_count = fit_data.validate_launched_support_fighter_count_verbose(
                &options.launched_support_fighter_count.kfs,
                ctx,
                calc,
                fit,
            );
        }
        if options.launched_st_light_fighter_count.enabled {
            result.launched_st_light_fighter_count = fit_data.validate_launched_st_light_fighter_count_verbose(
                &options.launched_st_light_fighter_count.kfs,
                ctx,
                calc,
                fit,
            );
        }
        if options.launched_st_heavy_fighter_count.enabled {
            result.launched_st_heavy_fighter_count = fit_data.validate_launched_st_heavy_fighter_count_verbose(
                &options.launched_st_heavy_fighter_count.kfs,
                ctx,
                calc,
                fit,
            );
        }
        if options.launched_st_support_fighter_count.enabled {
            result.launched_st_support_fighter_count = fit_data.validate_launched_st_support_fighter_count_verbose(
                &options.launched_st_support_fighter_count.kfs,
                ctx,
                calc,
                fit,
            );
        }
        if options.unlaunchable_fighter.enabled {
            result.unlaunchable_fighter =
                fit_data.validate_unlaunchable_fighter_verbose(&options.unlaunchable_fighter.kfs, ctx, calc, fit);
        }
        if options.unlaunchable_light_fighter.enabled {
            result.unlaunchable_light_fighter = fit_data.validate_unlaunchable_light_fighter_verbose(
                &options.unlaunchable_light_fighter.kfs,
                ctx,
                calc,
                fit,
            );
        }
        if options.unlaunchable_heavy_fighter.enabled {
            result.unlaunchable_heavy_fighter = fit_data.validate_unlaunchable_heavy_fighter_verbose(
                &options.unlaunchable_heavy_fighter.kfs,
                ctx,
                calc,
                fit,
            );
        }
        if options.unlaunchable_support_fighter.enabled {
            result.unlaunchable_support_fighter = fit_data.validate_unlaunchable_support_fighter_verbose(
                &options.unlaunchable_support_fighter.kfs,
                ctx,
                calc,
                fit,
            );
        }
        if options.unlaunchable_st_light_fighter.enabled {
            result.unlaunchable_st_light_fighter = fit_data.validate_unlaunchable_st_light_fighter_verbose(
                &options.unlaunchable_st_light_fighter.kfs,
                ctx,
                calc,
                fit,
            );
        }
        if options.unlaunchable_st_heavy_fighter.enabled {
            result.unlaunchable_st_heavy_fighter = fit_data.validate_unlaunchable_st_heavy_fighter_verbose(
                &options.unlaunchable_st_heavy_fighter.kfs,
                ctx,
                calc,
                fit,
            );
        }
        if options.unlaunchable_st_support_fighter.enabled {
            result.unlaunchable_st_support_fighter = fit_data.validate_unlaunchable_st_support_fighter_verbose(
                &options.unlaunchable_st_support_fighter.kfs,
                ctx,
                calc,
                fit,
            );
        }
        if options.fighter_squad_size.enabled {
            result.fighter_squad_size =
                fit_data.validate_fighter_squad_size_verbose(&options.fighter_squad_size.kfs, ctx);
        }
        // Projection, destination side
        if options.activation_blocked.enabled {
            result.activation_blocked =
                fit_data.validate_activation_blocked_verbose(&options.activation_blocked.kfs, ctx, calc);
        }
        if options.effect_stopper.enabled {
            result.effect_stopper =
                fit_data.validate_effect_stopper_verbose(&options.effect_stopper.kfs, ctx, calc, reffs);
        }
        // Projection, source side
        if options.assist_immunity.enabled {
            result.assist_immunity = fit_data.validate_assist_immunity_verbose(&options.assist_immunity.kfs, ctx, calc);
        }
        if options.offense_immunity.enabled {
            result.offense_immunity =
                fit_data.validate_offense_immunity_verbose(&options.offense_immunity.kfs, ctx, calc);
        }
        if options.resist_immunity.enabled {
            result.resist_immunity = fit_data.validate_resist_immunity_verbose(&options.resist_immunity.kfs, ctx, calc);
        }
        // Sec zone
        if options.sec_zone_fitted.enabled {
            result.sec_zone_fitted = fit_data.validate_sec_zone_fitted_verbose(&options.sec_zone_fitted.kfs, ctx, calc);
        }
        if options.sec_zone_online.enabled {
            result.sec_zone_online = fit_data.validate_sec_zone_online_verbose(&options.sec_zone_online.kfs, ctx);
        }
        if options.sec_zone_active.enabled {
            result.sec_zone_active = fit_data.validate_sec_zone_active_verbose(&options.sec_zone_active.kfs, ctx, calc);
        }
        if options.sec_zone_unonlineable.enabled {
            result.sec_zone_unonlineable =
                fit_data.validate_sec_zone_unonlineable_verbose(&options.sec_zone_unonlineable.kfs, ctx);
        }
        if options.sec_zone_unactivable.enabled {
            result.sec_zone_unactivable =
                fit_data.validate_sec_zone_unactivable_verbose(&options.sec_zone_unactivable.kfs, ctx, calc);
        }
        result
    }
}
