from tests import approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions


def test_injector_mixed(client, consts):
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
        attrs={eve_use_amount_attr_id: 225, eve_cycle_time_attr_id: 10000},
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
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 225, eve_regen_attr_id: 90000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_user_id, state=consts.ApiModuleState.active)
    api_fit.add_module(type_id=eve_injector_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    # Verification - at t0 ship has just enough cap for a single use of the module, then just before
    # next cycle at t10 injector is used, and finally at t20 injector can't cover module needs, so
    # it runs out
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_sim=True))
    assert api_fit_stats.cap_sim.one() == {'time': approx(20)}
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_sim=True))
    assert api_ship_stats.cap_sim.one() == {'time': approx(20)}
