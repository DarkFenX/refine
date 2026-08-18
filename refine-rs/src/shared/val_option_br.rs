use crate::{
    CmdResps, DefOptionExt, ItemId, ItemIdBr,
    err::BrResolveError,
    val::{ValEnabled, ValOptions},
};

pub(crate) fn val_options_br_resolve(
    options: ValOptions<ItemIdBr>,
    cmd_resps: &CmdResps,
) -> Result<ValOptions<ItemId>, BrResolveError> {
    Ok(ValOptions {
        default: options.default,
        not_loaded_item: conv_option(options.not_loaded_item, cmd_resps)?,
        item_kind: conv_option(options.item_kind, cmd_resps)?,
        skill_reqs: conv_option(options.skill_reqs, cmd_resps)?,
        implant_slot_index: conv_option(options.implant_slot_index, cmd_resps)?,
        booster_slot_index: conv_option(options.booster_slot_index, cmd_resps)?,
        cpu: conv_option(options.cpu, cmd_resps)?,
        powergrid: conv_option(options.powergrid, cmd_resps)?,
        ship_limit: conv_option(options.ship_limit, cmd_resps)?,
        max_group_fitted: conv_option(options.max_group_fitted, cmd_resps)?,
        max_group_online: conv_option(options.max_group_online, cmd_resps)?,
        max_group_active: conv_option(options.max_group_active, cmd_resps)?,
        max_type_fitted: conv_option(options.max_type_fitted, cmd_resps)?,
        item_vs_ship_kind: conv_option(options.item_vs_ship_kind, cmd_resps)?,
        high_slot_count: conv_option(options.high_slot_count, cmd_resps)?,
        mid_slot_count: conv_option(options.mid_slot_count, cmd_resps)?,
        low_slot_count: conv_option(options.low_slot_count, cmd_resps)?,
        turret_slot_count: conv_option(options.turret_slot_count, cmd_resps)?,
        launcher_slot_count: conv_option(options.launcher_slot_count, cmd_resps)?,
        module_state: conv_option(options.module_state, cmd_resps)?,
        capital_module: conv_option(options.capital_module, cmd_resps)?,
        overload_skill: conv_option(options.overload_skill, cmd_resps)?,
        unusable_cap: conv_option(options.unusable_cap, cmd_resps)?,
        charge_group: conv_option(options.charge_group, cmd_resps)?,
        charge_parent_group: conv_option(options.charge_parent_group, cmd_resps)?,
        charge_size: conv_option(options.charge_size, cmd_resps)?,
        charge_volume: conv_option(options.charge_volume, cmd_resps)?,
        rig_slot_count: conv_option(options.rig_slot_count, cmd_resps)?,
        calibration: conv_option(options.calibration, cmd_resps)?,
        rig_size: conv_option(options.rig_size, cmd_resps)?,
        service_slot_count: conv_option(options.service_slot_count, cmd_resps)?,
        subsystem_slot_count: conv_option(options.subsystem_slot_count, cmd_resps)?,
        subsystem_slot_index: conv_option(options.subsystem_slot_index, cmd_resps)?,
        ship_stance: conv_option(options.ship_stance, cmd_resps)?,
        drone_bay_volume: conv_option(options.drone_bay_volume, cmd_resps)?,
        launched_drone_count: conv_option(options.launched_drone_count, cmd_resps)?,
        drone_bandwidth: conv_option(options.drone_bandwidth, cmd_resps)?,
        unlaunchable_drone_slot: conv_option(options.unlaunchable_drone_slot, cmd_resps)?,
        unlaunchable_drone_bandwidth: conv_option(options.unlaunchable_drone_bandwidth, cmd_resps)?,
        drone_group: conv_option(options.drone_group, cmd_resps)?,
        fighter_bay_volume: conv_option(options.fighter_bay_volume, cmd_resps)?,
        launched_fighter_count: conv_option(options.launched_fighter_count, cmd_resps)?,
        launched_light_fighter_count: conv_option(options.launched_light_fighter_count, cmd_resps)?,
        launched_heavy_fighter_count: conv_option(options.launched_heavy_fighter_count, cmd_resps)?,
        launched_support_fighter_count: conv_option(options.launched_support_fighter_count, cmd_resps)?,
        launched_st_light_fighter_count: conv_option(options.launched_st_light_fighter_count, cmd_resps)?,
        launched_st_heavy_fighter_count: conv_option(options.launched_st_heavy_fighter_count, cmd_resps)?,
        launched_st_support_fighter_count: conv_option(options.launched_st_support_fighter_count, cmd_resps)?,
        unlaunchable_fighter: conv_option(options.unlaunchable_fighter, cmd_resps)?,
        unlaunchable_light_fighter: conv_option(options.unlaunchable_light_fighter, cmd_resps)?,
        unlaunchable_heavy_fighter: conv_option(options.unlaunchable_heavy_fighter, cmd_resps)?,
        unlaunchable_support_fighter: conv_option(options.unlaunchable_support_fighter, cmd_resps)?,
        unlaunchable_st_light_fighter: conv_option(options.unlaunchable_st_light_fighter, cmd_resps)?,
        unlaunchable_st_heavy_fighter: conv_option(options.unlaunchable_st_heavy_fighter, cmd_resps)?,
        unlaunchable_st_support_fighter: conv_option(options.unlaunchable_st_support_fighter, cmd_resps)?,
        fighter_squad_size: conv_option(options.fighter_squad_size, cmd_resps)?,
        activation_blocked: conv_option(options.activation_blocked, cmd_resps)?,
        effect_stopper: conv_option(options.effect_stopper, cmd_resps)?,
        cloaking_blocked: conv_option(options.cloaking_blocked, cmd_resps)?,
        projectee_filter: conv_option(options.projectee_filter, cmd_resps)?,
        assist_immunity: conv_option(options.assist_immunity, cmd_resps)?,
        offense_immunity: conv_option(options.offense_immunity, cmd_resps)?,
        resist_immunity: conv_option(options.resist_immunity, cmd_resps)?,
        sec_zone_fitted: conv_option(options.sec_zone_fitted, cmd_resps)?,
        sec_zone_online: conv_option(options.sec_zone_online, cmd_resps)?,
        sec_zone_active: conv_option(options.sec_zone_active, cmd_resps)?,
        sec_zone_unonlineable: conv_option(options.sec_zone_unonlineable, cmd_resps)?,
        sec_zone_unactivable: conv_option(options.sec_zone_unactivable, cmd_resps)?,
        sec_zone_effect: conv_option(options.sec_zone_effect, cmd_resps)?,
    })
}

fn conv_option(
    option: DefOptionExt<ValEnabled<ItemIdBr>>,
    cmd_resps: &CmdResps,
) -> Result<DefOptionExt<ValEnabled<ItemId>>, BrResolveError> {
    Ok(match option {
        DefOptionExt::Default => DefOptionExt::Default,
        DefOptionExt::Disabled => DefOptionExt::Disabled,
        DefOptionExt::Enabled => DefOptionExt::Enabled,
        DefOptionExt::EnabledExtended(enabled_opts) => {
            DefOptionExt::EnabledExtended(conv_enabled(enabled_opts, cmd_resps)?)
        }
    })
}

fn conv_enabled(
    enabled_opts: ValEnabled<ItemIdBr>,
    cmd_resps: &CmdResps,
) -> Result<ValEnabled<ItemId>, BrResolveError> {
    Ok(ValEnabled {
        kfs: cmd_resps.resolve_item_ids(enabled_opts.kfs)?,
    })
}
