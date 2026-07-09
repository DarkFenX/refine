from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions


def test_self_ship(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_range)
    eve_fuel_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.jump_drive_consumption_type)
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
