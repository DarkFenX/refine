"""
As of 2026-06-08, Dromi is a special case: it is the only fighter which applies attribute modifiers
with resistance attribute ID defined in a non-standard way (special attribute on fighter itself).
"""

from fw import approx
from fw.api import FitStatsOptions


def test_resistance(client, consts):
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_web_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_stasis_web_speed_penalty)
    eve_web_interim_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_stasis_web_speed_penalty_interim)
    eve_resist_ref_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_stasis_web_speed_resist_id)
    eve_resist_attr_id = client.mk_eve_attr()
    eve_boost_attr_id = client.mk_eve_attr()
    client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_size)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_resist_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_boost_attr_id,
        affectee_attr_id=eve_resist_attr_id)
    eve_web_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_stasis_web, cat_id=consts.EveEffCat.target)
    eve_resist_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_resist_mod])
    eve_dromi_id = client.mk_eve_item(
        attrs={
            eve_web_attr_id: -15,
            eve_web_interim_attr_id: 0,
            eve_resist_ref_attr_id: eve_resist_attr_id,
            eve_max_count_attr_id: 3},
        eff_ids=[eve_web_effect_id],
        defeff_id=eve_web_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_speed_attr_id: 1000, eve_resist_attr_id: 1})
    eve_booster_id = client.mk_eve_item(eff_ids=[eve_resist_effect_id], attrs={eve_boost_attr_id: -25})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_dromi = api_src_fit.add_fighter(type_id=eve_dromi_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_dromi.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(speed=True))
    assert api_tgt_fit_stats.speed.one() == approx(550)
    # Action
    api_booster = api_tgt_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(speed=True))
    assert api_tgt_fit_stats.speed.one() == approx(662.5)
    # Action
    api_booster.remove()
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(speed=True))
    assert api_tgt_fit_stats.speed.one() == approx(550)


def test_count(client, consts):
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_web_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_stasis_web_speed_penalty)
    eve_web_interim_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_stasis_web_speed_penalty_interim)
    client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_size)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_web_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_stasis_web, cat_id=consts.EveEffCat.target)
    eve_dromi_id = client.mk_eve_item(
        attrs={eve_web_attr_id: -15, eve_web_interim_attr_id: 0, eve_max_count_attr_id: 3},
        eff_ids=[eve_web_effect_id],
        defeff_id=eve_web_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_speed_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_dromi = api_src_fit.add_fighter(type_id=eve_dromi_id, state=consts.ApiMinionState.engaging, count=2)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_dromi.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(speed=True))
    assert api_tgt_fit_stats.speed.one() == approx(700)
    # Action
    api_src_dromi.change_fighter(count=4)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(speed=True))
    assert api_tgt_fit_stats.speed.one() == approx(400)


def test_attr_state(client, consts):
    # Check that even when ability/fighter are "disabled", web strength attribute value is still
    # multiplied
    eve_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_web_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_stasis_web_speed_penalty)
    eve_web_interim_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_stasis_web_speed_penalty_interim)
    client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_size)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_web_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_stasis_web, cat_id=consts.EveEffCat.target)
    eve_web_ability_id = client.mk_eve_abil(id_=consts.EveAbil.stasis_web)
    eve_dromi_id = client.mk_eve_fighter(
        attrs={eve_web_attr_id: -15, eve_web_interim_attr_id: 0, eve_max_count_attr_id: 3},
        eff_ids=[eve_web_effect_id],
        defeff_id=eve_web_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_web_ability_id)])
    eve_ship_id = client.mk_eve_ship(attrs={eve_speed_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_dromi = api_src_fit.add_fighter(
        type_id=eve_dromi_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_web_ability_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_dromi.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(speed=True))
    assert api_tgt_fit_stats.speed.one() == approx(550)
    assert api_src_dromi.update().attrs[eve_web_interim_attr_id].modified == approx(-45)
    # Action
    api_src_dromi.change_fighter(state=consts.ApiMinionState.in_bay)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(speed=True))
    assert api_tgt_fit_stats.speed.one() == approx(1000)
    assert api_src_dromi.update().attrs[eve_web_interim_attr_id].modified == approx(-45)
    # Action
    api_src_dromi.change_fighter(abilities={eve_web_ability_id: False})
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(speed=True))
    assert api_tgt_fit_stats.speed.one() == approx(1000)
    assert api_src_dromi.update().attrs[eve_web_interim_attr_id].modified == approx(-45)
    # Action
    api_src_dromi.change_fighter(state=consts.ApiMinionState.engaging)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(speed=True))
    assert api_tgt_fit_stats.speed.one() == approx(1000)
    assert api_src_dromi.update().attrs[eve_web_interim_attr_id].modified == approx(-45)
