"""
Hybrid (combat + ewar) drones are special, because they have 2 active effects which run
concurrently, even if only one of them is default. In the lib, this mechanic is specific to drones,
other items execute only default active effect upon activation.
"""

from fw import approx
from fw.api import FitStatsOptions


def test_neut(client, consts):
    eve_dmg_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.em_dmg)
    eve_dmg_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.therm_dmg)
    eve_dmg_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.kin_dmg)
    eve_dmg_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.expl_dmg)
    eve_dmg_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.dmg_mult)
    eve_dmg_cycle_time_attr_id = client.mk_eve_attr()
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_neut_cycle_time_attr_id = client.mk_eve_attr()
    eve_dmg_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.tgt_attack,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_dmg_cycle_time_attr_id)
    eve_neut_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.entity_energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_neut_cycle_time_attr_id)
    eve_drone_id = client.mk_eve_drone(
        attrs={
            eve_dmg_em_attr_id: 60, eve_dmg_therm_attr_id: 30,
            eve_dmg_kin_attr_id: 0, eve_dmg_expl_attr_id: 0,
            eve_dmg_mult_attr_id: 1.875, eve_dmg_cycle_time_attr_id: 4000,
            eve_neut_amount_attr_id: 25, eve_neut_cycle_time_attr_id: 6000},
        eff_ids=[eve_dmg_effect_id, eve_neut_effect_id],
        defeff_id=eve_dmg_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True, outgoing_nps=True))
    assert api_fit_stats.dmg.one().dps == [approx(28.125), approx(14.0625), 0, 0]
    assert api_fit_stats.outgoing_nps.one() == approx(4.166667)
    api_drone.update()
    assert api_drone.effects[eve_dmg_effect_id].running is True
    assert api_drone.effects[eve_dmg_effect_id].mode == consts.ApiEffMode.full_compliance
    assert api_drone.effects[eve_neut_effect_id].running is True
    assert api_drone.effects[eve_neut_effect_id].mode == consts.ApiEffMode.full_compliance
    # Action
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True, outgoing_nps=True))
    assert api_fit_stats.dmg.one().dps == [0, 0, 0, 0]
    assert api_fit_stats.outgoing_nps.one() == 0
    api_drone.update()
    assert api_drone.effects[eve_dmg_effect_id].running is False
    assert api_drone.effects[eve_dmg_effect_id].mode == consts.ApiEffMode.full_compliance
    assert api_drone.effects[eve_neut_effect_id].running is False
    assert api_drone.effects[eve_neut_effect_id].mode == consts.ApiEffMode.full_compliance
    # Action
    api_drone.change_drone(effect_modes={eve_neut_effect_id: consts.ApiEffMode.force_run})
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True, outgoing_nps=True))
    assert api_fit_stats.dmg.one().dps == [0, 0, 0, 0]
    assert api_fit_stats.outgoing_nps.one() == approx(4.166667)
    api_drone.update()
    assert api_drone.effects[eve_dmg_effect_id].running is False
    assert api_drone.effects[eve_dmg_effect_id].mode == consts.ApiEffMode.full_compliance
    assert api_drone.effects[eve_neut_effect_id].running is True
    assert api_drone.effects[eve_neut_effect_id].mode == consts.ApiEffMode.force_run
    # Action
    api_drone.change_drone(effect_modes={eve_dmg_effect_id: consts.ApiEffMode.force_run})
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True, outgoing_nps=True))
    assert api_fit_stats.dmg.one().dps == [approx(28.125), approx(14.0625), 0, 0]
    assert api_fit_stats.outgoing_nps.one() == approx(4.166667)
    api_drone.update()
    assert api_drone.effects[eve_dmg_effect_id].running is True
    assert api_drone.effects[eve_dmg_effect_id].mode == consts.ApiEffMode.force_run
    assert api_drone.effects[eve_neut_effect_id].running is True
    assert api_drone.effects[eve_neut_effect_id].mode == consts.ApiEffMode.force_run
