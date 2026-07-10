from fw import approx, check_no_field
from fw.api import FitStatsOptions, ItemStatsOptions


def test_ship_modified_range(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_jump_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_range_attr_id)
    eve_jump_mod_effect_id = client.mk_eve_effect(mod_info=[eve_jump_mod])
    eve_jump_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50}, eff_ids=[eve_jump_mod_effect_id])
    eve_fuel_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_range_attr_id: 5, eve_fuel_type_attr_id: eve_fuel_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump.one().max_range == approx(5)
    assert api_fit_stats.jump.one().fuel_type_id == eve_fuel_id
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump.one().max_range == approx(5)
    assert api_ship_stats.jump.one().fuel_type_id == eve_fuel_id
    # Action
    api_fit.add_rig(type_id=eve_jump_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump.one().max_range == approx(7.5)
    assert api_fit_stats.jump.one().fuel_type_id == eve_fuel_id
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump.one().max_range == approx(7.5)
    assert api_ship_stats.jump.one().fuel_type_id == eve_fuel_id


def test_attr_range_absent(client, consts):
    eve_range_attr_id = consts.EveAttr.jump_drive_range
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_range_attr_id: 5, eve_fuel_type_attr_id: eve_fuel_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump is None


def test_attr_fuel_type_values(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_range_attr_id: 5, eve_fuel_type_attr_id: 2.4})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_range_attr_id: 5, eve_fuel_type_attr_id: 2.6})
    eve_ship3_id = client.mk_eve_ship(attrs={eve_range_attr_id: 5, eve_fuel_type_attr_id: 3.1})
    eve_ship4_id = client.mk_eve_ship(attrs={eve_range_attr_id: 5, eve_fuel_type_attr_id: 0.4})
    eve_ship5_id = client.mk_eve_ship(attrs={eve_range_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump.one().fuel_type_id == 2
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump.one().fuel_type_id == 2
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump.one().fuel_type_id == 3
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump.one().fuel_type_id == 3
    # Action
    api_ship.change_ship(type_id=eve_ship3_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump.one().fuel_type_id == 3
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump.one().fuel_type_id == 3
    # Action
    api_ship.change_ship(type_id=eve_ship4_id)
    # Verification - fuel type ID of 0 (after rounding) means no jump drive
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump is None
    # Action
    api_ship.change_ship(type_id=eve_ship5_id)
    # Verification - not specified fuel type means no jump drive
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump is None


def test_attr_fuel_type_absent(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = consts.EveAttr.jump_drive_consumption_type
    eve_fuel_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_range_attr_id: 5, eve_fuel_type_attr_id: eve_fuel_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump is None


def test_ship_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_conduit_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.enable_perform_conduit_jump)
    eve_bridge_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.enable_open_jump_portal)
    client.mk_eve_attr(id_=consts.EveAttr.conduit_jump_passenger_count)
    client.mk_eve_attr(id_=consts.EveAttr.conduit_jump_drive_consumption_amount)
    eve_bridge_id = client.mk_eve_item(
        attrs={eve_conduit_flag_attr_id: 1, eve_bridge_flag_attr_id: 1})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_bridge_id, state=consts.ApiModuleState.active)
    # Verification - no conduit without bridge
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump is None


def test_not_requested(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_range_attr_id: 5, eve_fuel_type_attr_id: eve_fuel_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=False))
    with check_no_field():
        api_fit_stats.jump  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=False))
    with check_no_field():
        api_ship_stats.jump  # noqa: B018
