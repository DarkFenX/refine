from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionCapSim


def test_high_fluctuation(client, consts):
    # In pyfa, stability is defined by combination of two different low cap watermarks, which gives
    # super low value for some ships; the library uses different method to calculate stability
    # value, which is tested here
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_boost_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_bonus)
    eve_use_amount_attr_id = client.mk_eve_attr()
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_use_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_amount_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_user_id = client.mk_eve_item(
        attrs={eve_use_amount_attr_id: 45, eve_cycle_time_attr_id: 2448},
        eff_ids=[eve_use_effect_id],
        defeff_id=eve_use_effect_id)
    eve_inject_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.power_booster,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_injector_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 15, eve_cycle_time_attr_id: 12000, eve_reload_attr_id: 10000},
        eff_ids=[eve_inject_effect_id],
        defeff_id=eve_inject_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_boost_amount_attr_id: 400, eve_volume_attr_id: 12})
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 346.875, eve_regen_attr_id: 138750})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_user_id, state=consts.ApiModuleState.active)
    api_fit.add_module(type_id=eve_injector_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=True))
    assert api_fit_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.5002785)}
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=True))
    assert api_ship_stats.cap_sim.one() == {consts.ApiCapSimResult.stable: approx(0.5002785)}


def test_no_events(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 225, eve_regen_attr_id: 90000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_options = [StatsOptionCapSim(cap_perc=0), StatsOptionCapSim(cap_perc=0.3), StatsOptionCapSim(cap_perc=1)]
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
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 225, eve_regen_attr_id: 90000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_injector_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    # Verification
    api_options = [StatsOptionCapSim(cap_perc=0), StatsOptionCapSim(cap_perc=0.3), StatsOptionCapSim(cap_perc=1)]
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


def test_only_transfers(client, consts):
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
    api_options = [StatsOptionCapSim(cap_perc=0), StatsOptionCapSim(cap_perc=0.3), StatsOptionCapSim(cap_perc=1)]
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
