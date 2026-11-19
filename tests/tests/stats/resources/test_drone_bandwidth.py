from fw import approx, check_no_field
from fw.api import FitStatsOptions, StatsOptionDroneBandwidth


def test_ship_affectors(client, consts):
    # Only modules which lower drone bandwidth are switched, e.g. warp core stabilizers and
    # interdiction nullifiers in EVE
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_output_attr_id)
    eve_online_effect_id = client.mk_eve_online_effect()
    eve_mod_online_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod])
    eve_mod_active_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod])
    eve_online_penalty_module_id = client.mk_eve_item(
        attrs={eve_mod_attr_id: -50},
        eff_ids=[eve_mod_online_effect_id, eve_online_effect_id])
    eve_active_penalty_module_id = client.mk_eve_item(
        attrs={eve_mod_attr_id: -50},
        eff_ids=[eve_mod_active_effect_id],
        defeff_id=eve_mod_active_effect_id)
    eve_bonus_module_id = client.mk_eve_item(
        attrs={eve_mod_attr_id: 50},
        eff_ids=[eve_mod_online_effect_id, eve_online_effect_id])
    eve_drone_id = client.mk_eve_drone(attrs={eve_use_attr_id: 25})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_online_module = api_fit.add_module(type_id=eve_online_penalty_module_id, state=consts.ApiModuleState.online)
    api_active_module = api_fit.add_module(type_id=eve_active_penalty_module_id, state=consts.ApiModuleState.active)
    api_overload_module = api_fit.add_module(type_id=eve_active_penalty_module_id, state=consts.ApiModuleState.overload)
    api_bonus_module = api_fit.add_module(type_id=eve_bonus_module_id, state=consts.ApiModuleState.online)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification - use is not switched, bonus module stays online
    api_options = [
        StatsOptionDroneBandwidth(),
        StatsOptionDroneBandwidth(affectors=consts.ApiCtlAffector.offline),
        StatsOptionDroneBandwidth(affectors=consts.ApiCtlAffector.deactivate),
        StatsOptionDroneBandwidth(affectors=consts.ApiCtlAffector.unmodified)]
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=api_options))
    assert api_stats.drone_bandwidth == [
        (approx(25), approx(18.75)),
        (approx(25), approx(150)),
        (approx(25), approx(75)),
        (approx(25), approx(18.75))]
    # Verification - module states are restored after stats are fetched
    assert api_online_module.update().state == consts.ApiModuleState.online
    assert api_active_module.update().state == consts.ApiModuleState.active
    assert api_overload_module.update().state == consts.ApiModuleState.overload
    assert api_bonus_module.update().state == consts.ApiModuleState.online
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(25), approx(18.75))


def test_ship_absent(client, consts):
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_output_attr_id)
    eve_online_effect_id = client.mk_eve_online_effect()
    eve_mod_online_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_mod_attr_id: -50},
        eff_ids=[eve_mod_online_effect_id, eve_online_effect_id])
    eve_drone_id = client.mk_eve_drone(attrs={eve_use_attr_id: 25})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_options = [StatsOptionDroneBandwidth(), StatsOptionDroneBandwidth(affectors=consts.ApiCtlAffector.offline)]
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=api_options))
    assert api_stats.drone_bandwidth == [(approx(25), None), (approx(25), None)]
    assert api_module.update().state == consts.ApiModuleState.online


def test_not_requested(client, consts):
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=False))
    with check_no_field():
        api_stats.drone_bandwidth  # ruff:ignore[useless-expression]
