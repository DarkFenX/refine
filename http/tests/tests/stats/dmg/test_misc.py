from fw import check_no_field
from fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions
from tests.stats.dmg import make_eve_smartbomb, setup_dmg_basics


def test_not_loaded(client, consts):
    eve_item_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    api_fighter = api_fit.add_fighter(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [0, 0, 0, 0]
    assert api_fleet_stats.volley.one() == [0, 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_fit_stats.volley.one() == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps is None
    assert api_module_stats.volley is None
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_drone_stats.dps is None
    assert api_drone_stats.volley is None
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_fighter_stats.dps is None
    assert api_fighter_stats.volley is None


def test_not_requested(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_smartbomb(client=client, basic_info=eve_basic_info, dmgs=(45, 45, 45, 45), cycle_time=7500)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=False, volley=False))
    with check_no_field():
        api_fleet_stats.dps  # noqa: B018
    with check_no_field():
        api_fleet_stats.volley  # noqa: B018
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=False, volley=False))
    with check_no_field():
        api_fit_stats.dps  # noqa: B018
    with check_no_field():
        api_fit_stats.volley  # noqa: B018
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=False, volley=False))
    with check_no_field():
        api_module_stats.dps  # noqa: B018
    with check_no_field():
        api_module_stats.volley  # noqa: B018
