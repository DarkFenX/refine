from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionCapSim


def test_starting_cap(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_use_amount_attr_id = client.mk_eve_attr()
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_use_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_amount_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_user_id = client.mk_eve_item(
        attrs={eve_use_amount_attr_id: 12, eve_cycle_time_attr_id: 2000},
        eff_ids=[eve_use_effect_id],
        defeff_id=eve_use_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 346.875, eve_regen_attr_id: 138750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_user_id, state=consts.ApiModuleState.active)
    # Verification
    api_options = [StatsOptionCapSim(cap_perc=0.1), StatsOptionCapSim(cap_perc=0.3), StatsOptionCapSim(cap_perc=1)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(16)},
        {consts.ApiCapSimResult.stable: approx(0.3595835)},
        {consts.ApiCapSimResult.stable: approx(0.3595835)}]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(16)},
        {consts.ApiCapSimResult.stable: approx(0.3595835)},
        {consts.ApiCapSimResult.stable: approx(0.3595835)}]


def test_no_events(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 225, eve_regen_attr_id: 90000})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 225})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    api_options = [StatsOptionCapSim(cap_perc=0), StatsOptionCapSim(cap_perc=0.3), StatsOptionCapSim(cap_perc=1)]
    # Verification - without any cap use, regen gets cap to 100%
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
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification - without any cap use and regen, starting cap percentage is returned
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: approx(0)},
        {consts.ApiCapSimResult.stable: approx(0.3)},
        {consts.ApiCapSimResult.stable: approx(1)}]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: approx(0)},
        {consts.ApiCapSimResult.stable: approx(0.3)},
        {consts.ApiCapSimResult.stable: approx(1)}]


def test_no_events_with_self_killer(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_use_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.emergency_hull_energizer,
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 12000, eve_cycle_time_attr_id: 17500},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 60000, eve_regen_attr_id: 2767500})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 60000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_options = [StatsOptionCapSim(cap_perc=0.1), StatsOptionCapSim(cap_perc=0.3), StatsOptionCapSim(cap_perc=1)]
    # Verification - with the module which takes cap only once, cap gets back to 100% after module
    # stops cycling. Instability is returned only when there is not enough cap to use the module
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification - without any regen, starting cap minus used amount is returned as stability
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: approx(0.1)},
        {consts.ApiCapSimResult.stable: approx(0.8)}]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: approx(0.1)},
        {consts.ApiCapSimResult.stable: approx(0.8)}]


def test_zeroed_positive_events(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_boost_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_bonus)
    eve_nosf_override_attr_id = client.mk_eve_attr(id_=consts.EveAttr.nos_override)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_injector_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.power_booster,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_injector_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 640, eve_cycle_time_attr_id: 12000, eve_reload_attr_id: 10000},
        eff_ids=[eve_injector_effect_id],
        defeff_id=eve_injector_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_boost_amount_attr_id: 0, eve_volume_attr_id: 96})
    eve_nosf_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_nosf_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_nosf_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 0, eve_cycle_time_attr_id: 10000, eve_nosf_override_attr_id: 1},
        eff_ids=[eve_nosf_effect_id],
        defeff_id=eve_nosf_effect_id)
    eve_transfer_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_transfer_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 0, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_transfer_effect_id],
        defeff_id=eve_transfer_effect_id)
    eve_tgt_ship1_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 225, eve_regen_attr_id: 90000})
    eve_tgt_ship2_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 225})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_transfer = api_src_fit.add_module(type_id=eve_transfer_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id)
    api_tgt_fit.add_module(type_id=eve_injector_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit.add_module(type_id=eve_nosf_id, state=consts.ApiModuleState.active)
    api_src_transfer.change_module(add_projs=[api_tgt_ship.id])
    api_options = [StatsOptionCapSim(cap_perc=0), StatsOptionCapSim(cap_perc=0.3), StatsOptionCapSim(cap_perc=1)]
    # Verification - without any cap use, regen gets cap to 100%
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
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification - without any cap use and without cap regen, starting cap percentage is returned
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: approx(0)},
        {consts.ApiCapSimResult.stable: approx(0.3)},
        {consts.ApiCapSimResult.stable: approx(1)}]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: approx(0)},
        {consts.ApiCapSimResult.stable: approx(0.3)},
        {consts.ApiCapSimResult.stable: approx(1)}]


def test_zeroed_positive_events_with_self_killer(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_boost_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_bonus)
    eve_nosf_override_attr_id = client.mk_eve_attr(id_=consts.EveAttr.nos_override)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_use_attr_id = client.mk_eve_attr()
    eve_injector_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.power_booster,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_injector_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 640, eve_cycle_time_attr_id: 12000, eve_reload_attr_id: 10000},
        eff_ids=[eve_injector_effect_id],
        defeff_id=eve_injector_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_boost_amount_attr_id: 0, eve_volume_attr_id: 96})
    eve_nosf_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_nosf_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_nosf_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 0, eve_cycle_time_attr_id: 10000, eve_nosf_override_attr_id: 1},
        eff_ids=[eve_nosf_effect_id],
        defeff_id=eve_nosf_effect_id)
    eve_transfer_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_transfer_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 0, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_transfer_effect_id],
        defeff_id=eve_transfer_effect_id)
    eve_sk_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.emergency_hull_energizer,
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_sk_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 12000, eve_cycle_time_attr_id: 17500},
        eff_ids=[eve_sk_effect_id],
        defeff_id=eve_sk_effect_id)
    eve_tgt_ship1_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 60000, eve_regen_attr_id: 2767500})
    eve_tgt_ship2_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 60000})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_transfer = api_src_fit.add_module(type_id=eve_transfer_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id)
    api_tgt_fit.add_module(type_id=eve_injector_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit.add_module(type_id=eve_nosf_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(type_id=eve_sk_module_id, state=consts.ApiModuleState.active)
    api_src_transfer.change_module(add_projs=[api_tgt_ship.id])
    api_options = [StatsOptionCapSim(cap_perc=0.1), StatsOptionCapSim(cap_perc=0.3), StatsOptionCapSim(cap_perc=1)]
    # Verification - with the module which takes cap only once, cap gets back to 100% after module
    # stops cycling. Instability is returned only when there is not enough cap to use the module
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification - without any regen, starting cap minus used amount is returned as stability
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: approx(0.1)},
        {consts.ApiCapSimResult.stable: approx(0.8)}]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: approx(0.1)},
        {consts.ApiCapSimResult.stable: approx(0.8)}]


def test_zeroed_negative_events(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_use_attr_id = client.mk_eve_attr()
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_consumer_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_consumer_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 0, eve_cycle_time_attr_id: 15000},
        eff_ids=[eve_consumer_effect_id],
        defeff_id=eve_consumer_effect_id)
    eve_neut_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_neut_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 0, eve_cycle_time_attr_id: 24000},
        eff_ids=[eve_neut_effect_id],
        defeff_id=eve_neut_effect_id)
    eve_tgt_ship1_id = client.mk_eve_ship(
        attrs={eve_ship_amount_attr_id: 225, eve_sig_radius_attr_id: 1, eve_regen_attr_id: 90000})
    eve_tgt_ship2_id = client.mk_eve_ship(
        attrs={eve_ship_amount_attr_id: 225, eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_neut = api_src_fit.add_module(type_id=eve_neut_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id)
    api_tgt_fit.add_module(type_id=eve_consumer_id, state=consts.ApiModuleState.active)
    api_src_neut.change_module(add_projs=[api_tgt_ship.id])
    api_options = [StatsOptionCapSim(cap_perc=0), StatsOptionCapSim(cap_perc=0.3), StatsOptionCapSim(cap_perc=1)]
    # Verification - without any cap use, regen gets cap to 100%
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
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification - without any cap use and without cap regen, starting cap percentage is returned
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: approx(0)},
        {consts.ApiCapSimResult.stable: approx(0.3)},
        {consts.ApiCapSimResult.stable: approx(1)}]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.stable: approx(0)},
        {consts.ApiCapSimResult.stable: approx(0.3)},
        {consts.ApiCapSimResult.stable: approx(1)}]


def test_zeroed_negative_events_with_self_killer(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_use_attr_id = client.mk_eve_attr()
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_consumer_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_consumer_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 0, eve_cycle_time_attr_id: 15000},
        eff_ids=[eve_consumer_effect_id],
        defeff_id=eve_consumer_effect_id)
    eve_neut_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_neut_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 0, eve_cycle_time_attr_id: 24000},
        eff_ids=[eve_neut_effect_id],
        defeff_id=eve_neut_effect_id)
    eve_sk_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.emergency_hull_energizer,
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_sk_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 12000, eve_cycle_time_attr_id: 17500},
        eff_ids=[eve_sk_effect_id],
        defeff_id=eve_sk_effect_id)
    eve_tgt_ship1_id = client.mk_eve_ship(
        attrs={eve_ship_amount_attr_id: 60000, eve_sig_radius_attr_id: 1, eve_regen_attr_id: 2767500})
    eve_tgt_ship2_id = client.mk_eve_ship(
        attrs={eve_ship_amount_attr_id: 60000, eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_neut = api_src_fit.add_module(type_id=eve_neut_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id)
    api_tgt_fit.add_module(type_id=eve_consumer_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(type_id=eve_sk_module_id, state=consts.ApiModuleState.active)
    api_src_neut.change_module(add_projs=[api_tgt_ship.id])
    api_options = [StatsOptionCapSim(cap_perc=0.1), StatsOptionCapSim(cap_perc=0.3), StatsOptionCapSim(cap_perc=1)]
    # Verification - with the module which takes cap only once, cap gets back to 100% after module
    # stops cycling. Instability is returned only when there is not enough cap to use the module
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification - without any regen, starting cap minus used amount is returned as stability
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: approx(0.1)},
        {consts.ApiCapSimResult.stable: approx(0.8)}]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: approx(0.1)},
        {consts.ApiCapSimResult.stable: approx(0.8)}]


def test_only_injects(client, consts):
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
    eve_ship1_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 225, eve_regen_attr_id: 90000})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 225})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    api_fit.add_module(type_id=eve_injector_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_options = [StatsOptionCapSim(cap_perc=0), StatsOptionCapSim(cap_perc=0.3), StatsOptionCapSim(cap_perc=1)]
    # Verification
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
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
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


def test_only_injects_with_self_killer(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_boost_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_bonus)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_use_attr_id = client.mk_eve_attr()
    eve_inject_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.power_booster,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_injector_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 640, eve_cycle_time_attr_id: 12000, eve_reload_attr_id: 10000},
        eff_ids=[eve_inject_effect_id],
        defeff_id=eve_inject_effect_id)
    eve_sk_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.emergency_hull_energizer,
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_sk_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 12000, eve_cycle_time_attr_id: 17500},
        eff_ids=[eve_sk_effect_id],
        defeff_id=eve_sk_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_boost_amount_attr_id: 3200, eve_volume_attr_id: 96})
    eve_ship1_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 60000, eve_regen_attr_id: 2767500})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 60000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    api_fit.add_module(type_id=eve_injector_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fit.add_module(type_id=eve_sk_module_id, state=consts.ApiModuleState.active)
    api_options = [StatsOptionCapSim(cap_perc=0.1), StatsOptionCapSim(cap_perc=0.3), StatsOptionCapSim(cap_perc=1)]
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]


def test_only_transfers(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 351, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 500, eve_regen_attr_id: 90000})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship1_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    api_options = [StatsOptionCapSim(cap_perc=0), StatsOptionCapSim(cap_perc=0.3), StatsOptionCapSim(cap_perc=1)]
    # Verification
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
    # Action
    api_tgt_ship.change_ship(type_id=eve_ship2_id)
    # Verification
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


def test_only_transfers_with_self_killer(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_use_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 351, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_sk_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.emergency_hull_energizer,
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_sk_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 12000, eve_cycle_time_attr_id: 17500},
        eff_ids=[eve_sk_effect_id],
        defeff_id=eve_sk_effect_id)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 60000, eve_regen_attr_id: 2767500})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 60000})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship1_id)
    api_tgt_fit.add_module(type_id=eve_sk_module_id, state=consts.ApiModuleState.active)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    api_options = [StatsOptionCapSim(cap_perc=0.1), StatsOptionCapSim(cap_perc=0.3), StatsOptionCapSim(cap_perc=1)]
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]
    # Action
    api_tgt_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, api_options)))
    assert api_tgt_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(0)},
        {consts.ApiCapSimResult.stable: 1},
        {consts.ApiCapSimResult.stable: 1}]


def test_ancil_armor(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_use_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fueled_armor_repair,
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_use_attr_id: 40,
            eve_cycle_time_attr_id: 4500,
            eve_reload_attr_id: 60000,
            eve_capacity_attr_id: 0.08},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.01})
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 346.875, eve_regen_attr_id: 180000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=(True, [
        StatsOptionCapSim(optional_reloads=consts.ApiOptionalReload.disabled),
        StatsOptionCapSim(optional_reloads=consts.ApiOptionalReload.on_empty)])))
    assert api_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(63)},
        {consts.ApiCapSimResult.stable: approx(0.5492466)}]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, [
        StatsOptionCapSim(optional_reloads=consts.ApiOptionalReload.disabled),
        StatsOptionCapSim(optional_reloads=consts.ApiOptionalReload.on_empty)])))
    assert api_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(63)},
        {consts.ApiCapSimResult.stable: approx(0.5492466)}]
    # Action
    api_module.change_module(charge_type_id=None)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=(True, [
        StatsOptionCapSim(optional_reloads=consts.ApiOptionalReload.disabled),
        StatsOptionCapSim(optional_reloads=consts.ApiOptionalReload.on_empty)])))
    assert api_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(63)},
        {consts.ApiCapSimResult.time: approx(63)}]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, [
        StatsOptionCapSim(optional_reloads=consts.ApiOptionalReload.disabled),
        StatsOptionCapSim(optional_reloads=consts.ApiOptionalReload.on_empty)])))
    assert api_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(63)},
        {consts.ApiCapSimResult.time: approx(63)}]


def test_ancil_shield(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_use_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_cap_bonus_attr_id = client.mk_eve_attr(id_=consts.EveAttr.cap_need_bonus)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fueled_shield_boosting,
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_use_attr_id: 178.2,
            eve_cycle_time_attr_id: 3000,
            eve_reload_attr_id: 60000,
            eve_capacity_attr_id: 14},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_charge_id = client.mk_eve_item(
        grp_id=consts.EveItemGrp.capacitor_booster_charge,
        attrs={eve_volume_attr_id: 1.5, eve_cap_bonus_attr_id: -100})
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 346.875, eve_regen_attr_id: 180000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=(True, [
        StatsOptionCapSim(optional_reloads=consts.ApiOptionalReload.disabled),
        StatsOptionCapSim(optional_reloads=consts.ApiOptionalReload.on_empty)])))
    assert api_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(33)},
        {consts.ApiCapSimResult.stable: approx(1)}]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, [
        StatsOptionCapSim(optional_reloads=consts.ApiOptionalReload.disabled),
        StatsOptionCapSim(optional_reloads=consts.ApiOptionalReload.on_empty)])))
    assert api_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(33)},
        {consts.ApiCapSimResult.stable: approx(1)}]
    # Action
    api_module.change_module(charge_type_id=None)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=(True, [
        StatsOptionCapSim(optional_reloads=consts.ApiOptionalReload.disabled),
        StatsOptionCapSim(optional_reloads=consts.ApiOptionalReload.on_empty)])))
    assert api_fit_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(6)},
        {consts.ApiCapSimResult.time: approx(6)}]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, [
        StatsOptionCapSim(optional_reloads=consts.ApiOptionalReload.disabled),
        StatsOptionCapSim(optional_reloads=consts.ApiOptionalReload.on_empty)])))
    assert api_ship_stats.cap_sim == [
        {consts.ApiCapSimResult.time: approx(6)},
        {consts.ApiCapSimResult.time: approx(6)}]
