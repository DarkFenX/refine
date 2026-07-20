from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionCapSim


def test_override(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_nosf_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_override_attr_id = client.mk_eve_attr(id_=consts.EveAttr.nos_override)
    eve_use_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_need)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_nosf_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_nosf_falloff,
        cat_id=consts.EveEffCat.target,
        discharge_attr_id=eve_use_amount_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_nosf1_id = client.mk_eve_item(
        attrs={eve_nosf_amount_attr_id: 210, eve_cycle_time_attr_id: 10000, eve_override_attr_id: 1},
        eff_ids=[eve_nosf_effect_id],
        defeff_id=eve_nosf_effect_id)
    eve_nosf2_id = client.mk_eve_item(
        attrs={eve_nosf_amount_attr_id: 120, eve_cycle_time_attr_id: 10000, eve_override_attr_id: 0},
        eff_ids=[eve_nosf_effect_id],
        defeff_id=eve_nosf_effect_id)
    eve_use_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_amount_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_consumer_id = client.mk_eve_item(
        attrs={eve_use_amount_attr_id: 1000, eve_cycle_time_attr_id: 20000},
        eff_ids=[eve_use_effect_id],
        defeff_id=eve_use_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 1812.5, eve_regen_attr_id: 93750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_nosf = api_fit.add_module(type_id=eve_nosf1_id, state=consts.ApiModuleState.active)
    api_fit.add_module(type_id=eve_consumer_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=True))
    assert api_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.6782596)}
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=True))
    assert api_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.6782596)}
    # Action
    api_nosf.change_module(type_id=eve_nosf2_id)
    # Verification - active nosfs yield cap to the cap sim even when they are non-overridden
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=True))
    assert api_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.5139582)}
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=True))
    assert api_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.5139582)}
    # Action
    api_nosf.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=True))
    assert api_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.time: approx(80)}
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=True))
    assert api_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.time: approx(80)}


def test_projection_range_and_limit(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_nosf_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_override_attr_id = client.mk_eve_attr(id_=consts.EveAttr.nos_override)
    eve_use_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_need)
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_nosf_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_nosf_falloff,
        cat_id=consts.EveEffCat.target,
        discharge_attr_id=eve_use_amount_attr_id,
        duration_attr_id=eve_cycle_time_attr_id,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id)
    eve_nosf_id = client.mk_eve_item(
        attrs={
            eve_nosf_amount_attr_id: 60,
            eve_cycle_time_attr_id: 5000,
            eve_override_attr_id: 0,
            eve_optimal_attr_id: 20000,
            eve_falloff_attr_id: 10000},
        eff_ids=[eve_nosf_effect_id],
        defeff_id=eve_nosf_effect_id)
    eve_use_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_amount_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_consumer_id = client.mk_eve_item(
        attrs={eve_use_amount_attr_id: 10, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_use_effect_id],
        defeff_id=eve_use_effect_id)
    eve_src_ship_id = client.mk_eve_ship(
        attrs={eve_ship_amount_attr_id: 500, eve_regen_attr_id: 250000, eve_radius_attr_id: 400})
    eve_tgt_ship_id = client.mk_eve_ship(attrs={
        eve_ship_amount_attr_id: 50, eve_radius_attr_id: 120, eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_ship = api_src_fit.set_ship(type_id=eve_src_ship_id)
    api_src_fit.add_module(type_id=eve_nosf_id, state=consts.ApiModuleState.active)
    api_src_fit.add_module(type_id=eve_consumer_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 20520, 0))
    # Verification - gain limited by target ship cap amount
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        cap_sim=(True, [StatsOptionCapSim(nosf_projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.9519129)}
    api_src_ship_stats = api_src_ship.get_stats(options=ItemStatsOptions(
        cap_sim=(True, [StatsOptionCapSim(nosf_projectee_item_id=api_tgt_ship.id)])))
    assert api_src_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.9519129)}
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 30520, 0))
    # Verification - gain limited by range
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        cap_sim=(True, [StatsOptionCapSim(nosf_projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.529436)}
    api_src_ship_stats = api_src_ship.get_stats(options=ItemStatsOptions(
        cap_sim=(True, [StatsOptionCapSim(nosf_projectee_item_id=api_tgt_ship.id)])))
    assert api_src_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.529436)}


def test_projection_resist_and_limit(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_nosf_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_override_attr_id = client.mk_eve_attr(id_=consts.EveAttr.nos_override)
    eve_use_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_need)
    eve_resist_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_nosf_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_nosf_falloff,
        cat_id=consts.EveEffCat.target,
        discharge_attr_id=eve_use_amount_attr_id,
        duration_attr_id=eve_cycle_time_attr_id,
        resist_attr_id=eve_resist_attr_id)
    eve_nosf_id = client.mk_eve_item(
        attrs={eve_nosf_amount_attr_id: 60, eve_cycle_time_attr_id: 5000, eve_override_attr_id: 0},
        eff_ids=[eve_nosf_effect_id],
        defeff_id=eve_nosf_effect_id)
    eve_use_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_amount_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_consumer_id = client.mk_eve_item(
        attrs={eve_use_amount_attr_id: 10, eve_cycle_time_attr_id: 1000},
        eff_ids=[eve_use_effect_id],
        defeff_id=eve_use_effect_id)
    eve_src_ship_id = client.mk_eve_ship(
        attrs={eve_ship_amount_attr_id: 500, eve_regen_attr_id: 250000, eve_radius_attr_id: 400})
    eve_tgt_ship1_id = client.mk_eve_ship(
        attrs={eve_ship_amount_attr_id: 100, eve_resist_attr_id: 0.9, eve_sig_radius_attr_id: 1})
    eve_tgt_ship2_id = client.mk_eve_ship(
        attrs={eve_ship_amount_attr_id: 100, eve_resist_attr_id: 0.4, eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_ship = api_src_fit.set_ship(type_id=eve_src_ship_id)
    api_src_fit.add_module(type_id=eve_nosf_id, state=consts.ApiModuleState.active)
    api_src_fit.add_module(type_id=eve_consumer_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id)
    # Verification - gain limited by target ship cap amount
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        cap_sim=(True, [StatsOptionCapSim(nosf_projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.9519129)}
    api_src_ship_stats = api_src_ship.get_stats(options=ItemStatsOptions(
        cap_sim=(True, [StatsOptionCapSim(nosf_projectee_item_id=api_tgt_ship.id)])))
    assert api_src_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.9519129)}
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification - gain limited by resisted nosf amount
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        cap_sim=(True, [StatsOptionCapSim(nosf_projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.time: approx(674)}
    api_src_ship_stats = api_src_ship.get_stats(options=ItemStatsOptions(
        cap_sim=(True, [StatsOptionCapSim(nosf_projectee_item_id=api_tgt_ship.id)])))
    assert api_src_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.time: approx(674)}


def test_incorrect_projectee(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_nosf_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_override_attr_id = client.mk_eve_attr(id_=consts.EveAttr.nos_override)
    eve_use_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_need)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_nosf_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_nosf_falloff,
        cat_id=consts.EveEffCat.target,
        discharge_attr_id=eve_use_amount_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_nosf_id = client.mk_eve_item(
        attrs={eve_nosf_amount_attr_id: 120, eve_cycle_time_attr_id: 10000, eve_override_attr_id: 0},
        eff_ids=[eve_nosf_effect_id],
        defeff_id=eve_nosf_effect_id)
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 500})
    eve_tgt_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 100, eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_ship = api_src_fit.set_ship(type_id=eve_src_ship_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    api_nosf = api_src_fit.add_module(type_id=eve_nosf_id, state=consts.ApiModuleState.active)
    api_tmp = api_src_fit.add_module(type_id=eve_nosf_id)
    api_tmp.remove()
    # Verification - specifying incorrect projectee item IDs should fail only that specific option,
    # not whole stat batch
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(cap_sim=(True, [
        StatsOptionCapSim(nosf_projectee_item_id=api_tmp.id),
        StatsOptionCapSim(nosf_projectee_item_id=api_nosf.id),
        StatsOptionCapSim(nosf_projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.cap_sim == [None, None, {consts.ApiCapSimResult.stable: 1}]
    api_src_ship_stats = api_src_ship.get_stats(options=ItemStatsOptions(cap_sim=(True, [
        StatsOptionCapSim(nosf_projectee_item_id=api_tmp.id),
        StatsOptionCapSim(nosf_projectee_item_id=api_nosf.id),
        StatsOptionCapSim(nosf_projectee_item_id=api_tgt_ship.id)])))
    assert api_src_ship_stats.cap_sim == [None, None, {consts.ApiCapSimResult.stable: 1}]
