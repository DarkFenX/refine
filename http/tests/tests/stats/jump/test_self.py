from fw import approx, check_no_field
from fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionJump


def test_ship_ranges(client, consts):
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


def test_struct(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
    eve_fuel_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_amount)
    eve_fuel_id = client.mk_eve_item()
    eve_struct_id = client.mk_eve_struct(
        attrs={eve_range_attr_id: 5, eve_fuel_type_attr_id: eve_fuel_id, eve_fuel_use_attr_id: 3000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(jump=True))
    api_fit_jump_stat = api_fit_stats.jump.one()
    with check_no_field():
        api_fit_jump_stat.self  # noqa: B018
    api_struct_stats = api_struct.get_stats(options=ItemStatsOptions(jump=True))
    api_struct_jump_stat = api_struct_stats.jump.one()
    with check_no_field():
        api_struct_jump_stat.self  # noqa: B018
