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
from tests.stats.dmg import make_eve_bomb_guided, make_eve_launcher, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=400, cycle_time=33600, reload_time=60000)
    eve_charge_id = make_eve_bomb_guided(
        client=client, basic_info=eve_basic_info, dmgs=(2640, 2640, 2640, 2640), volume=100)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification - module is active, charge is not force-disabled, so stats are exposed without
    # extra flags
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_fleet_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_fit_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=True)]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_module_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_charge_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification - module not active, charge is not force-disabled - stats are exposed only when
    # state is ignored
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(include_charges=True),
        StatsOptionItemDmg(include_charges=True, ignore_state=True)])))
    api_module_dmg_normal, api_module_dmg_ignored = api_module_stats.dmg
    assert api_module_dmg_normal.dps == [0, 0, 0, 0]
    assert api_module_dmg_normal.volley == [0, 0, 0, 0]
    assert api_module_dmg_ignored.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_module_dmg_ignored.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_charge_dmg_normal, api_charge_dmg_ignored = api_charge_stats.dmg
    assert api_charge_dmg_normal.dps == [0, 0, 0, 0]
    assert api_charge_dmg_normal.volley == [0, 0, 0, 0]
    assert api_charge_dmg_ignored.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_charge_dmg_ignored.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    # Action
    api_module.charge.change_charge(state=False)
    # Verification - module not active, charge is force-disabled - stats are exposed only when state
    # is ignored
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(include_charges=True),
        StatsOptionItemDmg(include_charges=True, ignore_state=True)])))
    api_module_dmg_normal, api_module_dmg_ignored = api_module_stats.dmg
    assert api_module_dmg_normal.dps == [0, 0, 0, 0]
    assert api_module_dmg_normal.volley == [0, 0, 0, 0]
    assert api_module_dmg_ignored.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_module_dmg_ignored.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_charge_dmg_normal, api_charge_dmg_ignored = api_charge_stats.dmg
    assert api_charge_dmg_normal.dps == [0, 0, 0, 0]
    assert api_charge_dmg_normal.volley == [0, 0, 0, 0]
    assert api_charge_dmg_ignored.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_charge_dmg_ignored.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification - module active, but charge is force-disabled - stats are exposed only when state
    # is ignored
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(include_charges=True),
        StatsOptionItemDmg(include_charges=True, ignore_state=True)])))
    api_module_dmg_normal, api_module_dmg_ignored = api_module_stats.dmg
    assert api_module_dmg_normal.dps == [0, 0, 0, 0]
    assert api_module_dmg_normal.volley == [0, 0, 0, 0]
    assert api_module_dmg_ignored.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_module_dmg_ignored.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_charge_dmg_normal, api_charge_dmg_ignored = api_charge_stats.dmg
    assert api_charge_dmg_normal.dps == [0, 0, 0, 0]
    assert api_charge_dmg_normal.volley == [0, 0, 0, 0]
    assert api_charge_dmg_ignored.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_charge_dmg_ignored.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    # Action
    api_module.charge.change_charge(state=True)
    # Verification - module is active, charge is not force-disabled, so stats are exposed without
    # extra flags
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_fleet_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_fit_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=True)]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_module_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_charge_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=400, cycle_time=33600, reload_time=60000)
    eve_charge_id = make_eve_bomb_guided(
        client=client, basic_info=eve_basic_info, dmgs=(2640, 2640, 2640, 2640), volume=100)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fit2 = api_sol.create_fit()
    api_fit2.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit1.id, api_fit2.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(235.714286), approx(235.714286), approx(235.714286), approx(235.714286)]
    assert api_fleet_dmg_stats.volley == [approx(7920), approx(7920), approx(7920), approx(7920)]
    api_fit1_dmg_stats = api_fit1.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit1_dmg_stats.dps == [approx(157.142857), approx(157.142857), approx(157.142857), approx(157.142857)]
    assert api_fit1_dmg_stats.volley == [approx(5280), approx(5280), approx(5280), approx(5280)]
    api_fit2_dmg_stats = api_fit2.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit2_dmg_stats.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_fit2_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=400, cycle_time=33600, reload_time=60000)
    eve_charge_id = make_eve_bomb_guided(
        client=client, basic_info=eve_basic_info, dmgs=(2640, 2640, 2640, 2640), volume=100)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, bomb=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, bomb=True))])))
    api_fleet_dmg_default, api_fleet_dmg_disabled, api_fleet_dmg_enabled = api_fleet_stats.dmg
    assert api_fleet_dmg_default.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_fleet_dmg_default.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fleet_dmg_enabled.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_fleet_dmg_enabled.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, bomb=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, bomb=True))])))
    api_fit_dmg_default, api_fit_dmg_disabled, api_fit_dmg_enabled = api_fit_stats.dmg
    assert api_fit_dmg_default.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_fit_dmg_default.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    assert api_fit_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fit_dmg_enabled.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_fit_dmg_enabled.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]


def test_include_charges(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=400, cycle_time=33600, reload_time=60000)
    eve_charge_id = make_eve_bomb_guided(
        client=client, basic_info=eve_basic_info, dmgs=(2640, 2640, 2640, 2640), volume=100)
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
        dmg=(True, [StatsOptionItemDmg(include_charges=False), StatsOptionItemDmg(include_charges=True)])))
    api_module_dmg_without, api_module_dmg_with = api_module_stats.dmg
    assert api_module_dmg_without.dps == [0, 0, 0, 0]
    assert api_module_dmg_with.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_module_dmg_without.volley == [0, 0, 0, 0]
    assert api_module_dmg_with.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=False), StatsOptionItemDmg(include_charges=True)])))
    api_charge_dmg_without, api_charge_dmg_with = api_charge_stats.dmg
    assert api_charge_dmg_without.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_charge_dmg_without.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    assert api_charge_dmg_with.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_charge_dmg_with.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]


def test_time(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=400, cycle_time=33600, reload_time=60000)
    eve_charge_id = make_eve_bomb_guided(
        client=client, basic_info=eve_basic_info, dmgs=(2640, 2640, 2640, 2640), volume=100)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification - burst stats (reload is ignored)
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_fleet_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_fit_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_charge_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    # Verification - sim without time means stats with reload time are considered
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(54.320988), approx(54.320988), approx(54.320988), approx(54.320988)]
    assert api_fleet_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(54.320988), approx(54.320988), approx(54.320988), approx(54.320988)]
    assert api_fit_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(54.320988), approx(54.320988), approx(54.320988), approx(54.320988)]
    assert api_charge_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    # Verification - just after first hit landed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(2640), approx(2640), approx(2640), approx(2640)]
    assert api_fleet_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(2640), approx(2640), approx(2640), approx(2640)]
    assert api_fit_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(2640), approx(2640), approx(2640), approx(2640)]
    assert api_charge_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    # Verification - just before second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=33))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(80), approx(80), approx(80), approx(80)]
    assert api_fleet_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=33))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(80), approx(80), approx(80), approx(80)]
    assert api_fit_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=33))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(80), approx(80), approx(80), approx(80)]
    assert api_charge_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    # Verification - just after second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=34))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(155.294118), approx(155.294118), approx(155.294118), approx(155.294118)]
    assert api_fleet_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=34))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(155.294118), approx(155.294118), approx(155.294118), approx(155.294118)]
    assert api_fit_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=34))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [
        approx(155.294118), approx(155.294118), approx(155.294118), approx(155.294118)]
    assert api_charge_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    # Verification - just before reload starts
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=134))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(78.80597), approx(78.80597), approx(78.80597), approx(78.80597)]
    assert api_fleet_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=134))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(78.80597), approx(78.80597), approx(78.80597), approx(78.80597)]
    assert api_fit_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=134))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(78.80597), approx(78.80597), approx(78.80597), approx(78.80597)]
    assert api_charge_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    # Verification - just before reload completes
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=194))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(54.43299), approx(54.43299), approx(54.43299), approx(54.43299)]
    assert api_fleet_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=194))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(54.43299), approx(54.43299), approx(54.43299), approx(54.43299)]
    assert api_fit_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=194))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(54.43299), approx(54.43299), approx(54.43299), approx(54.43299)]
    assert api_charge_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    # Verification - after reload is done and another bomb hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=195))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(67.692308), approx(67.692308), approx(67.692308), approx(67.692308)]
    assert api_fleet_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=195))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(67.692308), approx(67.692308), approx(67.692308), approx(67.692308)]
    assert api_fit_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=195))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(67.692308), approx(67.692308), approx(67.692308), approx(67.692308)]
    assert api_charge_dmg_stats.volley == [approx(2640), approx(2640), approx(2640), approx(2640)]
