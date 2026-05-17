"""
Internally, cap sim merges events which have equal time parameters (start delay from staggering,
cycle sequence and its timings, output instance timings). When they match, sim attempts to merge
multiple different events into a single one.

Tests in this module cannot check that merging actually happens (it just saves sim performance), but
they check that if merging happens, it does not break basic logic and works like if it didn't
happen.
"""

from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionCapSim


def test_equal(client, consts):
    # Completely identical modules
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
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        cap_sim=(True, [StatsOptionCapSim(stagger=False)])))
    assert api_tgt_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.time: approx(390)}
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        cap_sim=(True, [StatsOptionCapSim(stagger=False)])))
    assert api_tgt_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.time: approx(390)}


def test_similar(client, consts):
    # Modules with identical cycling parameters, but different amounts
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
        attrs={eve_neut_amount_attr_id: 300, eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 60, eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module3_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 100, eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module4_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 20, eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_ship_amount_attr_id: 1812.5, eve_regen_attr_id: 93750, eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    for eve_module_id in [eve_module1_id, eve_module2_id, eve_module3_id, eve_module4_id]:
        api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
        api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        cap_sim=(True, [StatsOptionCapSim(stagger=False)])))
    assert api_tgt_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.time: approx(390)}
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        cap_sim=(True, [StatsOptionCapSim(stagger=False)])))
    assert api_tgt_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.time: approx(390)}


def test_different_signs(client, consts):
    # Merging happens for positive changes and for negative changes separately. Here, we check that
    # positive and negative events are processed separately.
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
    # Verification - if events were merged, they'd cancel each other out, and cap stability would've
    # been at 100%. But since every 5 seconds cap is drained to 0 and then gets back to 100%, 50%
    # stability value is recorded.
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        cap_sim=(True, [StatsOptionCapSim(stagger=False)])))
    assert api_tgt_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.5)}
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        cap_sim=(True, [StatsOptionCapSim(stagger=False)])))
    assert api_tgt_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.5)}


def test_hard_downtime(client, consts):

    def mk_fighter(refuel_time: float) -> int:
        # Not realistic fighter - use count on neuting ability is limited
        return client.mk_eve_fighter(
            attrs={
                eve_neut_amount_attr_id: 99,
                eve_cycle_time_attr_id: 6000,
                eve_refuel_time_attr_id: refuel_time,
                eve_max_count_attr_id: 3,
                eve_radius_attr_id: 35},
            eff_ids=[eve_effect_id],
            defeff_id=eve_effect_id,
            abils=[client.mk_eve_item_abil(id_=eve_abil_id, charge_count=5, charge_rearm_time=1)])

    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_energy_neut_amount)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_refuel_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_refueling_time)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_energy_neut,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_abil_id = client.mk_eve_abil(id_=consts.EveAbil.energy_neut)
    eve_fighter1_id = mk_fighter(refuel_time=5000)
    eve_fighter2_id = mk_fighter(refuel_time=50000)
    eve_tgt_ship_id = client.mk_eve_ship(
        attrs={eve_radius_attr_id: 220, eve_ship_amount_attr_id: 10000, eve_regen_attr_id: 500000})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter1 = api_src_fit.add_fighter(
        type_id=eve_fighter1_id,
        state=consts.ApiMinionState.engaging,
        rearm_minion=consts.ApiRearmMinion.on_first_empty)
    api_src_fighter2 = api_src_fit.add_fighter(
        type_id=eve_fighter2_id,
        state=consts.ApiMinionState.engaging,
        rearm_minion=consts.ApiRearmMinion.on_first_empty)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    api_src_fighter1.change_fighter(add_projs=[api_tgt_ship.id])
    api_src_fighter2.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification - fighters with similar stats but different hard downtime (refuel) duration
    # shouldn't be merged. If longer duration was taken for both, cap would've been stable; if
    # shorter - it'd last about 5 minutes
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        cap_sim=(True, [StatsOptionCapSim(stagger=False)])))
    assert api_tgt_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.time: approx(1149)}
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        cap_sim=(True, [StatsOptionCapSim(stagger=False)])))
    assert api_tgt_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.time: approx(1149)}
