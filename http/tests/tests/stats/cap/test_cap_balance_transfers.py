from tests import approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions, StatCapSrcKinds, StatsOptionCapBalance


def test_state(client, consts):
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
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == approx(70.2)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == approx(70.2)
    # Action
    api_src_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == 0
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == 0
    # Action
    api_src_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == approx(70.2)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == approx(70.2)


def test_range(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_range_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        range_attr_id=eve_range_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 351, eve_cycle_time_attr_id: 5000, eve_range_attr_id: 9750},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 400})
    eve_tgt_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 500, eve_radius_attr_id: 120})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 10269, 0))
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == approx(70.2)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == approx(70.2)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 10271, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == 0
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == 0


def test_resist_and_limit(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr(def_val=1)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        resist_attr_id=eve_resist_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 351, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 300})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 300, eve_resist_attr_id: 0.7})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship1_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == approx(60)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == approx(60)
    # Action
    api_tgt_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == approx(49.14)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == approx(49.14)


def test_src_kind(client, consts):
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
    api_options = [
        StatsOptionCapBalance(),
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(default=False, incoming_transfers=True)),
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(default=True, incoming_transfers=False))]
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=(True, api_options)))
    assert api_tgt_fit_stats.cap_balance == [approx(70.2), approx(70.2), 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=(True, api_options)))
    assert api_tgt_ship_stats.cap_balance == [approx(70.2), approx(70.2), 0]


def test_effect_no_duration(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target)
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
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == 0
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == 0


def test_attr_cycle_time_zero(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 351, eve_cycle_time_attr_id: 0},
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
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == 0
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == 0
