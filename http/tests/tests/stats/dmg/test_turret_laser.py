from tests import approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionFitDps, StatsOptionItemDps, StatsOptionItemVolley
from tests.tests.stats.dmg import make_eve_turret_charge_crystal, make_eve_turret_laser, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_laser(
        client=client, basic_info=eve_basic_info, dmg_mult=15, capacity=1, cycle_time=2050, reload_time=0.01)
    eve_charge_id = make_eve_turret_charge_crystal(
        client=client, basic_info=eve_basic_info, dmgs=(9, 2, 0, 0), volume=1,
        get_damaged=1, hp=1, vol_dmg=0.01, vol_chance=0.1)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_fit_stats.volley.one() == [approx(135), approx(30), 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_module_stats.volley.one() == [approx(135), approx(30), 0, 0]
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
    assert api_module_dps_ignored == [approx(65.853659), approx(14.634146), 0, 0]
    api_module_volley_normal, api_module_volley_ignored = api_module_stats.volley
    assert api_module_volley_normal == [0, 0, 0, 0]
    assert api_module_volley_ignored == [approx(135), approx(30), 0, 0]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_fit_stats.volley.one() == [approx(135), approx(30), 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_module_stats.volley.one() == [approx(135), approx(30), 0, 0]


def test_reload(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_laser(
        client=client, basic_info=eve_basic_info, dmg_mult=15, capacity=1, cycle_time=2050, reload_time=0.01)
    eve_charge1_id = make_eve_turret_charge_crystal(
        client=client, basic_info=eve_basic_info, dmgs=(9, 2, 0, 0), volume=1,
        get_damaged=1, hp=1, vol_dmg=0.01, vol_chance=0.1)
    eve_charge2_id = make_eve_turret_charge_crystal(
        client=client, basic_info=eve_basic_info, dmgs=(9, 2, 0, 0), volume=1,
        get_damaged=0, hp=1, vol_dmg=0.01, vol_chance=0.1)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge1_id)
    api_module2 = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge2_id)
    # Verification - impact of reload is significant despite reload time being super low due to
    # library enforcing 1 second reload time
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(), StatsOptionFitDps(reload=True)])))
    api_fit_dps_burst, api_fit_dps_reload = api_fit_stats.dps
    assert api_fit_dps_burst == [approx(131.707317), approx(29.268293), 0, 0]
    assert api_fit_dps_reload == [approx(131.675209), approx(29.261158), 0, 0]
    api_module1_stats = api_module1.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(), StatsOptionItemDps(reload=True)])))
    api_module1_dps_burst, api_module1_dps_reload = api_module1_stats.dps
    assert api_module1_dps_burst == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_module1_dps_reload == [approx(65.82155), approx(14.627011), 0, 0]
    api_module2_stats = api_module2.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(), StatsOptionItemDps(reload=True)])))
    api_module2_dps_burst, api_module2_dps_reload = api_module2_stats.dps
    assert api_module2_dps_burst == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_module2_dps_reload == [approx(65.853659), approx(14.634146), 0, 0]


def test_charge_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_laser(
        client=client, basic_info=eve_basic_info, dmg_mult=15, capacity=1, cycle_time=2050, reload_time=0.01)
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
    eve_module_id = make_eve_turret_laser(
        client=client, basic_info=eve_basic_info, dmg_mult=15, capacity=1, cycle_time=2050, reload_time=0.01)
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
