from fw import check_no_field
from fw.api import FitStatsOptions, ItemStatsOptions


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


def test_ship_not_loaded(client, consts):
    eve_use_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 30, eve_cycle_time_attr_id: 15000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=True))
    assert api_fit_stats.cap_sim is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=True))
    assert api_ship_stats.cap_sim is None


def test_ship_absent(client, consts):
    eve_use_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 30, eve_cycle_time_attr_id: 15000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=True))
    assert api_fit_stats.cap_sim is None


def test_incorrect_item_kind(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_use_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 30, eve_cycle_time_attr_id: 15000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 225, eve_regen_attr_id: 90000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_char = api_fit.set_character(type_id=eve_ship_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_char_stats = api_char.get_stats(options=ItemStatsOptions(cap_sim=True))
    assert api_char_stats.cap_sim is None


def test_not_requested(client, consts):
    eve_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_ship_id = client.mk_eve_ship(attrs={eve_amount_attr_id: 518.76, eve_regen_attr_id: 233437.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=False))
    with check_no_field():
        api_fit_stats.cap_sim  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=False))
    with check_no_field():
        api_ship_stats.cap_sim  # noqa: B018
