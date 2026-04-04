from fw import check_no_field
from fw.api import FitStatsOptions, ItemStatsOptions


def test_no_cap_changes(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == 0


def test_ship_absent(client, consts):
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
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_nosf_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance is None


def test_ship_not_loaded(client, consts):
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
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_nosf_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance is None


def test_incorrect_item_kind(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
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
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_char = api_fit.set_character(type_id=eve_ship_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_nosf_id, state=consts.ApiModuleState.active)
    # Verification
    api_char_stats = api_char.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_char_stats.cap_balance is None


def test_not_requested(client, consts):
    eve_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_ship_id = client.mk_eve_ship(attrs={eve_amount_attr_id: 518.76, eve_regen_attr_id: 233437.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=False))
    with check_no_field():
        api_fit_stats.cap_balance  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=False))
    with check_no_field():
        api_ship_stats.cap_balance  # noqa: B018
