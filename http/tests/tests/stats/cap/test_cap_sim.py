from tests import approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionCapSim


def test_stability_high_fluctuation(client, consts):
    # In pyfa, stability is defined by combination of two different low cap watermarks, which gives
    # super low value for some ships; the library uses different method to calculate stability
    # value, which is tested here
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_boost_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_bonus)
    eve_use_amount_attr_id = client.mk_eve_attr()
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_use_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_amount_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_user_id = client.mk_eve_item(
        attrs={eve_use_amount_attr_id: 45, eve_cycle_time_attr_id: 2448},
        eff_ids=[eve_use_effect_id],
        defeff_id=eve_use_effect_id)
    eve_inject_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.power_booster,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_injector_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 15, eve_cycle_time_attr_id: 12000, eve_reload_attr_id: 10000},
        eff_ids=[eve_inject_effect_id],
        defeff_id=eve_inject_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_boost_amount_attr_id: 400, eve_volume_attr_id: 12})
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 346.875, eve_regen_attr_id: 138750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_user_id, state=consts.ApiModuleState.active)
    api_fit.add_module(type_id=eve_injector_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=True))
    assert api_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.5002785)}
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=True))
    assert api_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.5002785)}


def test_stability_no_events(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 225, eve_regen_attr_id: 90000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_options = [StatsOptionCapSim(cap_perc=0), StatsOptionCapSim(cap_perc=0.3), StatsOptionCapSim(cap_perc=1)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]


def test_stability_only_injects(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_boost_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_bonus)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_inject_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.power_booster,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_injector_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 15, eve_cycle_time_attr_id: 12000, eve_reload_attr_id: 10000},
        eff_ids=[eve_inject_effect_id],
        defeff_id=eve_inject_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_boost_amount_attr_id: 400, eve_volume_attr_id: 12})
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 225, eve_regen_attr_id: 90000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_injector_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    # Verification
    api_options = [StatsOptionCapSim(cap_perc=0), StatsOptionCapSim(cap_perc=0.3), StatsOptionCapSim(cap_perc=1)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]


def test_stability_only_transfers(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 351, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_options = [StatsOptionCapSim(cap_perc=0), StatsOptionCapSim(cap_perc=0.3), StatsOptionCapSim(cap_perc=1)]
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]


def test_stagger_neuts(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 120, eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={
        eve_ship_amount_attr_id: 1812.5,
        eve_regen_attr_id: 93750,
        eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    for _ in range(4):
        api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
        api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification - when neuts are applied together, they break through peak regen, but when
    # staggered, they do not
    api_options = [StatsOptionCapSim(), StatsOptionCapSim(stagger=True), StatsOptionCapSim(stagger=False)]
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(390)},
        {consts.ApiCapSimResult.stable: approx(0.2891368)},
        {consts.ApiCapSimResult.time: approx(390)}]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(390)},
        {consts.ApiCapSimResult.stable: approx(0.2891368)},
        {consts.ApiCapSimResult.time: approx(390)}]


def test_injector_emergency(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_boost_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_bonus)
    eve_use_amount_attr_id = client.mk_eve_attr()
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_use_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_amount_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_user_id = client.mk_eve_item(
        attrs={eve_use_amount_attr_id: 225, eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_use_effect_id],
        defeff_id=eve_use_effect_id)
    eve_inject_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.power_booster,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_injector_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 15, eve_cycle_time_attr_id: 12000, eve_reload_attr_id: 10000},
        eff_ids=[eve_inject_effect_id],
        defeff_id=eve_inject_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_boost_amount_attr_id: 400, eve_volume_attr_id: 12})
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 225, eve_regen_attr_id: 90000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_user_id, state=consts.ApiModuleState.active)
    api_fit.add_module(type_id=eve_injector_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    # Verification - at t0 ship has just enough cap for a single use of the module, then just before
    # next cycle at t10 injector is used, and finally at t20 injector can't cover module needs, so
    # it runs out
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=True))
    assert api_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.time: approx(20)}
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=True))
    assert api_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.time: approx(20)}


def test_zeros(client, consts):
    # Zero cap, no cap use
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 0, eve_regen_attr_id: 90000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification - stability value of 100% is exposed for this case
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=True))
    assert api_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: 1}
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=True))
    assert api_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: 1}
