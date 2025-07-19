from tests import approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions


def test_state(client, consts):
    eve_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.em_dmg)
    eve_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.therm_dmg)
    eve_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.kin_dmg)
    eve_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.expl_dmg)
    eve_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.dmg_mult)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_reload_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.projectile_fired,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_mult_attr_id: 45,
            eve_capacity_attr_id: 0.25,
            eve_charge_rate_attr_id: 1,
            eve_cycle_time_attr_id: 8000,
            eve_reload_time_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_therm_attr_id: 23, eve_kin_attr_id: 4.6, eve_volume_attr_id: 0.0125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, approx(129.375), approx(25.875), 0]
    assert api_fit_stats.volley.one() == [0, approx(1035), approx(207), 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [0, approx(129.375), approx(25.875), 0]
    assert api_module_stats.volley.one() == [0, approx(1035), approx(207), 0]
