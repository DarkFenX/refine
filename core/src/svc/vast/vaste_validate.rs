use crate::{
    svc::{
        SvcCtx,
        calc::Calc,
        vast::{ValOptionInt, ValOptionsInt, ValOptionsSolInt, ValResultFit, ValResultSol, Vast},
    },
    ud::UFitId,
};

impl Vast {
    pub(in crate::svc) fn validate_sol_fast(
        &mut self,
        ctx: SvcCtx,
        calc: &mut Calc,
        options: &ValOptionsSolInt,
    ) -> bool {
        for &fit_uid in options.fit_uids.iter() {
            if !self.validate_fit_fast(ctx, calc, fit_uid, &options.options) {
                return false;
            }
        }
        if let ValOptionInt::Enabled(opts) = &options.options.not_loaded_item
            && !self.validate_not_loaded_item_fast(&opts.kfs)
        {
            return false;
        }
        true
    }
    pub(in crate::svc) fn validate_sol_verbose(
        &mut self,
        ctx: SvcCtx,
        calc: &mut Calc,
        options: &ValOptionsSolInt,
    ) -> ValResultSol {
        let mut sol_result = ValResultSol::new();
        for &fit_uid in options.fit_uids.iter() {
            let fit_result = self.validate_fit_verbose(ctx, calc, fit_uid, &options.options);
            if !fit_result.all_passed() {
                let fit_id = ctx.u_data.fits.xid_by_iid(fit_uid);
                sol_result.fits.insert(fit_id, fit_result);
            }
        }
        if let ValOptionInt::Enabled(opts) = &options.options.not_loaded_item {
            sol_result.not_loaded_item = self.validate_not_loaded_item_verbose(&opts.kfs, ctx);
        }
        sol_result
    }
    pub(in crate::svc) fn validate_fit_fast(
        &mut self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        options: &ValOptionsInt,
    ) -> bool {
        let fit = ctx.u_data.fits.get(fit_uid);
        let fit_data = self.get_fit_data_mut(&fit_uid);
        let ship = fit.ship.map(|v| ctx.u_data.items.get(v).dc_ship().unwrap());
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
        if let ValOptionInt::Enabled(opts) = &options.skill_reqs
            && !fit_data.validate_skill_reqs_fast(&opts.kfs)
        {
            return false;
        }
        // Very cheap check which prevents using big groups of modules/rigs on wrong kind of ship
        if let ValOptionInt::Enabled(opts) = &options.item_vs_ship_kind
            && !fit_data.validate_item_vs_ship_kind_fast(&opts.kfs)
        {
            return false;
        }
        // Cheap module validations are close to the top as well. The only expensive operation is
        // grabbing modified slot count from ship.
        if let ValOptionInt::Enabled(opts) = &options.high_slot_count
            && !fit_data.validate_high_slot_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.mid_slot_count
            && !fit_data.validate_mid_slot_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.low_slot_count
            && !fit_data.validate_low_slot_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.turret_slot_count
            && !fit_data.validate_turret_slot_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.launcher_slot_count
            && !fit_data.validate_launcher_slot_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Cheap checks related to charges; try-fit items functionality attempts to fit those now,
        // and quantity of charges is high, so those validations are close to the top
        if let ValOptionInt::Enabled(opts) = &options.charge_group
            && !fit_data.validate_charge_group_fast(&opts.kfs)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.charge_parent_group
            && !fit_data.validate_charge_cont_group_fast(&opts.kfs)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.charge_size
            && !fit_data.validate_charge_size_fast(&opts.kfs)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.charge_volume
            && !fit_data.validate_charge_volume_fast(&opts.kfs)
        {
            return false;
        }
        // Relatively expensive check, but cost scales with amount of limited items
        if let ValOptionInt::Enabled(opts) = &options.ship_limit
            && !fit_data.validate_ship_limit_fast(&opts.kfs, ship)
        {
            return false;
        }
        // A group of checks which isn't too cheap to run, but scales with amount of limited items,
        // and there are quite a few items with those limits.
        if let ValOptionInt::Enabled(opts) = &options.max_group_fitted
            && !fit_data.validate_max_group_fitted_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.max_group_online
            && !fit_data.validate_max_group_online_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.max_group_active
            && !fit_data.validate_max_group_active_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        // Cheap module check, but only one module uses it at the moment (rorq's PANIC)
        if let ValOptionInt::Enabled(opts) = &options.max_type_fitted
            && !fit_data.validate_max_type_fitted_fast(&opts.kfs)
        {
            return false;
        }
        // Niche but very cheap. Does not allow to fit cap mods to subcaps, filters out some modules
        // before more expensive PG check.
        if let ValOptionInt::Enabled(opts) = &options.capital_module
            && !fit_data.validate_capital_module_fast(&opts.kfs, ship)
        {
            return false;
        }
        // Cheap, but somewhat useless for "try fit" functionality check, since modules are added in
        // online state.
        if let ValOptionInt::Enabled(opts) = &options.module_state
            && !fit_data.validate_module_state_fast(&opts.kfs)
        {
            return false;
        }
        // Rigs - cheap slot validation first, then size which is likely to fail (~3/4th of rigs can
        // not be fit to a ship), then calibration which is expensive and not very likely to fail
        if let ValOptionInt::Enabled(opts) = &options.rig_slot_count
            && !fit_data.validate_rig_slot_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.rig_size
            && !fit_data.validate_rig_size_fast(&opts.kfs, ship)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.calibration
            && !fit_data.validate_calibration_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Implants - lots of implants, but validation is not likely to fail (need implant slots
        // filled for it to do so), so it's pushed down a bit
        if let ValOptionInt::Enabled(opts) = &options.implant_slot_index
            && !fit_data.validate_implant_slot_index_fast(&opts.kfs)
        {
            return false;
        }
        // Very expensive resource checks related to modules/services. PG over CPU since it is more
        // likely to break validation (modules of bigger sizes usually instantly take more PG than a
        // ship provides)
        if let ValOptionInt::Enabled(opts) = &options.powergrid
            && !fit_data.validate_powergrid_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.cpu
            && !fit_data.validate_cpu_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Drones
        if let ValOptionInt::Enabled(opts) = &options.drone_bay_volume
            && !fit_data.validate_drone_bay_volume_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_drone_bandwidth
            && !fit_data.validate_unlaunchable_drone_bandwidth_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Unlikely to fail, since drones are not added in in-space+ state
        if let ValOptionInt::Enabled(opts) = &options.drone_bandwidth
            && !fit_data.validate_drone_bandwidth_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Unlikely to fail, since drones are not added in in-space+ state
        if let ValOptionInt::Enabled(opts) = &options.launched_drone_count
            && !fit_data.validate_launched_drone_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Fighters
        // Volume goes first - since it's as cheap as unlaunchable fighter, but can also fail on a
        // carrier fit.
        if let ValOptionInt::Enabled(opts) = &options.fighter_bay_volume
            && !fit_data.validate_fighter_bay_volume_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_fighter
            && !fit_data.validate_unlaunchable_fighter_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_light_fighter
            && !fit_data.validate_unlaunchable_light_fighter_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_heavy_fighter
            && !fit_data.validate_unlaunchable_heavy_fighter_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_support_fighter
            && !fit_data.validate_unlaunchable_support_fighter_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_st_light_fighter
            && !fit_data.validate_unlaunchable_st_light_fighter_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_st_heavy_fighter
            && !fit_data.validate_unlaunchable_st_heavy_fighter_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_st_support_fighter
            && !fit_data.validate_unlaunchable_st_support_fighter_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Launched go after launchable, since they are less likely to fail due to fighter state
        // condition.
        if let ValOptionInt::Enabled(opts) = &options.launched_fighter_count
            && !fit_data.validate_launched_fighter_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.launched_light_fighter_count
            && !fit_data.validate_launched_light_fighter_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.launched_heavy_fighter_count
            && !fit_data.validate_launched_heavy_fighter_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.launched_support_fighter_count
            && !fit_data.validate_launched_support_fighter_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.launched_st_light_fighter_count
            && !fit_data.validate_launched_st_light_fighter_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.launched_st_heavy_fighter_count
            && !fit_data.validate_launched_st_heavy_fighter_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.launched_st_support_fighter_count
            && !fit_data.validate_launched_st_support_fighter_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Very niche, since fighter count has to be overridden to a value higher than squad
        // supports.
        if let ValOptionInt::Enabled(opts) = &options.fighter_squad_size
            && !fit_data.validate_fighter_squad_size_fast(&opts.kfs)
        {
            return false;
        }
        // Boosters are below drones and fighters because they are not likely to fail, despite being
        // more numerous item category
        if let ValOptionInt::Enabled(opts) = &options.booster_slot_index
            && !fit_data.validate_booster_slot_index_fast(&opts.kfs)
        {
            return false;
        }
        // Depends on some incoming projections or system/fit-wide effects, but can fail for some
        // modules in those conditions (e.g. MWD under ESS bubble effect).
        if let ValOptionInt::Enabled(opts) = &options.activation_blocked
            && !fit_data.validate_activation_blocked_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Subsystems - very few subsystems, unlikely to fail
        if let ValOptionInt::Enabled(opts) = &options.subsystem_slot_index
            && !fit_data.validate_subsystem_slot_index_fast(&opts.kfs)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.subsystem_slot_count
            && !fit_data.validate_subsystem_slot_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Services - very few services, applicable only to citadels, which usually do not have all
        // slots filled anyway
        if let ValOptionInt::Enabled(opts) = &options.service_slot_count
            && !fit_data.validate_service_slot_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Security zone-specific checks. Usually should pass, since expectation is to have fit in
        // nullsec, which has no sec zone limits, at least for now.
        if let ValOptionInt::Enabled(opts) = &options.sec_zone_fitted
            && !fit_data.validate_sec_zone_fitted_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.sec_zone_online
            && !fit_data.validate_sec_zone_online_fast(&opts.kfs, ctx)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.sec_zone_active
            && !fit_data.validate_sec_zone_active_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.sec_zone_unonlineable
            && !fit_data.validate_sec_zone_unonlineable_fast(&opts.kfs, ctx)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.sec_zone_unactivable
            && !fit_data.validate_sec_zone_unactivable_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.sec_zone_effect
            && !fit_data.validate_sec_zone_effect_fast(&opts.kfs, ctx)
        {
            return false;
        }
        // Incoming projection - effect stopper shouldn't fail for tried items, since there are no
        // indirect ways to stop item effects for now.
        if let ValOptionInt::Enabled(opts) = &options.effect_stopper
            && !fit_data.validate_effect_stopper_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        // Outgoing projections - useless for try-fit functionality, since tried items do not get
        // outgoing projections added.
        if let ValOptionInt::Enabled(opts) = &options.projectee_filter
            && !fit_data.validate_projectee_filter_fast(&opts.kfs, ctx)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.assist_immunity
            && !fit_data.validate_assist_immunity_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.offense_immunity
            && !fit_data.validate_offense_immunity_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = &options.resist_immunity
            && !fit_data.validate_resist_immunity_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        // Misc checks - rarely used, or unlikely to fail
        // Majority of fits are supposed to have thermodynamics 1 trained, and not every fit has
        // overloaded modules.
        if let ValOptionInt::Enabled(opts) = &options.overload_skill
            && !fit_data.validate_overload_skill_fast(&opts.kfs, fit)
        {
            return false;
        }
        // T3D-specific check which should pass if nothing goes wrong on the app side
        if let ValOptionInt::Enabled(opts) = &options.ship_stance
            && !fit_data.validate_ship_stance_fast(&opts.kfs, fit, ship)
        {
            return false;
        }
        // Happens only at drone skill 0, which is not something likely to see
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_drone_slot
            && !fit_data.validate_unlaunchable_drone_slot_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // In regular conditions, items kinds are supposed to match expected ones
        if let ValOptionInt::Enabled(opts) = &options.item_kind
            && !fit_data.validate_item_kind_fast(&opts.kfs)
        {
            return false;
        }
        // In regular conditions, items are supposed to be loaded
        if let ValOptionInt::Enabled(opts) = &options.not_loaded_item
            && !fit_data.validate_not_loaded_item_fast(&opts.kfs)
        {
            return false;
        }
        // Expensive check which rarely fails
        if let ValOptionInt::Enabled(opts) = &options.unusable_cap
            && !fit_data.validate_unusable_cap_fast(&opts.kfs, ctx, calc, fit.ship)
        {
            return false;
        }
        // No known items use it, only fighter drones used to have it
        if let ValOptionInt::Enabled(opts) = &options.drone_group
            && !fit_data.validate_drone_group_fast(&opts.kfs)
        {
            return false;
        }
        true
    }
    pub(in crate::svc) fn validate_fit_verbose(
        &mut self,
        ctx: SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        options: &ValOptionsInt,
    ) -> ValResultFit {
        let fit = ctx.u_data.fits.get(fit_uid);
        let fit_data = self.get_fit_data_mut(&fit_uid);
        let ship = fit.ship.map(|v| ctx.u_data.items.get(v).dc_ship().unwrap());
        let mut result = ValResultFit::new();
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Generic
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = &options.not_loaded_item {
            result.not_loaded_item = fit_data.validate_not_loaded_item_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = &options.item_kind {
            result.item_kind = fit_data.validate_item_kind_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = &options.skill_reqs {
            result.skill_reqs = fit_data.validate_skill_reqs_verbose(&opts.kfs, ctx);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Implants/boosters
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = &options.implant_slot_index {
            result.implant_slot_index = fit_data.validate_implant_slot_index_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = &options.booster_slot_index {
            result.booster_slot_index = fit_data.validate_booster_slot_index_verbose(&opts.kfs, ctx);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Shared between mod-alike items
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = &options.cpu {
            result.cpu = fit_data.validate_cpu_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.powergrid {
            result.powergrid = fit_data.validate_powergrid_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.ship_limit {
            result.ship_limit = fit_data.validate_ship_limit_verbose(&opts.kfs, ctx, ship);
        }
        if let ValOptionInt::Enabled(opts) = &options.max_group_fitted {
            result.max_group_fitted = fit_data.validate_max_group_fitted_verbose(&opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = &options.max_group_online {
            result.max_group_online = fit_data.validate_max_group_online_verbose(&opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = &options.max_group_active {
            result.max_group_active = fit_data.validate_max_group_active_verbose(&opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = &options.max_type_fitted {
            result.max_type_fitted = fit_data.validate_max_type_fitted_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = &options.item_vs_ship_kind {
            result.item_vs_ship_kind = fit_data.validate_item_vs_ship_kind_verbose(&opts.kfs, ctx, fit);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Modules
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = &options.high_slot_count {
            result.high_slot_count = fit_data.validate_high_slot_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.mid_slot_count {
            result.mid_slot_count = fit_data.validate_mid_slot_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.low_slot_count {
            result.low_slot_count = fit_data.validate_low_slot_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.turret_slot_count {
            result.turret_slot_count = fit_data.validate_turret_slot_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.launcher_slot_count {
            result.launcher_slot_count = fit_data.validate_launcher_slot_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.module_state {
            result.module_state = fit_data.validate_module_state_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = &options.capital_module {
            result.capital_module = fit_data.validate_capital_module_verbose(&opts.kfs, ctx, ship);
        }
        if let ValOptionInt::Enabled(opts) = &options.overload_skill {
            result.overload_skill = fit_data.validate_overload_skill_verbose(&opts.kfs, ctx, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.unusable_cap {
            result.unusable_cap = fit_data.validate_unusable_cap_verbose(&opts.kfs, ctx, calc, fit.ship);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Charges
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = &options.charge_group {
            result.charge_group = fit_data.validate_charge_group_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = &options.charge_parent_group {
            result.charge_parent_group = fit_data.validate_charge_cont_group_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = &options.charge_size {
            result.charge_size = fit_data.validate_charge_size_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = &options.charge_volume {
            result.charge_volume = fit_data.validate_charge_volume_verbose(&opts.kfs, ctx);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Rigs
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = &options.rig_slot_count {
            result.rig_slot_count = fit_data.validate_rig_slot_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.calibration {
            result.calibration = fit_data.validate_calibration_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.rig_size {
            result.rig_size = fit_data.validate_rig_size_verbose(&opts.kfs, ctx, ship);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Services
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = &options.service_slot_count {
            result.service_slot_count = fit_data.validate_service_slot_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // T3 subsystems/stances
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = &options.subsystem_slot_count {
            result.subsystem_slot_count = fit_data.validate_subsystem_slot_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.subsystem_slot_index {
            result.subsystem_slot_index = fit_data.validate_subsystem_slot_index_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = &options.ship_stance {
            result.ship_stance = fit_data.validate_ship_stance_verbose(&opts.kfs, ctx, fit, ship);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Drones
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = &options.drone_bay_volume {
            result.drone_bay_volume = fit_data.validate_drone_bay_volume_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.launched_drone_count {
            result.launched_drone_count = fit_data.validate_launched_drone_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.drone_bandwidth {
            result.drone_bandwidth = fit_data.validate_drone_bandwidth_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_drone_slot {
            result.unlaunchable_drone_slot =
                fit_data.validate_unlaunchable_drone_slot_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_drone_bandwidth {
            result.unlaunchable_drone_bandwidth =
                fit_data.validate_unlaunchable_drone_bandwidth_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.drone_group {
            result.drone_group = fit_data.validate_drone_group_verbose(&opts.kfs, ctx);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Fighters
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = &options.fighter_bay_volume {
            result.fighter_bay_volume = fit_data.validate_fighter_bay_volume_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.launched_fighter_count {
            result.launched_fighter_count = fit_data.validate_launched_fighter_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.launched_light_fighter_count {
            result.launched_light_fighter_count =
                fit_data.validate_launched_light_fighter_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.launched_heavy_fighter_count {
            result.launched_heavy_fighter_count =
                fit_data.validate_launched_heavy_fighter_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.launched_support_fighter_count {
            result.launched_support_fighter_count =
                fit_data.validate_launched_support_fighter_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.launched_st_light_fighter_count {
            result.launched_st_light_fighter_count =
                fit_data.validate_launched_st_light_fighter_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.launched_st_heavy_fighter_count {
            result.launched_st_heavy_fighter_count =
                fit_data.validate_launched_st_heavy_fighter_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.launched_st_support_fighter_count {
            result.launched_st_support_fighter_count =
                fit_data.validate_launched_st_support_fighter_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_fighter {
            result.unlaunchable_fighter = fit_data.validate_unlaunchable_fighter_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_light_fighter {
            result.unlaunchable_light_fighter =
                fit_data.validate_unlaunchable_light_fighter_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_heavy_fighter {
            result.unlaunchable_heavy_fighter =
                fit_data.validate_unlaunchable_heavy_fighter_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_support_fighter {
            result.unlaunchable_support_fighter =
                fit_data.validate_unlaunchable_support_fighter_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_st_light_fighter {
            result.unlaunchable_st_light_fighter =
                fit_data.validate_unlaunchable_st_light_fighter_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_st_heavy_fighter {
            result.unlaunchable_st_heavy_fighter =
                fit_data.validate_unlaunchable_st_heavy_fighter_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.unlaunchable_st_support_fighter {
            result.unlaunchable_st_support_fighter =
                fit_data.validate_unlaunchable_st_support_fighter_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.fighter_squad_size {
            result.fighter_squad_size = fit_data.validate_fighter_squad_size_verbose(&opts.kfs, ctx);
        }
        // Projection, destination side
        if let ValOptionInt::Enabled(opts) = &options.activation_blocked {
            result.activation_blocked = fit_data.validate_activation_blocked_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = &options.effect_stopper {
            result.effect_stopper = fit_data.validate_effect_stopper_verbose(&opts.kfs, ctx, calc);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Projection, source side
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = &options.projectee_filter {
            result.projectee_filter = fit_data.validate_projectee_filter_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = &options.assist_immunity {
            result.assist_immunity = fit_data.validate_assist_immunity_verbose(&opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = &options.offense_immunity {
            result.offense_immunity = fit_data.validate_offense_immunity_verbose(&opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = &options.resist_immunity {
            result.resist_immunity = fit_data.validate_resist_immunity_verbose(&opts.kfs, ctx, calc);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Sec zone
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = &options.sec_zone_fitted {
            result.sec_zone_fitted = fit_data.validate_sec_zone_fitted_verbose(&opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = &options.sec_zone_online {
            result.sec_zone_online = fit_data.validate_sec_zone_online_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = &options.sec_zone_active {
            result.sec_zone_active = fit_data.validate_sec_zone_active_verbose(&opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = &options.sec_zone_unonlineable {
            result.sec_zone_unonlineable = fit_data.validate_sec_zone_unonlineable_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = &options.sec_zone_unactivable {
            result.sec_zone_unactivable = fit_data.validate_sec_zone_unactivable_verbose(&opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = &options.sec_zone_effect {
            result.sec_zone_effect = fit_data.validate_sec_zone_effect_verbose(&opts.kfs, ctx);
        }
        result
    }
}
