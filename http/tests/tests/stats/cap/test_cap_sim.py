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


def test_stagger_consumers(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_use_amount_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_amount_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_use_amount_attr_id: 240, eve_cycle_time_attr_id: 60000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 375, eve_regen_attr_id: 93750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification - no cap to run mods at all when not staggered
    api_options = [StatsOptionCapSim(), StatsOptionCapSim(stagger=True), StatsOptionCapSim(stagger=False)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: 0},
        {consts.ApiCapSimResult.stable: approx(0.4264583)},
        {consts.ApiCapSimResult.time: 0}]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: 0},
        {consts.ApiCapSimResult.stable: approx(0.4264583)},
        {consts.ApiCapSimResult.time: 0}]


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
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_ship_amount_attr_id: 1812.5, eve_regen_attr_id: 93750, eve_sig_radius_attr_id: 1})
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
    api_options = [StatsOptionCapSim(stagger=True), StatsOptionCapSim(stagger=False)]
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: approx(0.2891368)},
        {consts.ApiCapSimResult.time: approx(390)}]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: approx(0.2891368)},
        {consts.ApiCapSimResult.time: approx(390)}]


def test_stagger_transfers(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_use_amount_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_use_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_amount_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_transfer_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_consumer_id = client.mk_eve_item(
        attrs={eve_use_amount_attr_id: 215, eve_cycle_time_attr_id: 3000},
        eff_ids=[eve_use_effect_id],
        defeff_id=eve_use_effect_id)
    eve_transfer_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 150, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_transfer_effect_id],
        defeff_id=eve_transfer_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 500, eve_regen_attr_id: 93750})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_fit.add_module(type_id=eve_consumer_id, state=consts.ApiModuleState.active)
    for _ in range(2):
        api_src_module = api_src_fit.add_module(type_id=eve_transfer_id, state=consts.ApiModuleState.active)
        api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_options = [StatsOptionCapSim(stagger=True), StatsOptionCapSim(stagger=False)]
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: approx(0.3963343)},
        {consts.ApiCapSimResult.time: approx(54)}]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: approx(0.3963343)},
        {consts.ApiCapSimResult.time: approx(54)}]


def test_stagger_cross_group(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_use_amount_attr_id = client.mk_eve_attr()
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_use_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_amount_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_neut_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_consumer_id = client.mk_eve_item(
        attrs={eve_use_amount_attr_id: 240, eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_use_effect_id],
        defeff_id=eve_use_effect_id)
    eve_neut_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 240, eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_neut_effect_id],
        defeff_id=eve_neut_effect_id)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_ship_amount_attr_id: 1812.5, eve_regen_attr_id: 93750, eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_fit.add_module(type_id=eve_consumer_id, state=consts.ApiModuleState.active)
    api_src_neut = api_src_fit.add_module(type_id=eve_neut_id, state=consts.ApiModuleState.active)
    api_src_neut.change_module(add_projs=[api_tgt_ship.id])
    # Verification - neuts and cap consumers are in different stagger groups, and are not staggered
    # against each other even if their cycle parameters coincide
    api_options = [StatsOptionCapSim(stagger=True), StatsOptionCapSim(stagger=False)]
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(390)},
        {consts.ApiCapSimResult.time: approx(390)}]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(390)},
        {consts.ApiCapSimResult.time: approx(390)}]


def test_stagger_different_amounts(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module1_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 180, eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 300, eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={
        eve_ship_amount_attr_id: 1830,
        eve_regen_attr_id: 93750,
        eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    for eve_module_id in (eve_module1_id, eve_module2_id):
        api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
        api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_options = [StatsOptionCapSim(stagger=True), StatsOptionCapSim(stagger=False)]
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: approx(0.3049515)},
        {consts.ApiCapSimResult.time: approx(630)}]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: approx(0.3049515)},
        {consts.ApiCapSimResult.time: approx(630)}]


def test_stagger_different_delays(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_nosf_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_override_attr_id = client.mk_eve_attr(id_=consts.EveAttr.nos_override)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_neut_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_nosf_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_nosf_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_neut_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 240, eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_neut_effect_id],
        defeff_id=eve_neut_effect_id)
    eve_nosf_id = client.mk_eve_item(
        attrs={eve_nosf_amount_attr_id: 240, eve_cycle_time_attr_id: 10000, eve_override_attr_id: 1},
        eff_ids=[eve_nosf_effect_id],
        defeff_id=eve_nosf_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={
        eve_ship_amount_attr_id: 1830,
        eve_regen_attr_id: 93750,
        eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    for eve_module_id in (eve_neut_id, eve_nosf_id):
        api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
        api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification - if nosf and neut were staggerable, target ship cap would've been stable. The
    # reason for that is that they have different application delays (neut is applied immediately,
    # nosf in the end of cycle), so the sim puts those into different staggering groups
    api_options = [StatsOptionCapSim(stagger=True), StatsOptionCapSim(stagger=False)]
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(640)},
        {consts.ApiCapSimResult.time: approx(640)}]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(640)},
        {consts.ApiCapSimResult.time: approx(640)}]


def test_stagger_exceptions(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_neut_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 120, eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_ship_amount_attr_id: 1812.5, eve_regen_attr_id: 93750, eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_neut1 = api_src_fit.add_module(type_id=eve_neut_id, state=consts.ApiModuleState.active)
    api_src_neut1.change_module(add_projs=[api_tgt_ship.id])
    api_src_neut2 = api_src_fit.add_module(type_id=eve_neut_id, state=consts.ApiModuleState.active)
    api_src_neut2.change_module(add_projs=[api_tgt_ship.id])
    api_src_neut3 = api_src_fit.add_module(type_id=eve_neut_id, state=consts.ApiModuleState.active)
    api_src_neut3.change_module(add_projs=[api_tgt_ship.id])
    api_src_neut4 = api_src_fit.add_module(type_id=eve_neut_id, state=consts.ApiModuleState.active)
    api_src_neut4.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_options = [
        StatsOptionCapSim(stagger=(True, [])),
        StatsOptionCapSim(stagger=(True, [api_src_neut1.id])),
        StatsOptionCapSim(stagger=(True, [api_src_neut1.id, api_src_neut2.id])),
        StatsOptionCapSim(stagger=(True, [api_src_neut1.id, api_src_neut2.id, api_src_neut3.id])),
        StatsOptionCapSim(stagger=(True, [api_src_neut1.id, api_src_neut2.id, api_src_neut3.id, api_src_neut4.id])),
        StatsOptionCapSim(stagger=(False, [])),
        StatsOptionCapSim(stagger=(False, [api_src_neut1.id])),
        StatsOptionCapSim(stagger=(False, [api_src_neut1.id, api_src_neut2.id])),
        StatsOptionCapSim(stagger=(False, [api_src_neut1.id, api_src_neut2.id, api_src_neut3.id])),
        StatsOptionCapSim(stagger=(False, [api_src_neut1.id, api_src_neut2.id, api_src_neut3.id, api_src_neut4.id])),
    ]
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: approx(0.2891368)},
        {consts.ApiCapSimResult.stable: approx(0.2803728)},
        {consts.ApiCapSimResult.time: approx(930)},
        {consts.ApiCapSimResult.time: approx(390)},
        {consts.ApiCapSimResult.time: approx(390)},
        {consts.ApiCapSimResult.time: approx(390)},
        {consts.ApiCapSimResult.time: approx(390)},
        {consts.ApiCapSimResult.time: approx(930)},
        {consts.ApiCapSimResult.stable: approx(0.2803728)},
        {consts.ApiCapSimResult.stable: approx(0.2891368)}]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: approx(0.2891368)},
        {consts.ApiCapSimResult.stable: approx(0.2803728)},
        {consts.ApiCapSimResult.time: approx(930)},
        {consts.ApiCapSimResult.time: approx(390)},
        {consts.ApiCapSimResult.time: approx(390)},
        {consts.ApiCapSimResult.time: approx(390)},
        {consts.ApiCapSimResult.time: approx(390)},
        {consts.ApiCapSimResult.time: approx(930)},
        {consts.ApiCapSimResult.stable: approx(0.2803728)},
        {consts.ApiCapSimResult.stable: approx(0.2891368)}]


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


def test_injector_topup(client, consts):
    # Whenever injector is ready, cap sim knows not to use it when not necessary. It will postpone
    # its use until it can make most use of its cap. We can check it indirectly by sim returning
    # lower stability value
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_boost_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_bonus)
    eve_use_amount_attr_id = client.mk_eve_attr()
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_use_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_nosf_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_user_id = client.mk_eve_item(
        attrs={eve_use_amount_attr_id: 40, eve_cycle_time_attr_id: 4500},
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
    eve_charge_id = client.mk_eve_item(attrs={eve_boost_amount_attr_id: 150, eve_volume_attr_id: 4.5})
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 375, eve_regen_attr_id: 93750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_user_id, state=consts.ApiModuleState.active)
    api_fit.add_module(type_id=eve_injector_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=True))
    assert api_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.7859468)}
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=True))
    assert api_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.7859468)}


def test_aggregation_different_signs(client, consts):
    # Internally, cap sim aggregates events which have equal starting time, same cycling, and same
    # output, aside from output amount. Aggregation happens for positive changes and for negative
    # changes separately. Here, we check that positive and negative events are processed separately.
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_override_attr_id = client.mk_eve_attr(id_=consts.EveAttr.nos_override)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_nosf_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_nosf_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_transfer_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_nosf_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 600, eve_cycle_time_attr_id: 10000, eve_override_attr_id: 1},
        eff_ids=[eve_nosf_effect_id],
        defeff_id=eve_nosf_effect_id)
    eve_transfer_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 600, eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_transfer_effect_id],
        defeff_id=eve_transfer_effect_id)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_ship_amount_attr_id: 500, eve_regen_attr_id: 93750, eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    for eve_module_id in (eve_nosf_id, eve_transfer_id):
        api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
        api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification - if events were aggregated, they'd cancel each other out, and cap stability
    # would've been at 100%. But since every 5 seconds cap is drained to 0 and then gets back to
    # 100%, 50% stability value is recorded.
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=True))
    assert api_tgt_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.5)}
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=True))
    assert api_tgt_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.5)}


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


def test_ship_not_loaded(client):
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=True))
    assert api_fit_stats.cap_sim is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=True))
    assert api_ship_stats.cap_sim is None


def test_ship_absent(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=True))
    assert api_fit_stats.cap_sim is None
