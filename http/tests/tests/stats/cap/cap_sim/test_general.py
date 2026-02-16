from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions


def test_nosf_override(client, consts):
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
