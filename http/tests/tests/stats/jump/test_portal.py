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
