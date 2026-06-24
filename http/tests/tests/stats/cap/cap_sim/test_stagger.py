from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionCapSim


def test_consumers(client, consts):
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


def test_neuts(client, consts):
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
        api_src_module.change_module(add_proj_item_ids=[api_tgt_ship.id])
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


def test_transfers(client, consts):
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
        api_src_module.change_module(add_proj_item_ids=[api_tgt_ship.id])
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


def test_cross_group(client, consts):
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
    api_src_neut.change_module(add_proj_item_ids=[api_tgt_ship.id])
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


def test_different_amounts(client, consts):
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
        api_src_module.change_module(add_proj_item_ids=[api_tgt_ship.id])
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


def test_different_delays(client, consts):
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
        api_src_module.change_module(add_proj_item_ids=[api_tgt_ship.id])
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


def test_exceptions(client, consts):
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
    api_src_neut1.change_module(add_proj_item_ids=[api_tgt_ship.id])
    api_src_neut2 = api_src_fit.add_module(type_id=eve_neut_id, state=consts.ApiModuleState.active)
    api_src_neut2.change_module(add_proj_item_ids=[api_tgt_ship.id])
    api_src_neut3 = api_src_fit.add_module(type_id=eve_neut_id, state=consts.ApiModuleState.active)
    api_src_neut3.change_module(add_proj_item_ids=[api_tgt_ship.id])
    api_src_neut4 = api_src_fit.add_module(type_id=eve_neut_id, state=consts.ApiModuleState.active)
    api_src_neut4.change_module(add_proj_item_ids=[api_tgt_ship.id])
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
