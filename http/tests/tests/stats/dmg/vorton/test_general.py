from fw import approx
from fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatDmgItemKinds,
    StatsOptionFitDps,
    StatsOptionFitVolley,
    StatsOptionItemDps,
    StatsOptionItemVolley,
    StatTimeBurst,
    StatTimeSim,
)
from tests.stats.dmg import make_eve_charge_normal, make_eve_vorton, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_vorton(
        client=client, basic_info=eve_basic_info, dmg_mult=2.62, capacity=1, cycle_time=6000, reload_time=5000)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(402, 0, 379, 0), volume=0.0125)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [approx(175.54), 0, approx(165.496667), 0]
    assert api_fleet_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [approx(175.54), 0, approx(165.496667), 0]
    assert api_fit_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [approx(175.54), 0, approx(165.496667), 0]
    assert api_module_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [0, 0, 0, 0]
    assert api_fleet_stats.volley.one() == [0, 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_fit_stats.volley.one() == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(), StatsOptionItemDps(ignore_state=True)]),
        volley=(True, [StatsOptionItemVolley(), StatsOptionItemVolley(ignore_state=True)])))
    api_module_dps_normal, api_module_dps_ignored = api_module_stats.dps
    assert api_module_dps_normal == [0, 0, 0, 0]
    assert api_module_dps_ignored == [approx(175.54), 0, approx(165.496667), 0]
    api_module_volley_normal, api_module_volley_ignored = api_module_stats.volley
    assert api_module_volley_normal == [0, 0, 0, 0]
    assert api_module_volley_ignored == [approx(1053.24), 0, approx(992.98), 0]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [approx(175.54), 0, approx(165.496667), 0]
    assert api_fleet_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [approx(175.54), 0, approx(165.496667), 0]
    assert api_fit_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [approx(175.54), 0, approx(165.496667), 0]
    assert api_module_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_vorton(
        client=client, basic_info=eve_basic_info, dmg_mult=2.62, capacity=1, cycle_time=6000, reload_time=5000)
    eve_charge1_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(402, 0, 379, 0), volume=0.0125)
    eve_charge2_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(218, 0, 212, 0), volume=0.0125)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge1_id)
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge2_id)
    api_fit2 = api_sol.create_fit()
    api_fit2.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge1_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [approx(446.273333), 0, approx(423.566667), 0]
    assert api_fleet_stats.volley.one() == [approx(2677.64), 0, approx(2541.4), 0]
    api_fit1_stats = api_fit1.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit1_stats.dps.one() == [approx(270.733333), 0, approx(258.07), 0]
    assert api_fit1_stats.volley.one() == [approx(1624.4), 0, approx(1548.42), 0]
    api_fit2_stats = api_fit2.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit2_stats.dps.one() == [approx(175.54), 0, approx(165.496667), 0]
    assert api_fit2_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_vorton(
        client=client, basic_info=eve_basic_info, dmg_mult=2.62, capacity=1, cycle_time=6000, reload_time=5000)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(402, 0, 379, 0), volume=0.0125)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [
            StatsOptionFitDps(),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=True, vorton=False)),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=False, vorton=True))]),
        volley=(True, [
            StatsOptionFitVolley(),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=True, vorton=False)),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=False, vorton=True))])))
    api_fleet_dps_default, api_fleet_dps_disabled, api_fleet_dps_enabled = api_fleet_stats.dps
    assert api_fleet_dps_default == [approx(175.54), 0, approx(165.496667), 0]
    assert api_fleet_dps_disabled == [0, 0, 0, 0]
    assert api_fleet_dps_enabled == [approx(175.54), 0, approx(165.496667), 0]
    api_fleet_volley_default, api_fleet_volley_disabled, api_fleet_volley_enabled = api_fleet_stats.volley
    assert api_fleet_volley_default == [approx(1053.24), 0, approx(992.98), 0]
    assert api_fleet_volley_disabled == [0, 0, 0, 0]
    assert api_fleet_volley_enabled == [approx(1053.24), 0, approx(992.98), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [
            StatsOptionFitDps(),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=True, vorton=False)),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=False, vorton=True))]),
        volley=(True, [
            StatsOptionFitVolley(),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=True, vorton=False)),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=False, vorton=True))])))
    api_fit_dps_default, api_fit_dps_disabled, api_fit_dps_enabled = api_fit_stats.dps
    assert api_fit_dps_default == [approx(175.54), 0, approx(165.496667), 0]
    assert api_fit_dps_disabled == [0, 0, 0, 0]
    assert api_fit_dps_enabled == [approx(175.54), 0, approx(165.496667), 0]
    api_fit_volley_default, api_fit_volley_disabled, api_fit_volley_enabled = api_fit_stats.volley
    assert api_fit_volley_default == [approx(1053.24), 0, approx(992.98), 0]
    assert api_fit_volley_disabled == [0, 0, 0, 0]
    assert api_fit_volley_enabled == [approx(1053.24), 0, approx(992.98), 0]


def test_time(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_vorton(
        client=client, basic_info=eve_basic_info, dmg_mult=2.62, capacity=1, cycle_time=6000, reload_time=5000)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(402, 0, 379, 0), volume=0.0125)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification - burst stats (reload is ignored)
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeBurst())]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeBurst())])))
    assert api_fleet_stats.dps.one() == [approx(175.54), 0, approx(165.496667), 0]
    assert api_fleet_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeBurst())]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeBurst())])))
    assert api_fit_stats.dps.one() == [approx(175.54), 0, approx(165.496667), 0]
    assert api_fit_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeBurst())]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeBurst())])))
    assert api_module_stats.dps.one() == [approx(175.54), 0, approx(165.496667), 0]
    assert api_module_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    # Verification - sim without time means stats with reload time are considered
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=None))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=None))])))
    assert api_fleet_stats.dps.one() == [approx(173.730309), 0, approx(163.790515), 0]
    assert api_fleet_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=None))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=None))])))
    assert api_fit_stats.dps.one() == [approx(173.730309), 0, approx(163.790515), 0]
    assert api_fit_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=None))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=None))])))
    assert api_module_stats.dps.one() == [approx(173.730309), 0, approx(163.790515), 0]
    assert api_module_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    # Verification - just after first hit landed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=1))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=1))])))
    assert api_fleet_stats.dps.one() == [approx(1053.24), 0, approx(992.98), 0]
    assert api_fleet_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=1))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=1))])))
    assert api_fit_stats.dps.one() == [approx(1053.24), 0, approx(992.98), 0]
    assert api_fit_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=1))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=1))])))
    assert api_module_stats.dps.one() == [approx(1053.24), 0, approx(992.98), 0]
    assert api_module_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    # Verification - just before second hit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=5))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=5))])))
    assert api_fleet_stats.dps.one() == [approx(210.648), 0, approx(198.596), 0]
    assert api_fleet_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=5))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=5))])))
    assert api_fit_stats.dps.one() == [approx(210.648), 0, approx(198.596), 0]
    assert api_fit_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=5))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=5))])))
    assert api_module_stats.dps.one() == [approx(210.648), 0, approx(198.596), 0]
    assert api_module_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    # Verification - just after second hit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=7))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=7))])))
    assert api_fleet_stats.dps.one() == [approx(300.925714), 0, approx(283.708571), 0]
    assert api_fleet_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=7))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=7))])))
    assert api_fit_stats.dps.one() == [approx(300.925714), 0, approx(283.708571), 0]
    assert api_fit_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=7))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=7))])))
    assert api_module_stats.dps.one() == [approx(300.925714), 0, approx(283.708571), 0]
    assert api_module_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    # Verification - just before reload starts
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=479))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=479))])))
    assert api_fleet_stats.dps.one() == [approx(175.906472), 0, approx(165.842171), 0]
    assert api_fleet_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=479))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=479))])))
    assert api_fit_stats.dps.one() == [approx(175.906472), 0, approx(165.842171), 0]
    assert api_fit_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=479))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=479))])))
    assert api_module_stats.dps.one() == [approx(175.906472), 0, approx(165.842171), 0]
    assert api_module_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    # Verification - just before reload completes
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=484))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=484))])))
    assert api_fleet_stats.dps.one() == [approx(174.089256), 0, approx(164.128926), 0]
    assert api_fleet_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=484))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=484))])))
    assert api_fit_stats.dps.one() == [approx(174.089256), 0, approx(164.128926), 0]
    assert api_fit_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=484))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=484))])))
    assert api_module_stats.dps.one() == [approx(174.089256), 0, approx(164.128926), 0]
    assert api_module_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    # Verification - after reload is done and another vorton hit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=486))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=486))])))
    assert api_fleet_stats.dps.one() == [approx(175.54), 0, approx(165.496667), 0]
    assert api_fleet_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=486))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=486))])))
    assert api_fit_stats.dps.one() == [approx(175.54), 0, approx(165.496667), 0]
    assert api_fit_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=486))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=486))])))
    assert api_module_stats.dps.one() == [approx(175.54), 0, approx(165.496667), 0]
    assert api_module_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]


def test_charge_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_vorton(
        client=client, basic_info=eve_basic_info, dmg_mult=2.62, capacity=1, cycle_time=6000, reload_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
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
    assert api_module_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_stats.volley.one() == [0, 0, 0, 0]


def test_charge_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_vorton(
        client=client, basic_info=eve_basic_info, dmg_mult=2.62, capacity=1, cycle_time=6000, reload_time=5000)
    eve_charge_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
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
    assert api_module_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_stats.volley.one() == [0, 0, 0, 0]
