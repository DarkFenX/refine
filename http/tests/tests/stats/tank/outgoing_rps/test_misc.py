from tests import check_no_field
from tests.fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions
from tests.tests.stats.tank import make_eve_remote_ar, setup_tank_basics


def test_not_requested(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=376, cycle_time=6000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=False))
    with check_no_field():
        api_fleet_stats.outgoing_rps  # noqa: B018
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=False))
    with check_no_field():
        api_fit_stats.outgoing_rps  # noqa: B018
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_rps=False))
    with check_no_field():
        api_module_stats.outgoing_rps  # noqa: B018
