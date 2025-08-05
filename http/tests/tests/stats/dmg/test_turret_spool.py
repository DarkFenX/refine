from tests import Spool, approx
from tests.fw.api import (
    FitStatsOptions,
    ItemStatsOptions,
    StatsOptionFitDps,
    StatsOptionFitVolley,
    StatsOptionItemDps,
    StatsOptionItemVolley,
)
from tests.tests.stats.dmg import make_eve_turret_charge_normal, make_eve_turret_spool, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_spool(
        client=client, basic_info=eve_basic_info,
        dmg_mult=4.28, spool_step=0.07, spool_max=2.125,
        capacity=5, cycle_time=2180, reload_time=0.01)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 63, 0, 33), volume=0.01)
    client.create_sources()
    api_sol = client.create_sol(default_spool=Spool.spool_scale_to_api(val=1))
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_fit_stats.volley.one() == [0, approx(842.625), 0, approx(441.375)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_module_stats.volley.one() == [0, approx(842.625), 0, approx(441.375)]
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
    assert api_module_dps_ignored == [0, approx(386.525229), 0, approx(202.465596)]
    api_module_volley_normal, api_module_volley_ignored = api_module_stats.volley
    assert api_module_volley_normal == [0, 0, 0, 0]
    assert api_module_volley_ignored == [0, approx(842.625), 0, approx(441.375)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_fit_stats.volley.one() == [0, approx(842.625), 0, approx(441.375)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_module_stats.volley.one() == [0, approx(842.625), 0, approx(441.375)]


def test_spool(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_spool(
        client=client, basic_info=eve_basic_info,
        dmg_mult=4.28, spool_step=0.07, spool_max=2.125,
        capacity=5, cycle_time=2180, reload_time=0.01)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 63, 0, 33), volume=0.01)
    client.create_sources()
    api_sol = client.create_sol(default_spool=Spool.spool_scale_to_api(val=0.5))
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [
            StatsOptionFitDps(spool=Spool.spool_scale_to_api(val=0)),
            StatsOptionFitDps(),
            StatsOptionFitDps(spool=Spool.spool_scale_to_api(val=1))]),
        volley=(True, [
            StatsOptionFitVolley(spool=Spool.spool_scale_to_api(val=0)),
            StatsOptionFitVolley(),
            StatsOptionFitVolley(spool=Spool.spool_scale_to_api(val=1))])))
    api_fit_dps_pre, api_fit_dps_default, api_fit_dps_full = api_fit_stats.dps
    assert api_fit_dps_pre == [0, approx(123.688073), 0, approx(64.788991)]
    assert api_fit_dps_default == [0, approx(262.218716), 0, approx(137.352661)]
    assert api_fit_dps_full == [0, approx(386.525229), 0, approx(202.465596)]
    api_fit_volley_pre, api_fit_volley_default, api_fit_volley_full = api_fit_stats.volley
    assert api_fit_volley_pre == [0, approx(269.64), 0, approx(141.24)]
    assert api_fit_volley_default == [0, approx(571.6368), 0, approx(299.4288)]
    assert api_fit_volley_full == [0, approx(842.625), 0, approx(441.375)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [
            StatsOptionItemDps(spool=Spool.spool_scale_to_api(val=0)),
            StatsOptionItemDps(),
            StatsOptionItemDps(spool=Spool.spool_scale_to_api(val=1))]),
        volley=(True, [
            StatsOptionItemVolley(spool=Spool.spool_scale_to_api(val=0)),
            StatsOptionItemVolley(),
            StatsOptionItemVolley(spool=Spool.spool_scale_to_api(val=1))])))
    api_module_dps_pre, api_module_dps_default, api_module_dps_full = api_module_stats.dps
    assert api_module_dps_pre == [0, approx(123.688073), 0, approx(64.788991)]
    assert api_module_dps_default == [0, approx(262.218716), 0, approx(137.352661)]
    assert api_module_dps_full == [0, approx(386.525229), 0, approx(202.465596)]
    api_module_volley_pre, api_module_volley_default, api_module_volley_full = api_module_stats.volley
    assert api_module_volley_pre == [0, approx(269.64), 0, approx(141.24)]
    assert api_module_volley_default == [0, approx(571.6368), 0, approx(299.4288)]
    assert api_module_volley_full == [0, approx(842.625), 0, approx(441.375)]
    # Action
    api_module.change_module(spool=Spool.spool_scale_to_api(val=0.7))
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [
            StatsOptionFitDps(spool=Spool.spool_scale_to_api(val=0)),
            StatsOptionFitDps(),
            StatsOptionFitDps(spool=Spool.spool_scale_to_api(val=1))]),
        volley=(True, [
            StatsOptionFitVolley(spool=Spool.spool_scale_to_api(val=0)),
            StatsOptionFitVolley(),
            StatsOptionFitVolley(spool=Spool.spool_scale_to_api(val=1))])))
    api_fit_dps_pre, api_fit_dps_default, api_fit_dps_full = api_fit_stats.dps
    assert api_fit_dps_pre == [0, approx(123.688073), 0, approx(64.788991)]
    assert api_fit_dps_default == [0, approx(314.167706), 0, approx(164.564037)]
    assert api_fit_dps_full == [0, approx(386.525229), 0, approx(202.465596)]
    api_fit_volley_pre, api_fit_volley_default, api_fit_volley_full = api_fit_stats.volley
    assert api_fit_volley_pre == [0, approx(269.64), 0, approx(141.24)]
    assert api_fit_volley_default == [0, approx(684.8856), 0, approx(358.7496)]
    assert api_fit_volley_full == [0, approx(842.625), 0, approx(441.375)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [
            StatsOptionItemDps(spool=Spool.spool_scale_to_api(val=0)),
            StatsOptionItemDps(),
            StatsOptionItemDps(spool=Spool.spool_scale_to_api(val=1))]),
        volley=(True, [
            StatsOptionItemVolley(spool=Spool.spool_scale_to_api(val=0)),
            StatsOptionItemVolley(),
            StatsOptionItemVolley(spool=Spool.spool_scale_to_api(val=1))])))
    api_module_dps_pre, api_module_dps_default, api_module_dps_full = api_module_stats.dps
    assert api_module_dps_pre == [0, approx(123.688073), 0, approx(64.788991)]
    assert api_module_dps_default == [0, approx(314.167706), 0, approx(164.564037)]
    assert api_module_dps_full == [0, approx(386.525229), 0, approx(202.465596)]
    api_module_volley_pre, api_module_volley_default, api_module_volley_full = api_module_stats.volley
    assert api_module_volley_pre == [0, approx(269.64), 0, approx(141.24)]
    assert api_module_volley_default == [0, approx(684.8856), 0, approx(358.7496)]
    assert api_module_volley_full == [0, approx(842.625), 0, approx(441.375)]


def test_reload(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_spool(
        client=client, basic_info=eve_basic_info,
        dmg_mult=4.28, spool_step=0.07, spool_max=2.125,
        capacity=5, cycle_time=2180, reload_time=0.01)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 63, 0, 33), volume=0.01)
    client.create_sources()
    api_sol = client.create_sol(default_spool=Spool.spool_scale_to_api(val=1))
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(), StatsOptionFitDps(reload=True)])))
    api_fit_dps_burst, api_fit_dps_reload = api_fit_stats.dps
    assert api_fit_dps_burst == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_fit_dps_reload == [0, approx(386.170944), 0, approx(202.280018)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(), StatsOptionItemDps(reload=True)])))
    api_module_dps_burst, api_module_dps_reload = api_module_stats.dps
    assert api_module_dps_burst == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_module_dps_reload == [0, approx(386.170944), 0, approx(202.280018)]


def test_charge_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_spool(
        client=client, basic_info=eve_basic_info,
        dmg_mult=4.28, spool_step=0.07, spool_max=2.125,
        capacity=5, cycle_time=2180, reload_time=0.01)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_fit_stats.volley.one() == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_stats.volley.one() == [0, 0, 0, 0]


def test_charge_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_spool(
        client=client, basic_info=eve_basic_info,
        dmg_mult=4.28, spool_step=0.07, spool_max=2.125,
        capacity=5, cycle_time=2180, reload_time=0.01)
    eve_charge_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_fit_stats.volley.one() == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_stats.volley.one() == [0, 0, 0, 0]
