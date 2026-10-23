use crate::{
    svc::{
        Calc, SvcCtx, Vast,
        vast::{
            ValOptionsInt, ValOptionsSolInt,
            val::{ValKind, ValOptionInt},
        },
    },
    ud::UFitId,
    val::{ValResultFit, ValResultSol},
};

impl Vast {
    pub(in crate::svc) fn validate_sol_fast(&self, ctx: &SvcCtx, calc: &mut Calc, options: &ValOptionsSolInt) -> bool {
        for &fit_uid in options.fit_uids.iter() {
            if !self.validate_fit_fast(ctx, calc, fit_uid, &options.options) {
                return false;
            }
        }
        if let ValOptionInt::Enabled(opts) = options.options.get(ValKind::NotLoadedItem)
            && !self.validate_not_loaded_item_fast(&opts.kfs)
        {
            return false;
        }
        true
    }
    pub(in crate::svc) fn validate_sol_verbose(
        &mut self,
        ctx: &SvcCtx,
        calc: &mut Calc,
        options: &ValOptionsSolInt,
    ) -> ValResultSol {
        let mut sol_result = ValResultSol::new();
        for &fit_uid in options.fit_uids.iter() {
            let fit_result = self.validate_fit_verbose(ctx, calc, fit_uid, &options.options);
            if !fit_result.all_passed() {
                let fit_id = ctx.u_data.fits.ext_id_by_int_id(fit_uid);
                sol_result.fits.push((fit_id, fit_result));
            }
        }
        if let ValOptionInt::Enabled(opts) = options.options.get(ValKind::NotLoadedItem) {
            sol_result.not_loaded_item = self.validate_not_loaded_item_verbose(&opts.kfs, ctx);
        }
        sol_result
    }
    pub(in crate::svc) fn validate_fit_fast(
        &self,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        options: &ValOptionsInt,
    ) -> bool {
        let fit = ctx.u_data.fits.get(fit_uid);
        let fit_data = self.get_fit_data(fit_uid);
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
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SkillReqs)
            && !fit_data.validate_skill_reqs_fast(&opts.kfs)
        {
            return false;
        }
        // Very cheap check which prevents using big groups of modules/rigs on wrong kind of ship
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ItemVsShipKind)
            && !fit_data.validate_item_vs_ship_kind_fast(&opts.kfs)
        {
            return false;
        }
        // Cheap module validations are close to the top as well. The only expensive operation is
        // grabbing modified slot count from ship.
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::HighSlotCount)
            && !fit_data.validate_high_slot_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::MidSlotCount)
            && !fit_data.validate_mid_slot_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LowSlotCount)
            && !fit_data.validate_low_slot_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::TurretSlotCount)
            && !fit_data.validate_turret_slot_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LauncherSlotCount)
            && !fit_data.validate_launcher_slot_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Cheap checks related to charges; try-fit items functionality attempts to fit those now,
        // and quantity of charges is high, so those validations are close to the top
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ChargeGroup)
            && !fit_data.validate_charge_group_fast(&opts.kfs)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ChargeParentGroup)
            && !fit_data.validate_charge_cont_group_fast(&opts.kfs)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ChargeSize)
            && !fit_data.validate_charge_size_fast(&opts.kfs)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ChargeVolume)
            && !fit_data.validate_charge_volume_fast(&opts.kfs)
        {
            return false;
        }
        // Relatively expensive check, but cost scales with amount of limited items
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ShipLimit)
            && !fit_data.validate_ship_limit_fast(&opts.kfs)
        {
            return false;
        }
        // A group of checks which isn't too cheap to run, but scales with amount of limited items,
        // and there are quite a few items with those limits.
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::MaxGroupFitted)
            && !fit_data.validate_max_group_fitted_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::MaxGroupOnline)
            && !fit_data.validate_max_group_online_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::MaxGroupActive)
            && !fit_data.validate_max_group_active_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        // Cheap module check, but only one module uses it at the moment (rorq's PANIC)
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::MaxTypeFitted)
            && !fit_data.validate_max_type_fitted_fast(&opts.kfs)
        {
            return false;
        }
        // Niche but very cheap. Does not allow to fit cap mods to subcaps, filters out some modules
        // before more expensive PG check.
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::CapitalModule)
            && !fit_data.validate_capital_module_fast(&opts.kfs, ship)
        {
            return false;
        }
        // Cheap, but somewhat useless for "try fit" functionality check, since modules are added in
        // online state.
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ModuleState)
            && !fit_data.validate_module_state_fast(&opts.kfs)
        {
            return false;
        }
        // Rigs - cheap slot validation first, then size which is likely to fail (~3/4th of rigs can
        // not be fit to a ship), then calibration which is expensive and not very likely to fail
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::RigSlotCount)
            && !fit_data.validate_rig_slot_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::RigSize)
            && !fit_data.validate_rig_size_fast(&opts.kfs)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::Calibration)
            && !fit_data.validate_calibration_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Implants - lots of implants, but validation is not likely to fail (need implant slots
        // filled for it to do so), so it's pushed down a bit
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ImplantSlotIndex)
            && !fit_data.validate_implant_slot_index_fast(&opts.kfs)
        {
            return false;
        }
        // Very expensive resource checks related to modules/services. PG over CPU since it is more
        // likely to break validation (modules of bigger sizes usually instantly take more PG than a
        // ship provides)
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::Powergrid)
            && !fit_data.validate_powergrid_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::Cpu)
            && !fit_data.validate_cpu_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Drones
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::DroneBayVolume)
            && !fit_data.validate_drone_bay_volume_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableDroneBandwidth)
            && !fit_data.validate_unlaunchable_drone_bandwidth_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Unlikely to fail, since drones are not added in in-space+ state
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::DroneBandwidth)
            && !fit_data.validate_drone_bandwidth_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Unlikely to fail, since drones are not added in in-space+ state
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LaunchedDroneCount)
            && !fit_data.validate_launched_drone_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Fighters
        // Volume goes first - since it's as cheap as unlaunchable fighter, but can also fail on a
        // carrier fit.
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::FighterBayVolume)
            && !fit_data.validate_fighter_bay_volume_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableFighter)
            && !fit_data.validate_unlaunchable_fighter_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableLightFighter)
            && !fit_data.validate_unlaunchable_light_fighter_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableHeavyFighter)
            && !fit_data.validate_unlaunchable_heavy_fighter_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableSupportFighter)
            && !fit_data.validate_unlaunchable_support_fighter_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableStLightFighter)
            && !fit_data.validate_unlaunchable_st_light_fighter_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableStHeavyFighter)
            && !fit_data.validate_unlaunchable_st_heavy_fighter_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableStSupportFighter)
            && !fit_data.validate_unlaunchable_st_support_fighter_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Launched go after launchable, since they are less likely to fail due to fighter state
        // condition.
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LaunchedFighterCount)
            && !fit_data.validate_launched_fighter_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LaunchedLightFighterCount)
            && !fit_data.validate_launched_light_fighter_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LaunchedHeavyFighterCount)
            && !fit_data.validate_launched_heavy_fighter_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LaunchedSupportFighterCount)
            && !fit_data.validate_launched_support_fighter_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LaunchedStLightFighterCount)
            && !fit_data.validate_launched_st_light_fighter_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LaunchedStHeavyFighterCount)
            && !fit_data.validate_launched_st_heavy_fighter_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LaunchedStSupportFighterCount)
            && !fit_data.validate_launched_st_support_fighter_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Very niche, since fighter count has to be overridden to a value higher than squad
        // supports.
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::FighterSquadSize)
            && !fit_data.validate_fighter_squad_size_fast(&opts.kfs)
        {
            return false;
        }
        // Boosters are below drones and fighters because they are not likely to fail, despite being
        // more numerous item category
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::BoosterSlotIndex)
            && !fit_data.validate_booster_slot_index_fast(&opts.kfs)
        {
            return false;
        }
        // Depends on some incoming projections or system/fit-wide effects, but can fail for some
        // modules in those conditions (e.g. MWD under ESS bubble effect).
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ActivationBlocked)
            && !fit_data.validate_activation_blocked_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        // Subsystems - very few subsystems, unlikely to fail
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SubsystemSlotIndex)
            && !fit_data.validate_subsystem_slot_index_fast(&opts.kfs)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SubsystemSlotCount)
            && !fit_data.validate_subsystem_slot_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Services - very few services, applicable only to citadels, which usually do not have all
        // slots filled anyway
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ServiceSlotCount)
            && !fit_data.validate_service_slot_count_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Moderate cost-wise check, which unlikely to fail, since it works only fit has multiple
        // cloaks, or cloak + items blocking it
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::CloakingBlocked)
            && !fit_data.validate_cloaking_blocked_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // Security zone-specific checks. Usually should pass, since expectation is to have fit in
        // nullsec, which has no sec zone limits, at least for now.
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SecZoneFitted)
            && !fit_data.validate_sec_zone_fitted_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SecZoneOnline)
            && !fit_data.validate_sec_zone_online_fast(&opts.kfs, ctx)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SecZoneActive)
            && !fit_data.validate_sec_zone_active_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SecZoneUnonlineable)
            && !fit_data.validate_sec_zone_unonlineable_fast(&opts.kfs, ctx)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SecZoneUnactivable)
            && !fit_data.validate_sec_zone_unactivable_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SecZoneEffect)
            && !fit_data.validate_sec_zone_effect_fast(&opts.kfs, ctx)
        {
            return false;
        }
        // Incoming projection - effect stopper shouldn't fail for tried items, since there are no
        // indirect ways to stop item effects for now.
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::EffectStopper)
            && !fit_data.validate_effect_stopper_fast(&opts.kfs, ctx, calc)
        {
            return false;
        }
        // Outgoing projections - useless for try-fit functionality, since tried items do not get
        // outgoing projections added.
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ProjecteeFilter)
            && !fit_data.validate_projectee_filter_fast(&opts.kfs, ctx)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::AssistImmunity)
            && !self.validate_assist_immunity_fast(fit_data, &opts.kfs, ctx, calc)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::OffenseImmunity)
            && !self.validate_offense_immunity_fast(fit_data, &opts.kfs, ctx, calc)
        {
            return false;
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ResistImmunity)
            && !self.validate_resist_immunity_fast(fit_data, &opts.kfs, ctx, calc)
        {
            return false;
        }
        // Misc checks - rarely used, or unlikely to fail
        // Majority of fits are supposed to have thermodynamics 1 trained, and not every fit has
        // overloaded modules.
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::OverloadSkill)
            && !fit_data.validate_overload_skill_fast(&opts.kfs, fit)
        {
            return false;
        }
        // T3D-specific check which should pass if nothing goes wrong on the app side
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ShipStance)
            && !fit_data.validate_ship_stance_fast(&opts.kfs, fit, ship)
        {
            return false;
        }
        // Happens only at drone skill 0, which is not something likely to see
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableDroneSlot)
            && !fit_data.validate_unlaunchable_drone_slot_fast(&opts.kfs, ctx, calc, fit)
        {
            return false;
        }
        // In regular conditions, items kinds are supposed to match expected ones
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ItemKind)
            && !fit_data.validate_item_kind_fast(&opts.kfs)
        {
            return false;
        }
        // In regular conditions, items are supposed to be loaded
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::NotLoadedItem)
            && !fit_data.validate_not_loaded_item_fast(&opts.kfs)
        {
            return false;
        }
        // Expensive check which rarely fails
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnusableCap)
            && !fit_data.validate_unusable_cap_fast(&opts.kfs, ctx, calc, fit.ship)
        {
            return false;
        }
        // No known items use it, only fighter drones used to have it
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::DroneGroup)
            && !fit_data.validate_drone_group_fast(&opts.kfs)
        {
            return false;
        }
        true
    }
    pub(in crate::svc) fn validate_fit_verbose(
        &self,
        ctx: &SvcCtx,
        calc: &mut Calc,
        fit_uid: UFitId,
        options: &ValOptionsInt,
    ) -> ValResultFit {
        let fit = ctx.u_data.fits.get(fit_uid);
        let fit_data = self.get_fit_data(fit_uid);
        let ship = fit.ship.map(|v| ctx.u_data.items.get(v).dc_ship().unwrap());
        let mut result = ValResultFit { .. };
        let ValResultFit {
            not_loaded_item,
            item_kind,
            skill_reqs,
            implant_slot_index,
            booster_slot_index,
            cpu,
            powergrid,
            ship_limit,
            max_group_fitted,
            max_group_online,
            max_group_active,
            max_type_fitted,
            item_vs_ship_kind,
            high_slot_count,
            mid_slot_count,
            low_slot_count,
            turret_slot_count,
            launcher_slot_count,
            module_state,
            capital_module,
            overload_skill,
            unusable_cap,
            charge_group,
            charge_parent_group,
            charge_size,
            charge_volume,
            rig_slot_count,
            calibration,
            rig_size,
            service_slot_count,
            subsystem_slot_count,
            subsystem_slot_index,
            ship_stance,
            drone_bay_volume,
            launched_drone_count,
            drone_bandwidth,
            unlaunchable_drone_slot,
            unlaunchable_drone_bandwidth,
            drone_group,
            fighter_bay_volume,
            launched_fighter_count,
            launched_light_fighter_count,
            launched_heavy_fighter_count,
            launched_support_fighter_count,
            launched_st_light_fighter_count,
            launched_st_heavy_fighter_count,
            launched_st_support_fighter_count,
            unlaunchable_fighter,
            unlaunchable_light_fighter,
            unlaunchable_heavy_fighter,
            unlaunchable_support_fighter,
            unlaunchable_st_light_fighter,
            unlaunchable_st_heavy_fighter,
            unlaunchable_st_support_fighter,
            fighter_squad_size,
            activation_blocked,
            effect_stopper,
            cloaking_blocked,
            projectee_filter,
            assist_immunity,
            offense_immunity,
            resist_immunity,
            sec_zone_fitted,
            sec_zone_online,
            sec_zone_active,
            sec_zone_unonlineable,
            sec_zone_unactivable,
            sec_zone_effect,
        } = &mut result;
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Generic
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::NotLoadedItem) {
            *not_loaded_item = fit_data.validate_not_loaded_item_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ItemKind) {
            *item_kind = fit_data.validate_item_kind_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SkillReqs) {
            *skill_reqs = fit_data.validate_skill_reqs_verbose(&opts.kfs, ctx);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Implants/boosters
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ImplantSlotIndex) {
            *implant_slot_index = fit_data.validate_implant_slot_index_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::BoosterSlotIndex) {
            *booster_slot_index = fit_data.validate_booster_slot_index_verbose(&opts.kfs, ctx);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Shared between mod-alike items
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::Cpu) {
            *cpu = fit_data.validate_cpu_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::Powergrid) {
            *powergrid = fit_data.validate_powergrid_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ShipLimit) {
            *ship_limit = fit_data.validate_ship_limit_verbose(&opts.kfs, ctx, ship);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::MaxGroupFitted) {
            *max_group_fitted = fit_data.validate_max_group_fitted_verbose(&opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::MaxGroupOnline) {
            *max_group_online = fit_data.validate_max_group_online_verbose(&opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::MaxGroupActive) {
            *max_group_active = fit_data.validate_max_group_active_verbose(&opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::MaxTypeFitted) {
            *max_type_fitted = fit_data.validate_max_type_fitted_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ItemVsShipKind) {
            *item_vs_ship_kind = fit_data.validate_item_vs_ship_kind_verbose(&opts.kfs, ctx, fit);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Modules
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::HighSlotCount) {
            *high_slot_count = fit_data.validate_high_slot_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::MidSlotCount) {
            *mid_slot_count = fit_data.validate_mid_slot_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LowSlotCount) {
            *low_slot_count = fit_data.validate_low_slot_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::TurretSlotCount) {
            *turret_slot_count = fit_data.validate_turret_slot_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LauncherSlotCount) {
            *launcher_slot_count = fit_data.validate_launcher_slot_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ModuleState) {
            *module_state = fit_data.validate_module_state_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::CapitalModule) {
            *capital_module = fit_data.validate_capital_module_verbose(&opts.kfs, ctx, ship);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::OverloadSkill) {
            *overload_skill = fit_data.validate_overload_skill_verbose(&opts.kfs, ctx, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnusableCap) {
            *unusable_cap = fit_data.validate_unusable_cap_verbose(&opts.kfs, ctx, calc, fit.ship);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Charges
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ChargeGroup) {
            *charge_group = fit_data.validate_charge_group_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ChargeParentGroup) {
            *charge_parent_group = fit_data.validate_charge_cont_group_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ChargeSize) {
            *charge_size = fit_data.validate_charge_size_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ChargeVolume) {
            *charge_volume = fit_data.validate_charge_volume_verbose(&opts.kfs, ctx);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Rigs
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::RigSlotCount) {
            *rig_slot_count = fit_data.validate_rig_slot_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::Calibration) {
            *calibration = fit_data.validate_calibration_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::RigSize) {
            *rig_size = fit_data.validate_rig_size_verbose(&opts.kfs, ctx, ship);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Services
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ServiceSlotCount) {
            *service_slot_count = fit_data.validate_service_slot_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // T3 subsystems/stances
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SubsystemSlotCount) {
            *subsystem_slot_count = fit_data.validate_subsystem_slot_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SubsystemSlotIndex) {
            *subsystem_slot_index = fit_data.validate_subsystem_slot_index_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ShipStance) {
            *ship_stance = fit_data.validate_ship_stance_verbose(&opts.kfs, ctx, fit, ship);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Drones
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::DroneBayVolume) {
            *drone_bay_volume = fit_data.validate_drone_bay_volume_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LaunchedDroneCount) {
            *launched_drone_count = fit_data.validate_launched_drone_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::DroneBandwidth) {
            *drone_bandwidth = fit_data.validate_drone_bandwidth_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableDroneSlot) {
            *unlaunchable_drone_slot = fit_data.validate_unlaunchable_drone_slot_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableDroneBandwidth) {
            *unlaunchable_drone_bandwidth =
                fit_data.validate_unlaunchable_drone_bandwidth_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::DroneGroup) {
            *drone_group = fit_data.validate_drone_group_verbose(&opts.kfs, ctx);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Fighters
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::FighterBayVolume) {
            *fighter_bay_volume = fit_data.validate_fighter_bay_volume_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LaunchedFighterCount) {
            *launched_fighter_count = fit_data.validate_launched_fighter_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LaunchedLightFighterCount) {
            *launched_light_fighter_count =
                fit_data.validate_launched_light_fighter_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LaunchedHeavyFighterCount) {
            *launched_heavy_fighter_count =
                fit_data.validate_launched_heavy_fighter_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LaunchedSupportFighterCount) {
            *launched_support_fighter_count =
                fit_data.validate_launched_support_fighter_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LaunchedStLightFighterCount) {
            *launched_st_light_fighter_count =
                fit_data.validate_launched_st_light_fighter_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LaunchedStHeavyFighterCount) {
            *launched_st_heavy_fighter_count =
                fit_data.validate_launched_st_heavy_fighter_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::LaunchedStSupportFighterCount) {
            *launched_st_support_fighter_count =
                fit_data.validate_launched_st_support_fighter_count_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableFighter) {
            *unlaunchable_fighter = fit_data.validate_unlaunchable_fighter_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableLightFighter) {
            *unlaunchable_light_fighter =
                fit_data.validate_unlaunchable_light_fighter_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableHeavyFighter) {
            *unlaunchable_heavy_fighter =
                fit_data.validate_unlaunchable_heavy_fighter_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableSupportFighter) {
            *unlaunchable_support_fighter =
                fit_data.validate_unlaunchable_support_fighter_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableStLightFighter) {
            *unlaunchable_st_light_fighter =
                fit_data.validate_unlaunchable_st_light_fighter_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableStHeavyFighter) {
            *unlaunchable_st_heavy_fighter =
                fit_data.validate_unlaunchable_st_heavy_fighter_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::UnlaunchableStSupportFighter) {
            *unlaunchable_st_support_fighter =
                fit_data.validate_unlaunchable_st_support_fighter_verbose(&opts.kfs, ctx, calc, fit);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::FighterSquadSize) {
            *fighter_squad_size = fit_data.validate_fighter_squad_size_verbose(&opts.kfs, ctx);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Projection, destination side
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ActivationBlocked) {
            *activation_blocked = fit_data.validate_activation_blocked_verbose(&opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::EffectStopper) {
            *effect_stopper = fit_data.validate_effect_stopper_verbose(&opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::CloakingBlocked) {
            *cloaking_blocked = fit_data.validate_cloaking_blocked_verbose(&opts.kfs, ctx, calc, fit);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Projection, source side
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ProjecteeFilter) {
            *projectee_filter = fit_data.validate_projectee_filter_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::AssistImmunity) {
            *assist_immunity = self.validate_assist_immunity_verbose(fit_data, &opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::OffenseImmunity) {
            *offense_immunity = self.validate_offense_immunity_verbose(fit_data, &opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::ResistImmunity) {
            *resist_immunity = self.validate_resist_immunity_verbose(fit_data, &opts.kfs, ctx, calc);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // Sec zone
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SecZoneFitted) {
            *sec_zone_fitted = fit_data.validate_sec_zone_fitted_verbose(&opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SecZoneOnline) {
            *sec_zone_online = fit_data.validate_sec_zone_online_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SecZoneActive) {
            *sec_zone_active = fit_data.validate_sec_zone_active_verbose(&opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SecZoneUnonlineable) {
            *sec_zone_unonlineable = fit_data.validate_sec_zone_unonlineable_verbose(&opts.kfs, ctx);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SecZoneUnactivable) {
            *sec_zone_unactivable = fit_data.validate_sec_zone_unactivable_verbose(&opts.kfs, ctx, calc);
        }
        if let ValOptionInt::Enabled(opts) = options.get(ValKind::SecZoneEffect) {
            *sec_zone_effect = fit_data.validate_sec_zone_effect_verbose(&opts.kfs, ctx);
        }
        result
    }
}
