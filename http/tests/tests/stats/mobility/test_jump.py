from fw import approx, check_no_field
from fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionJump


def test_general_ship_modified_range(client, consts):
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


def test_general_fuel_type_values(client, consts):
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


def test_self_ranges(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_amount)
    eve_fuel_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_range_attr_id: 5, eve_fuel_type_attr_id: eve_fuel_id, eve_fuel_use_attr_id: 3000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_jump_options = [
        StatsOptionJump(),
        StatsOptionJump(range=5.1),
        StatsOptionJump(range=5),
        StatsOptionJump(range='max'),
        StatsOptionJump(range=2),
        StatsOptionJump(range=0.1),
        StatsOptionJump(range=0),
        StatsOptionJump(range=1.00001)]
    (api_fit_jump_default,
     api_fit_jump_excessive,
     api_fit_jump_max_num,
     api_fit_jump_max_spec,
     api_fit_jump_med,
     api_fit_jump_low,
     api_fit_jump_zero,
     api_fit_jump_rounding) = api_fit.get_stats(options=FitStatsOptions(jump=(True, api_jump_options))).jump
    assert api_fit_jump_default.self.fuel_use == approx(15000)
    with check_no_field():
        api_fit_jump_excessive.self  # noqa: B018
    assert api_fit_jump_max_num.self.fuel_use == approx(15000)
    assert api_fit_jump_max_spec.self.fuel_use == approx(15000)
    assert api_fit_jump_med.self.fuel_use == approx(6000)
    assert api_fit_jump_low.self.fuel_use == approx(300)
    assert api_fit_jump_zero.self.fuel_use == approx(0)
    assert api_fit_jump_rounding.self.fuel_use == approx(3001)
    (api_ship_jump_default,
     api_ship_jump_excessive,
     api_ship_jump_max_num,
     api_ship_jump_max_spec,
     api_ship_jump_med,
     api_ship_jump_low,
     api_ship_jump_zero,
     api_ship_jump_rounding) = api_ship.get_stats(options=ItemStatsOptions(jump=(True, api_jump_options))).jump
    assert api_ship_jump_default.self.fuel_use == approx(15000)
    with check_no_field():
        api_ship_jump_excessive.self  # noqa: B018
    assert api_ship_jump_max_num.self.fuel_use == approx(15000)
    assert api_ship_jump_max_spec.self.fuel_use == approx(15000)
    assert api_ship_jump_med.self.fuel_use == approx(6000)
    assert api_ship_jump_low.self.fuel_use == approx(300)
    assert api_ship_jump_zero.self.fuel_use == approx(0)
    assert api_ship_jump_rounding.self.fuel_use == approx(3001)


def test_conduit_ship_flag_values(client, consts):
    eve_conduit_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.enable_perform_conduit_jump)
    eve_conduit_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.conduit_jump_passenger_count)
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.conduit_jump_drive_consumption_amount)
    eve_fuel_id = client.mk_eve_item()

    def mk_ship(*, conduit_flag: float | None) -> int:
        attrs = {
            eve_range_attr_id: 5,
            eve_fuel_type_attr_id: eve_fuel_id,
            eve_fuel_use_attr_id: 3000,
            eve_conduit_count_attr_id: 30}
        if conduit_flag is not None:
            attrs[eve_conduit_flag_attr_id] = conduit_flag
        return client.mk_eve_ship(attrs=attrs)

    eve_ship1_id = mk_ship(conduit_flag=1)
    eve_ship2_id = mk_ship(conduit_flag=-0.1)
    eve_ship3_id = mk_ship(conduit_flag=0.1)
    eve_ship4_id = mk_ship(conduit_flag=55)
    eve_ship5_id = mk_ship(conduit_flag=0)
    eve_ship6_id = mk_ship(conduit_flag=None)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_fit_stats.jump.one().conduit.fuel_use_self == approx(15000)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_ship_stats.jump.one().conduit.fuel_use_self == approx(15000)
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_fit_stats.jump.one().conduit.fuel_use_self == approx(15000)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_ship_stats.jump.one().conduit.fuel_use_self == approx(15000)
    # Action
    api_ship.change_ship(type_id=eve_ship3_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_fit_stats.jump.one().conduit.fuel_use_self == approx(15000)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_ship_stats.jump.one().conduit.fuel_use_self == approx(15000)
    # Action
    api_ship.change_ship(type_id=eve_ship4_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_fit_stats.jump.one().conduit.fuel_use_self == approx(15000)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_ship_stats.jump.one().conduit.fuel_use_self == approx(15000)
    # Action
    api_ship.change_ship(type_id=eve_ship5_id)
    # Verification - no conduit with 0 flag
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    api_fit_jump_stats = api_fit_stats.jump.one()
    with check_no_field():
        api_fit_jump_stats.conduit  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    api_ship_jump_stats = api_ship_stats.jump.one()
    with check_no_field():
        api_ship_jump_stats.conduit  # noqa: B018
    # Action
    api_ship.change_ship(type_id=eve_ship6_id)
    # Verification - no conduit with no flag
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    api_fit_jump_stats = api_fit_stats.jump.one()
    with check_no_field():
        api_fit_jump_stats.conduit  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    api_ship_jump_stats = api_ship_stats.jump.one()
    with check_no_field():
        api_ship_jump_stats.conduit  # noqa: B018


def test_conduit_bridge_flag_values(client, consts):
    eve_conduit_flag_attr_id = client.mk_eve_attr(id_=consts.EveAttr.enable_perform_conduit_jump)
    eve_conduit_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.conduit_jump_passenger_count)
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.conduit_jump_drive_consumption_amount)
    eve_fuel_id = client.mk_eve_item()
    eve_bridge1_id = client.mk_eve_item(attrs={eve_conduit_flag_attr_id: 1})
    eve_bridge2_id = client.mk_eve_item(attrs={eve_conduit_flag_attr_id: -0.1})
    eve_bridge3_id = client.mk_eve_item(attrs={eve_conduit_flag_attr_id: 0.1})
    eve_bridge4_id = client.mk_eve_item(attrs={eve_conduit_flag_attr_id: 55})
    eve_bridge5_id = client.mk_eve_item(attrs={eve_conduit_flag_attr_id: 0})
    eve_bridge6_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={
        eve_range_attr_id: 5,
        eve_fuel_type_attr_id: eve_fuel_id,
        eve_fuel_use_attr_id: 3000,
        eve_conduit_count_attr_id: 30})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification - no conduit without bridge
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    api_fit_jump_stats = api_fit_stats.jump.one()
    with check_no_field():
        api_fit_jump_stats.conduit  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    api_ship_jump_stats = api_ship_stats.jump.one()
    with check_no_field():
        api_ship_jump_stats.conduit  # noqa: B018
    # Action
    api_bridge = api_fit.add_module(type_id=eve_bridge1_id, state=consts.ApiModuleState.offline)
    # Verification - conduit is enabled even when bridge is offline
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_fit_stats.jump.one().conduit.fuel_use_self == approx(15000)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_ship_stats.jump.one().conduit.fuel_use_self == approx(15000)
    # Action
    api_bridge.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_fit_stats.jump.one().conduit.fuel_use_self == approx(15000)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_ship_stats.jump.one().conduit.fuel_use_self == approx(15000)
    # Action
    api_bridge.change_module(type_id=eve_bridge2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_fit_stats.jump.one().conduit.fuel_use_self == approx(15000)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_ship_stats.jump.one().conduit.fuel_use_self == approx(15000)
    # Action
    api_bridge.change_module(type_id=eve_bridge3_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_fit_stats.jump.one().conduit.fuel_use_self == approx(15000)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_ship_stats.jump.one().conduit.fuel_use_self == approx(15000)
    # Action
    api_bridge.change_module(type_id=eve_bridge4_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    assert api_fit_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_fit_stats.jump.one().conduit.fuel_use_self == approx(15000)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    assert api_ship_stats.jump.one().conduit.max_passengers == approx(30)
    assert api_ship_stats.jump.one().conduit.fuel_use_self == approx(15000)
    # Action
    api_bridge.change_module(type_id=eve_bridge5_id)
    # Verification - no conduit with 0 flag
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    api_fit_jump_stats = api_fit_stats.jump.one()
    with check_no_field():
        api_fit_jump_stats.conduit  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    api_ship_jump_stats = api_ship_stats.jump.one()
    with check_no_field():
        api_ship_jump_stats.conduit  # noqa: B018
    # Action
    api_bridge.change_module(type_id=eve_bridge6_id)
    # Verification - no conduit with no flag
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    api_fit_jump_stats = api_fit_stats.jump.one()
    with check_no_field():
        api_fit_jump_stats.conduit  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(jump=True))
    api_ship_jump_stats = api_ship_stats.jump.one()
    with check_no_field():
        api_ship_jump_stats.conduit  # noqa: B018


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
