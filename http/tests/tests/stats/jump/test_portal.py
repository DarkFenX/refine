from fw import approx, check_no_field
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
    eve_main_portal_id = client.mk_eve_item(attrs={eve_portal_flag_attr_id: 1})
    eve_psg_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 19500000})
    client.create_sources()
    api_sol = client.create_sol()
    api_main_fit = api_sol.create_fit()
    api_main_ship = api_main_fit.set_ship(type_id=eve_main_ship1_id)
    api_main_portal = api_main_fit.add_module(type_id=eve_main_portal_id, state=consts.ApiModuleState.active)
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
     api_fit_jump_rounding) = api_main_fit.get_stats(options=FitStatsOptions(jump=(True, api_jump_options))).jump
    assert api_fit_jump_default.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(147)
    with check_no_field():
        api_fit_jump_excessive.portals  # noqa: B018
    assert api_fit_jump_max_num.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(147)
    assert api_fit_jump_max_spec.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(147)
    assert api_fit_jump_med.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(59)
    assert api_fit_jump_low.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(3)
    assert api_fit_jump_zero.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(0)
    assert api_fit_jump_rounding.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(33)
    (api_ship_jump_default,
     api_ship_jump_excessive,
     api_ship_jump_max_num,
     api_ship_jump_max_spec,
     api_ship_jump_med,
     api_ship_jump_low,
     api_ship_jump_zero,
     api_ship_jump_rounding) = api_main_ship.get_stats(options=ItemStatsOptions(jump=(True, api_jump_options))).jump
    assert api_ship_jump_default.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(147)
    with check_no_field():
        api_ship_jump_excessive.portals  # noqa: B018
    assert api_ship_jump_max_num.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(147)
    assert api_ship_jump_max_spec.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(147)
    assert api_ship_jump_med.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(59)
    assert api_ship_jump_low.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(3)
    assert api_ship_jump_zero.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(0)
    assert api_ship_jump_rounding.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(33)
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
     api_fit_jump_rounding) = api_main_fit.get_stats(options=FitStatsOptions(jump=(True, api_jump_options))).jump
    assert api_fit_jump_default.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(197)
    with check_no_field():
        api_fit_jump_excessive.portals  # noqa: B018
    assert api_fit_jump_max_num.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(197)
    assert api_fit_jump_max_spec.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(197)
    assert api_fit_jump_med.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(109)
    assert api_fit_jump_low.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(53)
    assert api_fit_jump_zero.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(50)
    assert api_fit_jump_rounding.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(83)
    (api_ship_jump_default,
     api_ship_jump_excessive,
     api_ship_jump_max_num,
     api_ship_jump_max_spec,
     api_ship_jump_med,
     api_ship_jump_low,
     api_ship_jump_zero,
     api_ship_jump_rounding) = api_main_ship.get_stats(options=ItemStatsOptions(jump=(True, api_jump_options))).jump
    assert api_ship_jump_default.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(197)
    with check_no_field():
        api_ship_jump_excessive.portals  # noqa: B018
    assert api_ship_jump_max_num.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(197)
    assert api_ship_jump_max_spec.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(197)
    assert api_ship_jump_med.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(109)
    assert api_ship_jump_low.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(53)
    assert api_ship_jump_zero.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(50)
    assert api_ship_jump_rounding.portals[api_main_portal.id].fuel_use_passengers[api_psg_fit.id] == approx(83)


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
    eve_main_portal_id = client.mk_eve_item(attrs={
        eve_portal_flag_attr_id: 1,
        eve_portal_psg_ref_attr_id: eve_portal_psg_attr_id})
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
    api_portal = api_fit_main.add_module(type_id=eve_main_portal_id, state=consts.ApiModuleState.active)
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
