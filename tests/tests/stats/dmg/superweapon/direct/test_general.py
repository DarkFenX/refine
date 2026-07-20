from fw import approx
from fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatDmgItemKinds,
    StatsOptionFitDmg,
    StatsOptionItemDmg,
    StatTimeBurst,
    StatTimeSim,
)
from tests.stats.dmg import (
    make_eve_dd_direct_amarr,
    make_eve_dd_direct_caldari,
    make_eve_dd_direct_gallente,
    make_eve_dd_direct_minmatar,
    setup_dmg_basics,
)


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_direct_amarr(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(4950000, 0, 0, 0),
        cycle_time=240000,
        delay=9000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_module_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_module_dmg_normal, api_module_dmg_ignored = api_module_stats.dmg
    assert api_module_dmg_normal.dps == [0, 0, 0, 0]
    assert api_module_dmg_normal.volley == [0, 0, 0, 0]
    assert api_module_dmg_ignored.dps == [approx(20625), 0, 0, 0]
    assert api_module_dmg_ignored.volley == [approx(4950000), 0, 0, 0]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_module_dmg_stats.volley == [approx(4950000), 0, 0, 0]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_direct_amarr(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(4950000, 0, 0, 0),
        cycle_time=240000,
        delay=9000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit2 = api_sol.create_fit()
    api_fit2.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit1.id, api_fit2.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(61875), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(14850000), 0, 0, 0]
    api_fit1_dmg_stats = api_fit1.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit1_dmg_stats.dps == [approx(41250), 0, 0, 0]
    assert api_fit1_dmg_stats.volley == [approx(9900000), 0, 0, 0]
    api_fit2_dmg_stats = api_fit2.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit2_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_fit2_dmg_stats.volley == [approx(4950000), 0, 0, 0]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_direct_amarr(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(4950000, 0, 0, 0),
        cycle_time=240000,
        delay=9000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, superweapon=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, superweapon=True))])))
    api_fleet_dmg_default, api_fleet_dmg_disabled, api_fleet_dmg_enabled = api_fleet_stats.dmg
    assert api_fleet_dmg_default.dps == [approx(20625), 0, 0, 0]
    assert api_fleet_dmg_default.volley == [approx(4950000), 0, 0, 0]
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fleet_dmg_enabled.dps == [approx(20625), 0, 0, 0]
    assert api_fleet_dmg_enabled.volley == [approx(4950000), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, superweapon=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, superweapon=True))])))
    api_fit_dmg_default, api_fit_dmg_disabled, api_fit_dmg_enabled = api_fit_stats.dmg
    assert api_fit_dmg_default.dps == [approx(20625), 0, 0, 0]
    assert api_fit_dmg_default.volley == [approx(4950000), 0, 0, 0]
    assert api_fit_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fit_dmg_enabled.dps == [approx(20625), 0, 0, 0]
    assert api_fit_dmg_enabled.volley == [approx(4950000), 0, 0, 0]


def test_time(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_direct_amarr(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(4950000, 0, 0, 0),
        cycle_time=240000,
        delay=9000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification - burst stats
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_module_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    # Verification - sim without time
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_module_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    # Verification - sim with time before damage hits
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=8.9))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=8.9))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=8.9))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_dmg_stats.volley == [0, 0, 0, 0]
    # Verification - sim with time after first hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=9.1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(543956.043956), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=9.1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(543956.043956), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=9.1))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(543956.043956), 0, 0, 0]
    assert api_module_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    # Verification - sim with time before damage of second cycle
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=248.9))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(19887.505022), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=248.9))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(19887.505022), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=248.9))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(19887.505022), 0, 0, 0]
    assert api_module_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    # Verification - sim with time after damage of second cycle
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=249.1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(39743.07507), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=249.1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(39743.07507), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=249.1))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(39743.07507), 0, 0, 0]
    assert api_module_dmg_stats.volley == [approx(4950000), 0, 0, 0]


def test_other_factions(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_caldari_id = make_eve_dd_direct_caldari(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(0, 0, 4950000, 0),
        cycle_time=240000,
        delay=9000)
    eve_module_gallente_id = make_eve_dd_direct_gallente(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(0, 4950000, 0, 0),
        cycle_time=240000,
        delay=9000)
    eve_module_minmatar_id = make_eve_dd_direct_minmatar(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(0, 0, 0, 4950000),
        cycle_time=240000,
        delay=9000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_caldari_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, superweapon=True))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, approx(20625), 0]
    assert api_fleet_dmg_stats.volley == [0, 0, approx(4950000), 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, superweapon=True))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, approx(20625), 0]
    assert api_fit_dmg_stats.volley == [0, 0, approx(4950000), 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, approx(20625), 0]
    assert api_module_dmg_stats.volley == [0, 0, approx(4950000), 0]
    # Action
    api_module.change_module(type_id=eve_module_gallente_id)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, superweapon=True))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(20625), 0, 0]
    assert api_fleet_dmg_stats.volley == [0, approx(4950000), 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, superweapon=True))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(20625), 0, 0]
    assert api_fit_dmg_stats.volley == [0, approx(4950000), 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(20625), 0, 0]
    assert api_module_dmg_stats.volley == [0, approx(4950000), 0, 0]
    # Action
    api_module.change_module(type_id=eve_module_minmatar_id)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, superweapon=True))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(20625)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(4950000)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, superweapon=True))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, approx(20625)]
    assert api_fit_dmg_stats.volley == [0, 0, 0, approx(4950000)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, 0, approx(20625)]
    assert api_module_dmg_stats.volley == [0, 0, 0, approx(4950000)]
