from fw import check_no_field
from fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionJump


def test_ranges(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_amount)
    eve_portal_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.enable_open_jump_portal)
    eve_portal_fuel_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_consumption_mass_factor)
    eve_portal_fuel_add_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_additional_consumption)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_fuel_id = client.mk_eve_item()
    eve_main_ship1_id = client.mk_eve_ship(attrs={
        eve_range_attr_id: 5,
        eve_fuel_type_attr_id: eve_fuel_id,
        eve_fuel_use_attr_id: 1500,
        eve_portal_fuel_mult_attr_id: 0.000000001})
    eve_main_ship2_id = client.mk_eve_ship(attrs={
        eve_range_attr_id: 5,
        eve_fuel_type_attr_id: eve_fuel_id,
        eve_fuel_use_attr_id: 1500,
        eve_portal_fuel_mult_attr_id: 0.000000001,
        eve_portal_fuel_add_attr_id: 50})
    eve_portal_id = client.mk_eve_item(attrs={eve_portal_flag_attr_id: 1})
    eve_psg_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit_main = api_sol.create_fit()
    api_main_ship = api_fit_main.set_ship(type_id=eve_main_ship1_id)
    api_main_portal = api_fit_main.add_module(type_id=eve_portal_id, state=consts.ApiModuleState.active)
    api_psg_fit = api_sol.create_fit()
    api_psg_fit.set_ship(type_id=eve_psg_ship_id)
    # Verification
    api_jump_options = [
        StatsOptionJump(passenger_fit_ids=[api_psg_fit.id]),
        StatsOptionJump(passenger_fit_ids=[api_psg_fit.id], range=5.1),
        StatsOptionJump(passenger_fit_ids=[api_psg_fit.id], range=5),
        StatsOptionJump(passenger_fit_ids=[api_psg_fit.id], range='max'),
        StatsOptionJump(passenger_fit_ids=[api_psg_fit.id], range=2),
        StatsOptionJump(passenger_fit_ids=[api_psg_fit.id], range=0.1),
        StatsOptionJump(passenger_fit_ids=[api_psg_fit.id], range=0),
        StatsOptionJump(passenger_fit_ids=[api_psg_fit.id], range=1.1)]
    (api_fit_jump_default,
     api_fit_jump_excessive,
     api_fit_jump_max_num,
     api_fit_jump_max_spec,
     api_fit_jump_med,
     api_fit_jump_low,
     api_fit_jump_zero,
     api_fit_jump_rounding) = api_fit_main.get_stats(options=FitStatsOptions(jump=(True, api_jump_options))).jump
    assert api_fit_jump_default.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 147
    with check_no_field():
        api_fit_jump_excessive.portals  # noqa: B018
    assert api_fit_jump_max_num.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 147
    assert api_fit_jump_max_spec.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 147
    assert api_fit_jump_med.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 59
    assert api_fit_jump_low.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 3
    assert api_fit_jump_zero.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 0
    assert api_fit_jump_rounding.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 33
    (api_ship_jump_default,
     api_ship_jump_excessive,
     api_ship_jump_max_num,
     api_ship_jump_max_spec,
     api_ship_jump_med,
     api_ship_jump_low,
     api_ship_jump_zero,
     api_ship_jump_rounding) = api_main_ship.get_stats(options=ItemStatsOptions(jump=(True, api_jump_options))).jump
    assert api_ship_jump_default.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 147
    with check_no_field():
        api_ship_jump_excessive.portals  # noqa: B018
    assert api_ship_jump_max_num.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 147
    assert api_ship_jump_max_spec.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 147
    assert api_ship_jump_med.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 59
    assert api_ship_jump_low.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 3
    assert api_ship_jump_zero.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 0
    assert api_ship_jump_rounding.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 33
    # Action
    api_main_ship.change_ship(type_id=eve_main_ship2_id)
    # Verification - check that extra fuel is added (used in ansi portal)
    api_jump_options = [
        StatsOptionJump(passenger_fit_ids=[api_psg_fit.id]),
        StatsOptionJump(passenger_fit_ids=[api_psg_fit.id], range=5.1),
        StatsOptionJump(passenger_fit_ids=[api_psg_fit.id], range=5),
        StatsOptionJump(passenger_fit_ids=[api_psg_fit.id], range='max'),
        StatsOptionJump(passenger_fit_ids=[api_psg_fit.id], range=2),
        StatsOptionJump(passenger_fit_ids=[api_psg_fit.id], range=0.1),
        StatsOptionJump(passenger_fit_ids=[api_psg_fit.id], range=0),
        StatsOptionJump(passenger_fit_ids=[api_psg_fit.id], range=1.1)]
    (api_fit_jump_default,
     api_fit_jump_excessive,
     api_fit_jump_max_num,
     api_fit_jump_max_spec,
     api_fit_jump_med,
     api_fit_jump_low,
     api_fit_jump_zero,
     api_fit_jump_rounding) = api_fit_main.get_stats(options=FitStatsOptions(jump=(True, api_jump_options))).jump
    assert api_fit_jump_default.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 197
    with check_no_field():
        api_fit_jump_excessive.portals  # noqa: B018
    assert api_fit_jump_max_num.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 197
    assert api_fit_jump_max_spec.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 197
    assert api_fit_jump_med.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 109
    assert api_fit_jump_low.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 53
    assert api_fit_jump_zero.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 50
    assert api_fit_jump_rounding.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 83
    (api_ship_jump_default,
     api_ship_jump_excessive,
     api_ship_jump_max_num,
     api_ship_jump_max_spec,
     api_ship_jump_med,
     api_ship_jump_low,
     api_ship_jump_zero,
     api_ship_jump_rounding) = api_main_ship.get_stats(options=ItemStatsOptions(jump=(True, api_jump_options))).jump
    assert api_ship_jump_default.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 197
    with check_no_field():
        api_ship_jump_excessive.portals  # noqa: B018
    assert api_ship_jump_max_num.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 197
    assert api_ship_jump_max_spec.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 197
    assert api_ship_jump_med.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 109
    assert api_ship_jump_low.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 53
    assert api_ship_jump_zero.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 50
    assert api_ship_jump_rounding.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == 83


def test_passenger_status(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_amount)
    eve_portal_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.enable_open_jump_portal)
    eve_portal_psg_attr_id = client.mk_eve_attr()
    eve_portal_psg_ref_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_passenger_required_attr_id)
    eve_portal_fuel_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_consumption_mass_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_psg_mod_attr_id = client.mk_eve_attr()
    eve_psg_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_psg_mod_attr_id,
        affectee_attr_id=eve_portal_psg_attr_id)
    eve_psg_effect_id = client.mk_eve_effect(mod_info=[eve_psg_mod])
    eve_subsystem_id = client.mk_eve_item(attrs={eve_psg_mod_attr_id: 1}, eff_ids=[eve_psg_effect_id])
    eve_fuel_id = client.mk_eve_item()
    eve_portal_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr_id})
    eve_main_ship_id = client.mk_eve_ship(attrs={
        eve_range_attr_id: 5,
        eve_fuel_type_attr_id: eve_fuel_id,
        eve_fuel_use_attr_id: 1500,
        eve_portal_fuel_mult_attr_id: 0.000000001})
    eve_psg_enabled_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000, eve_portal_psg_attr_id: 1})
    eve_psg_disabled_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000, eve_portal_psg_attr_id: 0})
    eve_psg_not_set_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000})
    eve_psg_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit_main = api_sol.create_fit()
    api_ship = api_fit_main.set_ship(type_id=eve_main_ship_id)
    api_portal = api_fit_main.add_module(type_id=eve_portal_id, state=consts.ApiModuleState.active)
    api_fit_psg_enabled = api_sol.create_fit()
    api_fit_psg_enabled.set_ship(type_id=eve_psg_enabled_id)
    api_fit_psg_disabled = api_sol.create_fit()
    api_fit_psg_disabled.set_ship(type_id=eve_psg_disabled_id)
    api_fit_psg_not_set = api_sol.create_fit()
    api_fit_psg_not_set.set_ship(type_id=eve_psg_not_set_id)
    api_fit_psg_not_loaded = api_sol.create_fit()
    api_fit_psg_not_loaded.set_ship(type_id=eve_psg_not_loaded_id)
    # Verification
    api_fit_main_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[
            api_fit_psg_enabled.id,
            api_fit_psg_disabled.id,
            api_fit_psg_not_set.id,
            api_fit_psg_not_loaded.id])])))
    api_fit_main_psgs = api_fit_main_stats.jump.one().portals[api_portal.id].fuel_use_passengers
    assert api_fit_main_psgs[api_fit_psg_enabled.id] == 147
    assert api_fit_main_psgs[api_fit_psg_disabled.id] is None
    assert api_fit_main_psgs[api_fit_psg_not_set.id] is None
    assert api_fit_main_psgs[api_fit_psg_not_loaded.id] is None
    api_ship_main_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[
            api_fit_psg_enabled.id,
            api_fit_psg_disabled.id,
            api_fit_psg_not_set.id,
            api_fit_psg_not_loaded.id])])))
    api_ship_main_psgs = api_ship_main_stats.jump.one().portals[api_portal.id].fuel_use_passengers
    assert api_ship_main_psgs[api_fit_psg_enabled.id] == 147
    assert api_ship_main_psgs[api_fit_psg_disabled.id] is None
    assert api_ship_main_psgs[api_fit_psg_not_set.id] is None
    assert api_ship_main_psgs[api_fit_psg_not_loaded.id] is None
    # Action
    api_fit_psg_disabled.add_subsystem(type_id=eve_subsystem_id)
    # Verification - when passenger flag is modified to value which enables it, fit is allowed to be
    # a passenger
    api_fit_main_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg_disabled.id])])))
    api_fit_main_psgs = api_fit_main_stats.jump.one().portals[api_portal.id].fuel_use_passengers
    assert api_fit_main_psgs[api_fit_psg_disabled.id] == 147
    api_ship_main_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg_disabled.id])])))
    api_ship_main_psgs = api_ship_main_stats.jump.one().portals[api_portal.id].fuel_use_passengers
    assert api_ship_main_psgs[api_fit_psg_disabled.id] == 147


def test_passenger_fuel_affectors(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_amount)
    eve_portal_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.enable_open_jump_portal)
    eve_portal_psg_attr_id = client.mk_eve_attr()
    eve_portal_psg_ref_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_passenger_required_attr_id)
    eve_portal_fuel_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_consumption_mass_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_mass_attr_id)
    eve_online_effect_id = client.mk_eve_online_effect()
    eve_plate_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod])
    eve_prop_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod])
    eve_plate_id = client.mk_eve_item(
        attrs={eve_mod_attr_id: 1450000},
        eff_ids=[eve_plate_effect_id, eve_online_effect_id])
    eve_prop_id = client.mk_eve_item(
        attrs={eve_mod_attr_id: 5000000},
        eff_ids=[eve_prop_effect_id],
        defeff_id=eve_prop_effect_id)
    eve_fuel_id = client.mk_eve_item()
    eve_portal_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr_id})
    eve_main_ship_id = client.mk_eve_ship(attrs={
        eve_range_attr_id: 5,
        eve_fuel_type_attr_id: eve_fuel_id,
        eve_fuel_use_attr_id: 1500,
        eve_portal_fuel_mult_attr_id: 0.000000001})
    eve_psg_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000, eve_portal_psg_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit_main = api_sol.create_fit()
    api_ship = api_fit_main.set_ship(type_id=eve_main_ship_id)
    api_portal = api_fit_main.add_module(type_id=eve_portal_id, state=consts.ApiModuleState.active)
    api_fit_psg = api_sol.create_fit()
    api_fit_psg.set_ship(type_id=eve_psg_ship_id)
    api_plate = api_fit_psg.add_module(type_id=eve_plate_id, state=consts.ApiModuleState.online)
    api_prop = api_fit_psg.add_module(type_id=eve_prop_id, state=consts.ApiModuleState.overload)
    # Verification - more fuel with active prop / online plate, less without them
    api_options = [
        StatsOptionJump(passenger_fit_ids=[api_fit_psg.id]),
        StatsOptionJump(passenger_fit_ids=[api_fit_psg.id], passenger_fuel_affectors=consts.ApiCtlAffector.offline),
        StatsOptionJump(passenger_fit_ids=[api_fit_psg.id], passenger_fuel_affectors=consts.ApiCtlAffector.deactivate),
        StatsOptionJump(passenger_fit_ids=[api_fit_psg.id], passenger_fuel_affectors=consts.ApiCtlAffector.unmodified)]
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(jump=(True, api_options)))
    assert api_fit_stats.jump.map(
        lambda i: i.portals[api_portal.id].fuel_use_passengers[api_fit_psg.id],
    ) == [195, 147, 158, 195]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=(True, api_options)))
    assert api_ship_stats.jump.map(
        lambda i: i.portals[api_portal.id].fuel_use_passengers[api_fit_psg.id],
    ) == [195, 147, 158, 195]
    # Action
    api_prop.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_options = [
        StatsOptionJump(passenger_fit_ids=[api_fit_psg.id]),
        StatsOptionJump(passenger_fit_ids=[api_fit_psg.id], passenger_fuel_affectors=consts.ApiCtlAffector.offline),
        StatsOptionJump(passenger_fit_ids=[api_fit_psg.id], passenger_fuel_affectors=consts.ApiCtlAffector.deactivate),
        StatsOptionJump(passenger_fit_ids=[api_fit_psg.id], passenger_fuel_affectors=consts.ApiCtlAffector.unmodified)]
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(jump=(True, api_options)))
    assert api_fit_stats.jump.map(
        lambda i: i.portals[api_portal.id].fuel_use_passengers[api_fit_psg.id],
    ) == [195, 147, 158, 195]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=(True, api_options)))
    assert api_ship_stats.jump.map(
        lambda i: i.portals[api_portal.id].fuel_use_passengers[api_fit_psg.id],
    ) == [195, 147, 158, 195]
    # Action
    api_plate.change_module(state=consts.ApiModuleState.offline)
    # Verification
    api_options = [
        StatsOptionJump(passenger_fit_ids=[api_fit_psg.id]),
        StatsOptionJump(passenger_fit_ids=[api_fit_psg.id], passenger_fuel_affectors=consts.ApiCtlAffector.offline),
        StatsOptionJump(passenger_fit_ids=[api_fit_psg.id], passenger_fuel_affectors=consts.ApiCtlAffector.deactivate),
        StatsOptionJump(passenger_fit_ids=[api_fit_psg.id], passenger_fuel_affectors=consts.ApiCtlAffector.unmodified)]
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(jump=(True, api_options)))
    assert api_fit_stats.jump.map(
        lambda i: i.portals[api_portal.id].fuel_use_passengers[api_fit_psg.id],
    ) == [184, 147, 147, 184]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=(True, api_options)))
    assert api_ship_stats.jump.map(
        lambda i: i.portals[api_portal.id].fuel_use_passengers[api_fit_psg.id],
    ) == [184, 147, 147, 184]
    # Action
    api_prop.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_options = [
        StatsOptionJump(passenger_fit_ids=[api_fit_psg.id]),
        StatsOptionJump(passenger_fit_ids=[api_fit_psg.id], passenger_fuel_affectors=consts.ApiCtlAffector.offline),
        StatsOptionJump(passenger_fit_ids=[api_fit_psg.id], passenger_fuel_affectors=consts.ApiCtlAffector.deactivate),
        StatsOptionJump(passenger_fit_ids=[api_fit_psg.id], passenger_fuel_affectors=consts.ApiCtlAffector.unmodified)]
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(jump=(True, api_options)))
    assert api_fit_stats.jump.map(
        lambda i: i.portals[api_portal.id].fuel_use_passengers[api_fit_psg.id],
    ) == [147, 147, 147, 147]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=(True, api_options)))
    assert api_ship_stats.jump.map(
        lambda i: i.portals[api_portal.id].fuel_use_passengers[api_fit_psg.id],
    ) == [147, 147, 147, 147]


def test_multiple_portals(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_amount)
    eve_portal_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.enable_open_jump_portal)
    eve_portal_psg_attr1_id = client.mk_eve_attr()
    eve_portal_psg_attr2_id = client.mk_eve_attr()
    eve_portal_psg_ref_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_passenger_required_attr_id)
    eve_portal_fuel_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_consumption_mass_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_fuel_id = client.mk_eve_item()
    eve_portal1_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr1_id})
    eve_portal2_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr2_id})
    eve_main_ship_id = client.mk_eve_ship(attrs={
        eve_range_attr_id: 5,
        eve_fuel_type_attr_id: eve_fuel_id,
        eve_fuel_use_attr_id: 1500,
        eve_portal_fuel_mult_attr_id: 0.000000001})
    eve_psg_ship1_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000, eve_portal_psg_attr1_id: 1})
    eve_psg_ship2_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000, eve_portal_psg_attr2_id: 1})
    eve_psg_ship3_id = client.mk_eve_ship(
        attrs={eve_mass_attr_id: 19500000, eve_portal_psg_attr1_id: 1, eve_portal_psg_attr2_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit_main = api_sol.create_fit()
    api_ship_main = api_fit_main.set_ship(type_id=eve_main_ship_id)
    api_portal1 = api_fit_main.add_module(type_id=eve_portal1_id, state=consts.ApiModuleState.active)
    api_portal2 = api_fit_main.add_module(type_id=eve_portal2_id, state=consts.ApiModuleState.active)
    api_fit_psg1 = api_sol.create_fit()
    api_fit_psg1.set_ship(type_id=eve_psg_ship1_id)
    api_fit_psg2 = api_sol.create_fit()
    api_fit_psg2.set_ship(type_id=eve_psg_ship2_id)
    api_fit_psg3 = api_sol.create_fit()
    api_fit_psg3.set_ship(type_id=eve_psg_ship3_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg1.id, api_fit_psg2.id, api_fit_psg3.id])])))
    api_fit_portal1_psg_stats = api_fit_stats.jump.one().portals[api_portal1.id].fuel_use_passengers
    assert api_fit_portal1_psg_stats[api_fit_psg1.id] == 147
    assert api_fit_portal1_psg_stats[api_fit_psg2.id] is None
    assert api_fit_portal1_psg_stats[api_fit_psg3.id] == 147
    api_fit_portal2_psg_stats = api_fit_stats.jump.one().portals[api_portal2.id].fuel_use_passengers
    assert api_fit_portal2_psg_stats[api_fit_psg1.id] is None
    assert api_fit_portal2_psg_stats[api_fit_psg2.id] == 147
    assert api_fit_portal2_psg_stats[api_fit_psg3.id] == 147
    api_ship_stats = api_ship_main.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg1.id, api_fit_psg2.id, api_fit_psg3.id])])))
    api_ship_portal1_psg_stats = api_ship_stats.jump.one().portals[api_portal1.id].fuel_use_passengers
    assert api_ship_portal1_psg_stats[api_fit_psg1.id] == 147
    assert api_ship_portal1_psg_stats[api_fit_psg2.id] is None
    assert api_ship_portal1_psg_stats[api_fit_psg3.id] == 147
    api_ship_portal2_psg_stats = api_ship_stats.jump.one().portals[api_portal2.id].fuel_use_passengers
    assert api_ship_portal2_psg_stats[api_fit_psg1.id] is None
    assert api_ship_portal2_psg_stats[api_fit_psg2.id] == 147
    assert api_ship_portal2_psg_stats[api_fit_psg3.id] == 147


def test_attr_portal_flag_values_portal(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_amount)
    eve_portal_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.enable_open_jump_portal)
    eve_portal_psg_attr_id = client.mk_eve_attr()
    eve_portal_psg_ref_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_passenger_required_attr_id)
    eve_portal_fuel_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_consumption_mass_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_fuel_id = client.mk_eve_item()
    eve_portal1_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr_id})
    eve_portal2_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: -0.1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr_id})
    eve_portal3_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 0.1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr_id})
    eve_portal4_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 55, eve_portal_psg_ref_attr_id: eve_portal_psg_attr_id})
    eve_portal5_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 0, eve_portal_psg_ref_attr_id: eve_portal_psg_attr_id})
    eve_portal6_id = client.mk_eve_item(attrs={eve_portal_psg_ref_attr_id: eve_portal_psg_attr_id})
    eve_main_ship_id = client.mk_eve_ship(attrs={
        eve_range_attr_id: 5,
        eve_fuel_type_attr_id: eve_fuel_id,
        eve_fuel_use_attr_id: 1500,
        eve_portal_fuel_mult_attr_id: 0.000000001})
    eve_psg_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000, eve_portal_psg_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit_main = api_sol.create_fit()
    api_ship = api_fit_main.set_ship(type_id=eve_main_ship_id)
    api_fit_psg = api_sol.create_fit()
    api_fit_psg.set_ship(type_id=eve_psg_ship_id)
    # Verification - no bridge without portal
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    api_fit_jump_stats = api_fit_stats.jump.one()
    with check_no_field():
        api_fit_jump_stats.portals  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    api_ship_jump_stats = api_ship_stats.jump.one()
    with check_no_field():
        api_ship_jump_stats.portals  # noqa: B018
    # Action
    api_portal = api_fit_main.add_module(type_id=eve_portal1_id, state=consts.ApiModuleState.online)
    # Verification - no bridge without active portal
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    api_fit_jump_stats = api_fit_stats.jump.one()
    with check_no_field():
        api_fit_jump_stats.portals  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    api_ship_jump_stats = api_ship_stats.jump.one()
    with check_no_field():
        api_ship_jump_stats.portals  # noqa: B018
    # Action
    api_portal.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    # Action
    api_portal.change_module(type_id=eve_portal2_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    # Action
    api_portal.change_module(type_id=eve_portal3_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    # Action
    api_portal.change_module(type_id=eve_portal4_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    # Action
    api_portal.change_module(type_id=eve_portal5_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    api_fit_jump_stats = api_fit_stats.jump.one()
    with check_no_field():
        api_fit_jump_stats.portals  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    api_ship_jump_stats = api_ship_stats.jump.one()
    with check_no_field():
        api_ship_jump_stats.portals  # noqa: B018
    # Action
    api_portal.change_module(type_id=eve_portal6_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    api_fit_jump_stats = api_fit_stats.jump.one()
    with check_no_field():
        api_fit_jump_stats.portals  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    api_ship_jump_stats = api_ship_stats.jump.one()
    with check_no_field():
        api_ship_jump_stats.portals  # noqa: B018


def test_attr_fuel_absent(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = consts.EveAttr.jump_drive_consumption_amount
    eve_portal_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.enable_open_jump_portal)
    eve_portal_psg_attr_id = client.mk_eve_attr()
    eve_portal_psg_ref_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_passenger_required_attr_id)
    eve_portal_fuel_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_consumption_mass_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_fuel_id = client.mk_eve_item()
    eve_portal_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr_id})
    eve_main_ship_id = client.mk_eve_ship(attrs={
        eve_range_attr_id: 5,
        eve_fuel_type_attr_id: eve_fuel_id,
        eve_fuel_use_attr_id: 1500,
        eve_portal_fuel_mult_attr_id: 0.000000001})
    eve_psg_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000, eve_portal_psg_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit_main = api_sol.create_fit()
    api_ship = api_fit_main.set_ship(type_id=eve_main_ship_id)
    api_portal = api_fit_main.add_module(type_id=eve_portal_id, state=consts.ApiModuleState.active)
    api_fit_psg = api_sol.create_fit()
    api_fit_psg.set_ship(type_id=eve_psg_ship_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 0


def test_attr_fuel_mult_absent(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_amount)
    eve_portal_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.enable_open_jump_portal)
    eve_portal_psg_attr_id = client.mk_eve_attr()
    eve_portal_psg_ref_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_passenger_required_attr_id)
    eve_portal_fuel_mult_attr_id = consts.EveAttr.jump_portal_consumption_mass_factor
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_fuel_id = client.mk_eve_item()
    eve_portal_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr_id})
    eve_main_ship_id = client.mk_eve_ship(attrs={
        eve_range_attr_id: 5,
        eve_fuel_type_attr_id: eve_fuel_id,
        eve_fuel_use_attr_id: 1500,
        eve_portal_fuel_mult_attr_id: 0.000000001})
    eve_psg_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000, eve_portal_psg_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit_main = api_sol.create_fit()
    api_ship = api_fit_main.set_ship(type_id=eve_main_ship_id)
    api_portal = api_fit_main.add_module(type_id=eve_portal_id, state=consts.ApiModuleState.active)
    api_fit_psg = api_sol.create_fit()
    api_fit_psg.set_ship(type_id=eve_psg_ship_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 0


def test_attr_psg_ref_rounding(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_amount)
    eve_portal_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.enable_open_jump_portal)
    eve_portal_psg_attr1_id = client.mk_eve_attr()
    eve_portal_psg_attr2_id = client.mk_eve_attr(id_=0)
    eve_portal_psg_ref_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_passenger_required_attr_id)
    eve_portal_fuel_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_consumption_mass_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_fuel_id = client.mk_eve_item()
    eve_portal1_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr1_id})
    eve_portal2_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr1_id + 0.4})
    eve_portal3_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr1_id - 0.4})
    eve_portal4_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr1_id + 0.6})
    eve_portal5_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr1_id - 0.6})
    eve_portal6_id = client.mk_eve_item(attrs={eve_portal_flag_attr_id: 1})
    eve_portal7_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr2_id})
    eve_main_ship_id = client.mk_eve_ship(attrs={
        eve_range_attr_id: 5,
        eve_fuel_type_attr_id: eve_fuel_id,
        eve_fuel_use_attr_id: 1500,
        eve_portal_fuel_mult_attr_id: 0.000000001})
    eve_psg_ship1_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000, eve_portal_psg_attr1_id: 1})
    eve_psg_ship2_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit_main = api_sol.create_fit()
    api_ship_main = api_fit_main.set_ship(type_id=eve_main_ship_id)
    api_portal = api_fit_main.add_module(type_id=eve_portal1_id, state=consts.ApiModuleState.active)
    api_fit_psg = api_sol.create_fit()
    api_ship_psg = api_fit_psg.set_ship(type_id=eve_psg_ship1_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    api_ship_stats = api_ship_main.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    # Action
    api_portal.change_module(type_id=eve_portal2_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    api_ship_stats = api_ship_main.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    # Action
    api_portal.change_module(type_id=eve_portal3_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    api_ship_stats = api_ship_main.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    # Action
    api_portal.change_module(type_id=eve_portal4_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] is None
    api_ship_stats = api_ship_main.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] is None
    # Action
    api_portal.change_module(type_id=eve_portal5_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] is None
    api_ship_stats = api_ship_main.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] is None
    # Action
    api_portal.change_module(type_id=eve_portal6_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    api_ship_stats = api_ship_main.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    # Action
    api_portal.change_module(type_id=eve_portal7_id)
    api_ship_psg.change_ship(type_id=eve_psg_ship2_id)
    # Verification - 0 means no reference in EVE terms, so passengers are accepted
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    api_ship_stats = api_ship_main.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147


def test_attr_psg_ref_absent(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_amount)
    eve_portal_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.enable_open_jump_portal)
    eve_portal_psg_attr_id = client.mk_eve_attr()
    eve_portal_psg_ref_attr_id = consts.EveAttr.jump_portal_passenger_required_attr_id
    eve_portal_fuel_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_consumption_mass_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_fuel_id = client.mk_eve_item()
    eve_portal_id = client.mk_eve_item(attrs={
        eve_portal_flag_attr_id: 1,
        eve_portal_psg_ref_attr_id: eve_portal_psg_attr_id})
    eve_main_ship_id = client.mk_eve_ship(attrs={
        eve_range_attr_id: 5,
        eve_fuel_type_attr_id: eve_fuel_id,
        eve_fuel_use_attr_id: 1500,
        eve_portal_fuel_mult_attr_id: 0.000000001})
    eve_psg_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000, eve_portal_psg_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit_main = api_sol.create_fit()
    api_ship = api_fit_main.set_ship(type_id=eve_main_ship_id)
    api_portal = api_fit_main.add_module(type_id=eve_portal_id, state=consts.ApiModuleState.active)
    api_fit_psg = api_sol.create_fit()
    api_fit_psg.set_ship(type_id=eve_psg_ship_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147


def test_attr_psg_flag_values(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_amount)
    eve_portal_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.enable_open_jump_portal)
    eve_portal_psg_attr_id = client.mk_eve_attr()
    eve_portal_psg_ref_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_passenger_required_attr_id)
    eve_portal_fuel_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_consumption_mass_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_fuel_id = client.mk_eve_item()
    eve_portal_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr_id})
    eve_main_ship_id = client.mk_eve_ship(attrs={
        eve_range_attr_id: 5,
        eve_fuel_type_attr_id: eve_fuel_id,
        eve_fuel_use_attr_id: 1500,
        eve_portal_fuel_mult_attr_id: 0.000000001})
    eve_psg_ship1_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000, eve_portal_psg_attr_id: 1})
    eve_psg_ship2_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000, eve_portal_psg_attr_id: 0.1})
    eve_psg_ship3_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000, eve_portal_psg_attr_id: -0.1})
    eve_psg_ship4_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000, eve_portal_psg_attr_id: 55})
    eve_psg_ship5_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit_main = api_sol.create_fit()
    api_ship = api_fit_main.set_ship(type_id=eve_main_ship_id)
    api_portal = api_fit_main.add_module(type_id=eve_portal_id, state=consts.ApiModuleState.active)
    api_fit_psg = api_sol.create_fit()
    api_ship_psg = api_fit_psg.set_ship(type_id=eve_psg_ship1_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    # Action
    api_ship_psg.change_ship(type_id=eve_psg_ship2_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    # Action
    api_ship_psg.change_ship(type_id=eve_psg_ship3_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    # Action
    api_ship_psg.change_ship(type_id=eve_psg_ship4_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    # Action
    api_ship_psg.change_ship(type_id=eve_psg_ship5_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] is None


def test_attr_psg_flag_absent(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_amount)
    eve_portal_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.enable_open_jump_portal)
    eve_portal_psg_attr_id = client.alloc_attr_id()
    eve_portal_psg_ref_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_passenger_required_attr_id)
    eve_portal_fuel_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_consumption_mass_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_fuel_id = client.mk_eve_item()
    eve_portal_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr_id})
    eve_main_ship_id = client.mk_eve_ship(attrs={
        eve_range_attr_id: 5,
        eve_fuel_type_attr_id: eve_fuel_id,
        eve_fuel_use_attr_id: 1500,
        eve_portal_fuel_mult_attr_id: 0.000000001})
    eve_psg_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000, eve_portal_psg_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit_main = api_sol.create_fit()
    api_ship = api_fit_main.set_ship(type_id=eve_main_ship_id)
    api_portal = api_fit_main.add_module(type_id=eve_portal_id, state=consts.ApiModuleState.active)
    api_fit_psg = api_sol.create_fit()
    api_fit_psg.set_ship(type_id=eve_psg_ship_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] is None


def test_ansiblex_state(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_amount)
    eve_portal_fuel_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_consumption_mass_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_fuel_id = client.mk_eve_item()
    eve_portal_id = client.mk_eve_item(id_=consts.EveItem.st_conduit_generator)
    eve_main_ship_id = client.mk_eve_ship(attrs={
        eve_range_attr_id: 5,
        eve_fuel_type_attr_id: eve_fuel_id,
        eve_fuel_use_attr_id: 1500,
        eve_portal_fuel_mult_attr_id: 0.000000001})
    eve_psg_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit_main = api_sol.create_fit()
    api_ship = api_fit_main.set_ship(type_id=eve_main_ship_id)
    api_fit_psg = api_sol.create_fit()
    api_fit_psg.set_ship(type_id=eve_psg_ship_id)
    # Verification - no bridging without portal service
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    api_fit_jump_stats = api_fit_stats.jump.one()
    with check_no_field():
        api_fit_jump_stats.portals  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    api_ship_jump_stats = api_ship_stats.jump.one()
    with check_no_field():
        api_ship_jump_stats.portals  # noqa: B018
    # Action
    api_portal = api_fit_main.add_service(type_id=eve_portal_id, state=consts.ApiServiceState.offline)
    # Verification - no bridging still, need to have it online
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    api_fit_jump_stats = api_fit_stats.jump.one()
    with check_no_field():
        api_fit_jump_stats.portals  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    api_ship_jump_stats = api_ship_stats.jump.one()
    with check_no_field():
        api_ship_jump_stats.portals  # noqa: B018
    # Action
    api_portal.change_service(state=consts.ApiServiceState.online)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    assert api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers[api_fit_psg.id] == 147
    # Action
    api_portal.change_service(state=consts.ApiServiceState.offline)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    api_fit_jump_stats = api_fit_stats.jump.one()
    with check_no_field():
        api_fit_jump_stats.portals  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg.id])])))
    api_ship_jump_stats = api_ship_stats.jump.one()
    with check_no_field():
        api_ship_jump_stats.portals  # noqa: B018


def test_ansiblex_mass_limit(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_amount)
    eve_portal_fuel_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_consumption_mass_factor)
    eve_portal_mass_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.gate_max_jump_mass)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_fuel_id = client.mk_eve_item()
    eve_portal_id = client.mk_eve_item(id_=consts.EveItem.st_conduit_generator)
    eve_main_ship_id = client.mk_eve_ship(attrs={
        eve_range_attr_id: 5,
        eve_fuel_type_attr_id: eve_fuel_id,
        eve_fuel_use_attr_id: 1500,
        eve_portal_fuel_mult_attr_id: 0.000000001,
        eve_portal_mass_limit_attr_id: 1480000000})
    eve_psg_ship1_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000})
    eve_psg_ship2_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 2000000000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit_main = api_sol.create_fit()
    api_ship = api_fit_main.set_ship(type_id=eve_main_ship_id)
    api_fit_psg1 = api_sol.create_fit()
    api_fit_psg1.set_ship(type_id=eve_psg_ship1_id)
    api_fit_psg2 = api_sol.create_fit()
    api_fit_psg2.set_ship(type_id=eve_psg_ship2_id)
    api_portal = api_fit_main.add_service(type_id=eve_portal_id, state=consts.ApiServiceState.online)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg1.id, api_fit_psg2.id])])))
    api_fit_fuel_stat = api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers
    assert api_fit_fuel_stat[api_fit_psg1.id] == 147
    assert api_fit_fuel_stat[api_fit_psg2.id] is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg1.id, api_fit_psg2.id])])))
    api_ship_fuel_stat = api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers
    assert api_ship_fuel_stat[api_fit_psg1.id] == 147
    assert api_ship_fuel_stat[api_fit_psg2.id] is None


def test_unexpected_fit(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_amount)
    eve_portal_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.enable_open_jump_portal)
    eve_portal_psg_attr_id = client.mk_eve_attr()
    eve_portal_psg_ref_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_passenger_required_attr_id)
    eve_portal_fuel_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_portal_consumption_mass_factor)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_fuel_id = client.mk_eve_item()
    eve_portal_id = client.mk_eve_item(
        attrs={eve_portal_flag_attr_id: 1, eve_portal_psg_ref_attr_id: eve_portal_psg_attr_id})
    eve_main_ship_id = client.mk_eve_ship(attrs={
        eve_range_attr_id: 5,
        eve_fuel_type_attr_id: eve_fuel_id,
        eve_fuel_use_attr_id: 1500,
        eve_portal_fuel_mult_attr_id: 0.000000001})
    eve_psg_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000, eve_portal_psg_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit_main = api_sol.create_fit()
    api_ship_main = api_fit_main.set_ship(type_id=eve_main_ship_id)
    api_portal = api_fit_main.add_module(type_id=eve_portal_id, state=consts.ApiModuleState.active)
    api_fit_psg1 = api_sol.create_fit()
    api_fit_psg1.set_ship(type_id=eve_psg_ship_id)
    api_fit_psg2 = api_sol.create_fit()
    api_fit_psg2.set_ship(type_id=eve_psg_ship_id)
    # Verification
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg1.id, api_fit_psg2.id])])))
    api_fit_psg_stats = api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers
    assert api_fit_psg_stats[api_fit_psg1.id] == 147
    assert api_fit_psg_stats[api_fit_psg2.id] == 147
    api_ship_stats = api_ship_main.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg1.id, api_fit_psg2.id])])))
    api_ship_psg_stats = api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers
    assert api_ship_psg_stats[api_fit_psg1.id] == 147
    assert api_ship_psg_stats[api_fit_psg2.id] == 147
    # Action
    api_fit_psg1.remove()
    # Verification - removed fit should be ignored
    api_fit_stats = api_fit_main.get_stats(options=FitStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg1.id, api_fit_psg2.id])])))
    api_fit_psg_stats = api_fit_stats.jump.one().portals[api_portal.id].fuel_use_passengers
    assert len(api_fit_psg_stats) == 1
    assert api_fit_psg_stats[api_fit_psg2.id] == 147
    api_ship_stats = api_ship_main.get_stats(options=ItemStatsOptions(
        jump=(True, [StatsOptionJump(passenger_fit_ids=[api_fit_psg1.id, api_fit_psg2.id])])))
    api_ship_psg_stats = api_ship_stats.jump.one().portals[api_portal.id].fuel_use_passengers
    assert len(api_ship_psg_stats) == 1
    assert api_ship_psg_stats[api_fit_psg2.id] == 147
