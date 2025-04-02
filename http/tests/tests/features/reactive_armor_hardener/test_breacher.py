"""
As of 2025-02-07, breacher damage dealt to armor counted as EM damage for RAH purposes. Here, we
test all the interactions between breacher pods and RAH.
"""

from tests import approx
from tests.tests.features.reactive_armor_hardener import make_eve_rah, make_eve_ship, setup_rah_basics


def test_damage_kind(client, consts):
    # Check that breacher pods are considered as EM damage
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9), hps=(1, 1, 1))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(0, 0, 0, 0, (1, 0.01)))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.4)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(1)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.2)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.65)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.75)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.9)
    # Action
    api_fit.change(rah_incoming_dps=(1, 0, 0, 0, (1, 0.01)))
    # Verification - mixing up with EM damage shouldn't change results
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.4)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(1)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.2)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.65)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.75)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.9)


def test_absolute_relative_min(client, consts):
    # Check that minimal damage value is taken
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9), hps=(0, 100, 0))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(0, 5, 5, 5, (3, 0.1)))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification - due to absolute value, breacher DPS is limited to 3
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.97)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.925)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.82)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.685)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.485)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.60125)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.615)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.6165)
    # Action
    api_fit.change(rah_incoming_dps=(0, 5, 5, 5, (0, 0.1)))
    # Verification - due to absolute value, breacher DPS is limited to 0
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.925)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.82)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.655)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.5)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.60125)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.615)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.5895)
    # Action
    api_fit.change(rah_incoming_dps=(0, 5, 5, 5, (10, 0.1)))
    # Verification - here both relative and absolute limit breacher DPS to 10
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.5575)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.8425)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.27875)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.65)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.75)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.75825)
    # Action
    api_fit.change(rah_incoming_dps=(0, 5, 5, 5, (10, 0.03)))
    # Verification - due to relative value, breacher DPS is limited to 3
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.97)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.925)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.82)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.685)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.485)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.60125)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.615)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.6165)
    # Action
    api_fit.change(rah_incoming_dps=(0, 5, 5, 5, (10, 0)))
    # Verification - due to relative value, breacher DPS is limited to 0
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.925)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.82)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.655)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.5)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.60125)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.615)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.5895)


def test_ignore_resists(client, consts):
    # Check that breacher damage ignores ship resists
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9), hps=(0, 100, 0))
    eve_boost_attr_id = client.mk_eve_attr()
    eve_em_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_boost_attr_id,
        affectee_attr_id=consts.EveAttr.armor_em_dmg_resonance)
    eve_em_effect_id = client.mk_eve_effect(mod_info=[eve_em_mod])
    eve_em_rig_id = client.mk_eve_item(attrs={eve_boost_attr_id: -90}, eff_ids=[eve_em_effect_id])
    eve_therm_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_boost_attr_id,
        affectee_attr_id=consts.EveAttr.armor_therm_dmg_resonance)
    eve_therm_effect_id = client.mk_eve_effect(mod_info=[eve_therm_mod])
    eve_therm_rig_id = client.mk_eve_item(attrs={eve_boost_attr_id: -90}, eff_ids=[eve_therm_effect_id])
    eve_kin_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_boost_attr_id,
        affectee_attr_id=consts.EveAttr.armor_kin_dmg_resonance)
    eve_kin_effect_id = client.mk_eve_effect(mod_info=[eve_kin_mod])
    eve_kin_rig_id = client.mk_eve_item(attrs={eve_boost_attr_id: -90}, eff_ids=[eve_kin_effect_id])
    eve_expl_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_boost_attr_id,
        affectee_attr_id=consts.EveAttr.armor_expl_dmg_resonance)
    eve_expl_effect_id = client.mk_eve_effect(mod_info=[eve_expl_mod])
    eve_expl_rig_id = client.mk_eve_item(attrs={eve_boost_attr_id: -90}, eff_ids=[eve_expl_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(0, 15, 15, 15, (10, 0.1)))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.745)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(1.0)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.91)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.745)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.3725)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.65)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.6825)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.6705)
    # Action
    api_em_rig = api_fit.add_rig(type_id=eve_em_rig_id)
    # Verification - changed EM resist doesn't change RAH behavior
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.745)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(1.0)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.91)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.745)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.03725)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.65)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.6825)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.6705)
    # Action
    api_em_rig.remove()
    api_fit.change(rah_incoming_dps=(5, 0, 15, 15, (10, 0.1)))
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.5575)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(1.0)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(1.0)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.8425)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.27875)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.65)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.75)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.75825)
    # Action
    api_therm_rig = api_fit.add_rig(type_id=eve_therm_rig_id)
    # Verification - changed thermal resist doesn't change RAH behavior
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.5575)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(1.0)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(1.0)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.8425)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.27875)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.065)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.75)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.75825)
    # Action
    api_therm_rig.remove()
    api_fit.change(rah_incoming_dps=(5, 15, 0, 15, (10, 0.1)))
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.655)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(1.0)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(1.0)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.745)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.3275)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.65)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.75)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.6705)
    # Action
    api_kin_rig = api_fit.add_rig(type_id=eve_kin_rig_id)
    # Verification - changed kinetic resist doesn't change RAH behavior
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.655)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(1.0)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(1.0)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.745)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.3275)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.65)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.075)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.6705)
    # Action
    api_kin_rig.remove()
    api_fit.change(rah_incoming_dps=(5, 15, 15, 0, (10, 0.1)))
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.5125)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(1.0)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.8875)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(1)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.25625)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.65)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.665625)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.9)
    # Action
    api_fit.add_rig(type_id=eve_expl_rig_id)
    # Verification - changed explosive resist doesn't change RAH behavior
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.5125)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(1.0)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.8875)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(1)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.25625)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.65)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.665625)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.09)
