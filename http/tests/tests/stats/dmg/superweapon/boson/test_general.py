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
from tests.stats.dmg import make_eve_dd_boson, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_boson(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(20625, 20625, 20625, 20625),
        cycle_time=240000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=20000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [approx(1718.75), approx(1718.75), approx(1718.75), approx(1718.75)]
    assert api_fleet_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [approx(1718.75), approx(1718.75), approx(1718.75), approx(1718.75)]
    assert api_fit_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [approx(1718.75), approx(1718.75), approx(1718.75), approx(1718.75)]
    assert api_module_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_fit_stats.volley.one() == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(), StatsOptionItemDps(ignore_state=True)]),
        volley=(True, [StatsOptionItemVolley(), StatsOptionItemVolley(ignore_state=True)])))
    api_module_dps_normal, api_module_dps_ignored = api_module_stats.dps
    assert api_module_dps_normal == [0, 0, 0, 0]
    assert api_module_dps_ignored == [approx(1718.75), approx(1718.75), approx(1718.75), approx(1718.75)]
    api_module_volley_normal, api_module_volley_ignored = api_module_stats.volley
    assert api_module_volley_normal == [0, 0, 0, 0]
    assert api_module_volley_ignored == [approx(20625), approx(20625), approx(20625), approx(20625)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [approx(1718.75), approx(1718.75), approx(1718.75), approx(1718.75)]
    assert api_fit_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [approx(1718.75), approx(1718.75), approx(1718.75), approx(1718.75)]
    assert api_module_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_boson(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(20625, 20625, 20625, 20625),
        cycle_time=240000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=20000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit2 = api_sol.create_fit()
    api_fit2.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [approx(5156.25), approx(5156.25), approx(5156.25), approx(5156.25)]
    assert api_fleet_stats.volley.one() == [approx(61875), approx(61875), approx(61875), approx(61875)]
    api_fit1_stats = api_fit1.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit1_stats.dps.one() == [approx(3437.5), approx(3437.5), approx(3437.5), approx(3437.5)]
    assert api_fit1_stats.volley.one() == [approx(41250), approx(41250), approx(41250), approx(41250)]
    api_fit2_stats = api_fit2.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit2_stats.dps.one() == [approx(1718.75), approx(1718.75), approx(1718.75), approx(1718.75)]
    assert api_fit2_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_boson(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(20625, 20625, 20625, 20625),
        cycle_time=240000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=20000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [
            StatsOptionFitDps(),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=True, superweapon=False)),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=False, superweapon=True))]),
        volley=(True, [
            StatsOptionFitVolley(),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=True, superweapon=False)),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=False, superweapon=True))])))
    api_fleet_dps_default, api_fleet_dps_disabled, api_fleet_dps_enabled = api_fleet_stats.dps
    assert api_fleet_dps_default == [approx(1718.75), approx(1718.75), approx(1718.75), approx(1718.75)]
    assert api_fleet_dps_disabled == [0, 0, 0, 0]
    assert api_fleet_dps_enabled == [approx(1718.75), approx(1718.75), approx(1718.75), approx(1718.75)]
    api_fleet_volley_default, api_fleet_volley_disabled, api_fleet_volley_enabled = api_fleet_stats.volley
    assert api_fleet_volley_default == [approx(20625), approx(20625), approx(20625), approx(20625)]
    assert api_fleet_volley_disabled == [0, 0, 0, 0]
    assert api_fleet_volley_enabled == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [
            StatsOptionFitDps(),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=True, superweapon=False)),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=False, superweapon=True))]),
        volley=(True, [
            StatsOptionFitVolley(),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=True, superweapon=False)),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=False, superweapon=True))])))
    api_fit_dps_default, api_fit_dps_disabled, api_fit_dps_enabled = api_fit_stats.dps
    assert api_fit_dps_default == [approx(1718.75), approx(1718.75), approx(1718.75), approx(1718.75)]
    assert api_fit_dps_disabled == [0, 0, 0, 0]
    assert api_fit_dps_enabled == [approx(1718.75), approx(1718.75), approx(1718.75), approx(1718.75)]
    api_fit_volley_default, api_fit_volley_disabled, api_fit_volley_enabled = api_fit_stats.volley
    assert api_fit_volley_default == [approx(20625), approx(20625), approx(20625), approx(20625)]
    assert api_fit_volley_disabled == [0, 0, 0, 0]
    assert api_fit_volley_enabled == [approx(20625), approx(20625), approx(20625), approx(20625)]


def test_time(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_boson(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(20625, 20625, 20625, 20625),
        cycle_time=240000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=20000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification - burst stats
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeBurst())]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeBurst())])))
    assert api_fleet_stats.dps.one() == [approx(1718.75), approx(1718.75), approx(1718.75), approx(1718.75)]
    assert api_fleet_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeBurst())]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeBurst())])))
    assert api_fit_stats.dps.one() == [approx(1718.75), approx(1718.75), approx(1718.75), approx(1718.75)]
    assert api_fit_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeBurst())]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeBurst())])))
    assert api_module_stats.dps.one() == [approx(1718.75), approx(1718.75), approx(1718.75), approx(1718.75)]
    assert api_module_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    # Verification - sim without time
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=None))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=None))])))
    assert api_fleet_stats.dps.one() == [approx(1718.75), approx(1718.75), approx(1718.75), approx(1718.75)]
    assert api_fleet_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=None))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=None))])))
    assert api_fit_stats.dps.one() == [approx(1718.75), approx(1718.75), approx(1718.75), approx(1718.75)]
    assert api_fit_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=None))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=None))])))
    assert api_module_stats.dps.one() == [approx(1718.75), approx(1718.75), approx(1718.75), approx(1718.75)]
    assert api_module_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    # Verification - sim with time before first hit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=14.9))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=14.9))])))
    assert api_fleet_stats.dps.one() == [0, 0, 0, 0]
    assert api_fleet_stats.volley.one() == [0, 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=14.9))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=14.9))])))
    assert api_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_fit_stats.volley.one() == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=14.9))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=14.9))])))
    assert api_module_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_stats.volley.one() == [0, 0, 0, 0]
    # Verification - sim with time after first hit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=15.1))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=15.1))])))
    assert api_fleet_stats.dps.one() == [approx(1365.89404), approx(1365.89404), approx(1365.89404), approx(1365.89404)]
    assert api_fleet_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=15.1))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=15.1))])))
    assert api_fit_stats.dps.one() == [approx(1365.89404), approx(1365.89404), approx(1365.89404), approx(1365.89404)]
    assert api_fit_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=15.1))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=15.1))])))
    assert api_module_stats.dps.one() == [
        approx(1365.89404), approx(1365.89404), approx(1365.89404), approx(1365.89404)]
    assert api_module_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    # Verification - sim with time before second hit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=15.9))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=15.9))])))
    assert api_fleet_stats.dps.one() == [
        approx(1297.169811), approx(1297.169811), approx(1297.169811), approx(1297.169811)]
    assert api_fleet_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=15.9))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=15.9))])))
    assert api_fit_stats.dps.one() == [
        approx(1297.169811), approx(1297.169811), approx(1297.169811), approx(1297.169811)]
    assert api_fit_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=15.9))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=15.9))])))
    assert api_module_stats.dps.one() == [
        approx(1297.169811), approx(1297.169811), approx(1297.169811), approx(1297.169811)]
    assert api_module_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    # Verification - sim with time after second hit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=16.1))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=16.1))])))
    assert api_fleet_stats.dps.one() == [
        approx(2562.111801), approx(2562.111801), approx(2562.111801), approx(2562.111801)]
    assert api_fleet_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=16.1))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=16.1))])))
    assert api_fit_stats.dps.one() == [
        approx(2562.111801), approx(2562.111801), approx(2562.111801), approx(2562.111801)]
    assert api_fit_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=16.1))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=16.1))])))
    assert api_module_stats.dps.one() == [
        approx(2562.111801), approx(2562.111801), approx(2562.111801), approx(2562.111801)]
    assert api_module_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    # Verification - sim with time after last hit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=34.1))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=34.1))])))
    assert api_fleet_stats.dps.one() == [
        approx(12096.774194), approx(12096.774194), approx(12096.774194), approx(12096.774194)]
    assert api_fleet_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=34.1))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=34.1))])))
    assert api_fit_stats.dps.one() == [
        approx(12096.774194), approx(12096.774194), approx(12096.774194), approx(12096.774194)]
    assert api_fit_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=34.1))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=34.1))])))
    assert api_module_stats.dps.one() == [
        approx(12096.774194), approx(12096.774194), approx(12096.774194), approx(12096.774194)]
    assert api_module_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    # Verification - sim with time after 2nd cycle starts, just before first damage tick
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=254.9))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=254.9))])))
    assert api_fleet_stats.dps.one() == [
        approx(1618.281679), approx(1618.281679), approx(1618.281679), approx(1618.281679)]
    assert api_fleet_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=254.9))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=254.9))])))
    assert api_fit_stats.dps.one() == [
        approx(1618.281679), approx(1618.281679), approx(1618.281679), approx(1618.281679)]
    assert api_fit_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=254.9))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=254.9))])))
    assert api_module_stats.dps.one() == [
        approx(1618.281679), approx(1618.281679), approx(1618.281679), approx(1618.281679)]
    assert api_module_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    # Verification - sim with time after first damage tick of 2nd cycle
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=255.1))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=255.1))])))
    assert api_fleet_stats.dps.one() == [
        approx(1697.863583), approx(1697.863583), approx(1697.863583), approx(1697.863583)]
    assert api_fleet_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=255.1))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=255.1))])))
    assert api_fit_stats.dps.one() == [
        approx(1697.863583), approx(1697.863583), approx(1697.863583), approx(1697.863583)]
    assert api_fit_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=255.1))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=255.1))])))
    assert api_module_stats.dps.one() == [
        approx(1697.863583), approx(1697.863583), approx(1697.863583), approx(1697.863583)]
    assert api_module_stats.volley.one() == [approx(20625), approx(20625), approx(20625), approx(20625)]
