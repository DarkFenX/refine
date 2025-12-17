from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions, StatCapSrcKinds, StatsOptionCapBalance


def test_state(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 600, eve_cycle_time_attr_id: 24000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 1000, eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == approx(-25)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == approx(-25)
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
    assert api_tgt_fit_stats.cap_balance.one() == approx(-25)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == approx(-25)


def test_nosf_override(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_nosf_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_override_attr_id = client.mk_eve_attr(id_=consts.EveAttr.nos_override)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_nosf_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module1_id = client.mk_eve_item(
        attrs={eve_nosf_amount_attr_id: 210, eve_cycle_time_attr_id: 10000, eve_override_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item(
        attrs={eve_nosf_amount_attr_id: 210, eve_cycle_time_attr_id: 10000, eve_override_attr_id: 0},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 1000, eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == approx(-21)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == approx(-21)
    # Action
    api_src_module.change_module(type_id=eve_module2_id)
    # Verification - non-overridden nosfs are not considered as neuts
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == 0
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == 0


def test_range_and_limit(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 600,
            eve_cycle_time_attr_id: 24000,
            eve_optimal_attr_id: 20000,
            eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 400})
    eve_tgt_ship_id = client.mk_eve_ship(attrs={
        eve_ship_amount_attr_id: 450,
        eve_radius_attr_id: 120,
        eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 20520, 0))
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == approx(-18.75)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == approx(-18.75)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 30520, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == approx(-12.5)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == approx(-12.5)


def test_resist_and_limit(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_resist_attr_id = client.mk_eve_attr(def_val=1)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        resist_attr_id=eve_resist_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 600, eve_cycle_time_attr_id: 24000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_src_ship_id = client.mk_eve_ship()
    eve_tgt_ship1_id = client.mk_eve_ship(
        attrs={eve_ship_amount_attr_id: 450, eve_resist_attr_id: 0.9, eve_sig_radius_attr_id: 1})
    eve_tgt_ship2_id = client.mk_eve_ship(
        attrs={eve_ship_amount_attr_id: 450, eve_resist_attr_id: 0.4, eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id)
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == approx(-18.75)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == approx(-18.75)
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == approx(-10)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == approx(-10)


def test_application_and_limit(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_resolution_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_sig_res)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 4400, eve_cycle_time_attr_id: 48000, eve_resolution_attr_id: 8000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_src_ship_id = client.mk_eve_ship()
    eve_tgt_ship1_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 3000, eve_sig_radius_attr_id: 10000})
    eve_tgt_ship2_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 3000, eve_sig_radius_attr_id: 2000})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id)
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == approx(-62.5)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == approx(-62.5)
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == approx(-22.916667)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == approx(-22.916667)


def test_bomb(client, consts):
    # Bombs are special because their effect does not carry duration directly
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reactivation_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.module_reactivation_delay)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.use_missiles,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_bomb_effect_id = client.mk_eve_effect(id_=consts.EveEffect.bomb_launching, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_cycle_time_attr_id: 10000, eve_reactivation_time_attr_id: 67500, eve_capacity_attr_id: 300},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 1800, eve_volume_attr_id: 75},
        eff_ids=[eve_bomb_effect_id],
        defeff_id=eve_bomb_effect_id)
    eve_tgt_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 3000, eve_sig_radius_attr_id: 2000})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_tgt_fit_stats.cap_balance.one() == approx(-23.225806)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_tgt_ship_stats.cap_balance.one() == approx(-23.225806)


def test_src_kind(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 600, eve_cycle_time_attr_id: 24000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 1000, eve_sig_radius_attr_id: 1})
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
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(default=False, incoming_neuts=True)),
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(default=True, incoming_neuts=False))]
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(cap_balance=(True, api_options)))
    assert api_tgt_fit_stats.cap_balance == [approx(-25.0), approx(-25.0), 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(cap_balance=(True, api_options)))
    assert api_tgt_ship_stats.cap_balance == [approx(-25.0), approx(-25.0), 0]


def test_effect_no_duration(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.energy_neut_falloff, cat_id=consts.EveEffCat.target)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 600, eve_cycle_time_attr_id: 24000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 1000, eve_sig_radius_attr_id: 1})
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
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 600, eve_cycle_time_attr_id: 0},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 1000, eve_sig_radius_attr_id: 1})
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
