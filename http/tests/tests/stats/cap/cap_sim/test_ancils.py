from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionCapSim


def test_armor(client, consts):
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
