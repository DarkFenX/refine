from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions


def test_different_signs(client, consts):
    # Internally, cap sim merges events which have equal time parameters (start delay, cycle
    # sequence and timings, output timings). Merging happens for positive changes and for negative
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
    # Verification - if events were merged, they'd cancel each other out, and cap stability would've
    # been at 100%. But since every 5 seconds cap is drained to 0 and then gets back to 100%, 50%
    # stability value is recorded.
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_sim=True))
    assert api_tgt_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.5)}
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_sim=True))
    assert api_tgt_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.5)}
