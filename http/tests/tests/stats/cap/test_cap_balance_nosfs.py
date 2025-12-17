from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions, StatCapSrcKinds, StatsOptionCapBalance


def test_state(client, consts):
    eve_nosf_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
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
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_nosf = api_fit.add_module(type_id=eve_nosf_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(12)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(12)
    # Action
    api_nosf.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == 0


def test_override(client, consts):
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
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_nosf = api_fit.add_module(type_id=eve_nosf1_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(21)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(21)
    # Action
    api_nosf.change_module(type_id=eve_nosf2_id)
    # Verification - non-overridden nosfs also contribute to cap balance
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(12)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(12)


def test_src_kind(client, consts):
    eve_nosf_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_use_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_need)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_nosf_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_nosf_falloff,
        cat_id=consts.EveEffCat.target,
        discharge_attr_id=eve_use_amount_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_nosf_id = client.mk_eve_item(
        attrs={eve_nosf_amount_attr_id: 120, eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_nosf_effect_id],
        defeff_id=eve_nosf_effect_id)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_nosf_id, state=consts.ApiModuleState.active)
    # Verification
    api_options = [
        StatsOptionCapBalance(),
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(default=False, nosfs=True)),
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(default=True, nosfs=False))]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=(True, api_options)))
    assert api_fit_stats.cap_balance == [approx(12), approx(12), 0]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=(True, api_options)))
    assert api_ship_stats.cap_balance == [approx(12), approx(12), 0]
