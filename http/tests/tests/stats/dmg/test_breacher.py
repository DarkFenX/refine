from tests import approx
from tests.fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatDmgItemKinds,
    StatsOptionFitDps,
    StatsOptionFitVolley,
    StatsOptionItemDps,
    StatsOptionItemVolley,
)
from tests.tests.stats.dmg import make_eve_breacher, make_eve_launcher, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=25, cycle_time=10000, reload_time=30000)
    eve_charge_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=1000, dmg_rel=1, dmg_duration=75000, volume=0.5)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one().breacher == [approx(1000), approx(0.01)]
    assert api_fit_stats.volley.one().breacher == [approx(1000), approx(0.01)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_charge_stats.dps.one().breacher == [approx(1000), approx(0.01)]
    assert api_charge_stats.volley.one().breacher == [approx(1000), approx(0.01)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one().breacher is None
    assert api_fit_stats.volley.one().breacher is None
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(), StatsOptionItemDps(ignore_state=True)]),
        volley=(True, [StatsOptionItemVolley(), StatsOptionItemVolley(ignore_state=True)])))
    api_charge_dps_normal, api_charge_dps_ignored = api_charge_stats.dps
    assert api_charge_dps_normal.breacher is None
    assert api_charge_dps_ignored.breacher == [approx(1000), approx(0.01)]
    api_charge_volley_normal, api_charge_volley_ignored = api_charge_stats.volley
    assert api_charge_volley_normal.breacher is None
    assert api_charge_volley_ignored.breacher == [approx(1000), approx(0.01)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one().breacher == [approx(1000), approx(0.01)]
    assert api_fit_stats.volley.one().breacher == [approx(1000), approx(0.01)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_charge_stats.dps.one().breacher == [approx(1000), approx(0.01)]
    assert api_charge_stats.volley.one().breacher == [approx(1000), approx(0.01)]


def test_stacking_simple(client, consts):
    # Simple scenario is when best relative/absolute breachers are infinitely cycling; in this case,
    # breacher "sim" is not used
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=25, cycle_time=10000, reload_time=30000)
    eve_charge1_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=1000, dmg_rel=0.8, dmg_duration=75000, volume=0.5)
    eve_charge2_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=800, dmg_rel=1, dmg_duration=75000, volume=0.5)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge1_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one().breacher == [approx(1000), approx(0.01)]
    assert api_fit_stats.volley.one().breacher == [approx(1000), approx(0.01)]


def test_stacking_complex_realistic(client, consts):
    # Realistic case of 2 Tholoses - one with higher DPS and bad reload/duration skills, and another
    # with permanently applied breacher with worse DPS
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module1_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=4, cycle_time=24000, reload_time=30000)
    eve_module2_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=4, cycle_time=12000, reload_time=30000)
    eve_charge1_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=200, dmg_rel=0.75, dmg_duration=50000, volume=0.1)
    eve_charge2_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=160, dmg_rel=0.6, dmg_duration=75000, volume=0.1)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit1.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge1_id)
    api_fit2 = api_sol.create_fit()
    api_fit2.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge2_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    # Verification
    api_fit_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=(True, [StatsOptionFitDps(reload=True)])))
    assert api_fit_stats.dps.one().breacher == [approx(199.838384), approx(0.007493939)]


def test_stacking_complex_different_multiple_downtimes(client, consts):
    # Both breachers in this case have different downtime - shorter on every module cycle, longer
    # during reload. They partially cover each other's downtime, so when both are on, they deal more
    # damage
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module1_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=10, cycle_time=30000, reload_time=15000)
    eve_charge1_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=1000, dmg_rel=0.5, dmg_duration=15000, volume=1)
    eve_module2_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=10, cycle_time=20000, reload_time=10000)
    eve_charge2_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=500, dmg_rel=1, dmg_duration=10000, volume=1)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(
        type_id=eve_module1_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge1_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(), StatsOptionFitDps(reload=True)])))
    api_fit_dps_burst, api_fit_dps_reload = api_fit_stats.dps
    assert api_fit_dps_burst.breacher == [approx(500), approx(0.0025)]
    assert api_fit_dps_reload.breacher == [approx(476.190476), approx(0.002380952)]
    # Action
    api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(), StatsOptionFitDps(reload=True)])))
    api_fit_dps_burst, api_fit_dps_reload = api_fit_stats.dps
    assert api_fit_dps_burst.breacher == [approx(625), approx(0.00625)]
    assert api_fit_dps_reload.breacher == [approx(527.777778), approx(0.005277778)]
    # Action
    api_module1.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(), StatsOptionFitDps(reload=True)])))
    api_fit_dps_burst, api_fit_dps_reload = api_fit_stats.dps
    assert api_fit_dps_burst.breacher == [approx(250), approx(0.005)]
    assert api_fit_dps_reload.breacher == [approx(238.095238), approx(0.004761905)]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=25, cycle_time=10000, reload_time=30000)
    eve_charge_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=1000, dmg_rel=1, dmg_duration=75000, volume=0.5)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [
            StatsOptionFitDps(),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=True, breacher=False)),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=False, breacher=True))]),
        volley=(True, [
            StatsOptionFitVolley(),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=True, breacher=False)),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=False, breacher=True))])))
    api_fit_dps_default, api_fit_dps_disabled, api_fit_dps_enabled = api_fit_stats.dps
    assert api_fit_dps_default.breacher == [approx(1000), approx(0.01)]
    assert api_fit_dps_disabled.breacher is None
    assert api_fit_dps_enabled.breacher == [approx(1000), approx(0.01)]
    api_fit_volley_default, api_fit_volley_disabled, api_fit_volley_enabled = api_fit_stats.volley
    assert api_fit_volley_default.breacher == [approx(1000), approx(0.01)]
    assert api_fit_volley_disabled.breacher is None
    assert api_fit_volley_enabled.breacher == [approx(1000), approx(0.01)]


def test_include_charges(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=25, cycle_time=10000, reload_time=30000)
    eve_charge_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=1000, dmg_rel=1, dmg_duration=75000, volume=0.5)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification - need to include charges for module to show dps, since it's on-charge effect
    # which deals damage. For charges, this option doesn't do anything
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(include_charges=False), StatsOptionItemDps(include_charges=True)]),
        volley=(True, [StatsOptionItemVolley(include_charges=False), StatsOptionItemVolley(include_charges=True)])))
    api_module_dps_without, api_module_dps_with = api_module_stats.dps
    assert api_module_dps_without.breacher is None
    assert api_module_dps_with.breacher == [approx(1000), approx(0.01)]
    api_module_volley_without, api_module_volley_with = api_module_stats.volley
    assert api_module_volley_without.breacher is None
    assert api_module_volley_with.breacher == [approx(1000), approx(0.01)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(include_charges=False), StatsOptionItemDps(include_charges=True)]),
        volley=(True, [StatsOptionItemVolley(include_charges=False), StatsOptionItemVolley(include_charges=True)])))
    api_charge_dps_without, api_charge_dps_with = api_charge_stats.dps
    assert api_charge_dps_without.breacher == [approx(1000), approx(0.01)]
    assert api_charge_dps_with.breacher == [approx(1000), approx(0.01)]
    api_charge_volley_without, api_charge_volley_with = api_charge_stats.volley
    assert api_charge_volley_without.breacher == [approx(1000), approx(0.01)]
    assert api_charge_volley_with.breacher == [approx(1000), approx(0.01)]


def test_reload(client, consts):
    # Realistic case of Tholos with poor breacher duration/reload skills - when it has to reload,
    # there is downtime, so overall dps changes
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=4, cycle_time=24000, reload_time=30000)
    eve_charge_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=200, dmg_rel=0.75, dmg_duration=50000, volume=0.1)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(), StatsOptionFitDps(reload=True)])))
    api_fit_dps_burst, api_fit_dps_reload = api_fit_stats.dps
    assert api_fit_dps_burst.breacher == [approx(200), approx(0.0075)]
    assert api_fit_dps_reload.breacher == [approx(199.191919), approx(0.007469697)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(), StatsOptionItemDps(reload=True)])))
    api_charge_dps_burst, api_charge_dps_reload = api_charge_stats.dps
    assert api_charge_dps_burst.breacher == [approx(200), approx(0.0075)]
    assert api_charge_dps_reload.breacher == [approx(199.191919), approx(0.007469697)]
