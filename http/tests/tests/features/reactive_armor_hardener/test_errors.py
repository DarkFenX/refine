from tests import approx
from tests.tests.features.reactive_armor_hardener import make_eve_rah, make_eve_ship, setup_rah_basics


def test_no_attr_res_em(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts, attr_res_em=None)
    eve_rah_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.85000000001, 0.85000000001, 0.85000000001, 0.85000000001),
        shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant_attr_id = client.mk_eve_attr()
    eve_implant_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            loc=consts.EveModLoc.ship,
            op=consts.EveModOp.post_percent,
            affector_attr_id=eve_implant_attr_id,
            affectee_attr_id=attr_id)
        for attr_id in (
            eve_basic_info.res_therm_attr_id,
            eve_basic_info.res_kin_attr_id,
            eve_basic_info.res_expl_attr_id)]
    eve_implant_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_implant_mods)
    eve_implant_id = client.mk_eve_item(attrs={eve_implant_attr_id: -10}, eff_ids=[eve_implant_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification - check that results were not rounded as well
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.85000000001, accuracy=11)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.5525)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.6375)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.765)
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification - modified values should be exposed, despite having non-modified stored
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.765)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.765)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.765)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.49725)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.57375)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.6885)


def test_no_attr_res_therm(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts, attr_res_therm=None)
    eve_rah_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.85000000001, 0.85000000001, 0.85000000001, 0.85000000001),
        shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant_attr_id = client.mk_eve_attr()
    eve_implant_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            loc=consts.EveModLoc.ship,
            op=consts.EveModOp.post_percent,
            affector_attr_id=eve_implant_attr_id,
            affectee_attr_id=attr_id)
        for attr_id in (
            eve_basic_info.res_em_attr_id,
            eve_basic_info.res_kin_attr_id,
            eve_basic_info.res_expl_attr_id)]
    eve_implant_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_implant_mods)
    eve_implant_id = client.mk_eve_item(attrs={eve_implant_attr_id: -10}, eff_ids=[eve_implant_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification - check that results were not rounded as well
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.85000000001, accuracy=11)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.425)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.6375)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.765)
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification - modified values should be exposed, despite having non-modified stored
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.765)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.765)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.765)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.3825)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.57375)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.6885)


def test_no_attr_res_kin(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts, attr_res_kin=None)
    eve_rah_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.85000000001, 0.85000000001, 0.85000000001, 0.85000000001),
        shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant_attr_id = client.mk_eve_attr()
    eve_implant_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            loc=consts.EveModLoc.ship,
            op=consts.EveModOp.post_percent,
            affector_attr_id=eve_implant_attr_id,
            affectee_attr_id=attr_id)
        for attr_id in (
            eve_basic_info.res_em_attr_id,
            eve_basic_info.res_therm_attr_id,
            eve_basic_info.res_expl_attr_id)]
    eve_implant_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_implant_mods)
    eve_implant_id = client.mk_eve_item(attrs={eve_implant_attr_id: -10}, eff_ids=[eve_implant_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification - check that results were not rounded as well
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.85000000001, accuracy=11)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.425)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.5525)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.765)
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification - modified values should be exposed, despite having non-modified stored
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.765)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.765)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.765)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.3825)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.49725)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.6885)


def test_no_attr_res_expl(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts, attr_res_expl=None)
    eve_rah_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.85000000001, 0.85000000001, 0.85000000001, 0.85000000001),
        shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant_attr_id = client.mk_eve_attr()
    eve_implant_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            loc=consts.EveModLoc.ship,
            op=consts.EveModOp.post_percent,
            affector_attr_id=eve_implant_attr_id,
            affectee_attr_id=attr_id)
        for attr_id in (
            eve_basic_info.res_em_attr_id,
            eve_basic_info.res_therm_attr_id,
            eve_basic_info.res_kin_attr_id)]
    eve_implant_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_implant_mods)
    eve_implant_id = client.mk_eve_item(attrs={eve_implant_attr_id: -10}, eff_ids=[eve_implant_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification - check that results were not rounded as well
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.85000000001, accuracy=11)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.425)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.5525)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.6375)
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification - modified values should be exposed, despite having non-modified stored
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.765)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.765)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.765)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.3825)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.49725)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.57375)


def test_no_attr_shift(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts, attr_shift=None)
    eve_rah_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.85000000001, 0.85000000001, 0.85000000001, 0.85000000001))
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant_attr_id = client.mk_eve_attr()
    eve_implant_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            loc=consts.EveModLoc.ship,
            op=consts.EveModOp.post_percent,
            affector_attr_id=eve_implant_attr_id,
            affectee_attr_id=attr_id)
        for attr_id in (
            eve_basic_info.res_em_attr_id,
            eve_basic_info.res_therm_attr_id,
            eve_basic_info.res_kin_attr_id,
            eve_basic_info.res_expl_attr_id)]
    eve_implant_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_implant_mods)
    eve_implant_id = client.mk_eve_item(attrs={eve_implant_attr_id: -10}, eff_ids=[eve_implant_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification - check that results were not rounded as well
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.85000000001, accuracy=11)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.425)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.5525)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.6375)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.765)
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification - modified values should be exposed, despite having non-modified stored
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.765)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.765)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.765)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.765)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.3825)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.49725)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.57375)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.6885)


def test_no_attr_cycle_time(client, consts):
    # Repeat setup here, because generic setup is hard to customize for this specific case
    eve_res_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_max_dmg_resonance, def_val=1)
    eve_res_em_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_em_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_therm_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_therm_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_kin_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_kin_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_expl_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_expl_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_shift_attr_id = client.mk_eve_attr(id_=consts.EveAttr.resist_shift_amount)
    eve_cycle_time_attr_id = consts.EveAttr.duration
    eve_rah_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.adaptive_armor_hardener,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_rah_id = client.mk_eve_item(
        attrs={
            eve_res_em_attr_id: 0.85000000001,
            eve_res_therm_attr_id: 0.85000000001,
            eve_res_kin_attr_id: 0.85000000001,
            eve_res_expl_attr_id: 0.85000000001,
            eve_res_shift_attr_id: 6,
            eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_rah_effect_id],
        defeff_id=eve_rah_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={
        eve_res_em_attr_id: 0.5,
        eve_res_therm_attr_id: 0.65,
        eve_res_kin_attr_id: 0.75,
        eve_res_expl_attr_id: 0.9})
    eve_implant_attr_id = client.mk_eve_attr()
    eve_implant_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            loc=consts.EveModLoc.ship,
            op=consts.EveModOp.post_percent,
            affector_attr_id=eve_implant_attr_id,
            affectee_attr_id=attr_id)
        for attr_id in (
            eve_res_em_attr_id,
            eve_res_therm_attr_id,
            eve_res_kin_attr_id,
            eve_res_expl_attr_id)]
    eve_implant_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_implant_mods)
    eve_implant_id = client.mk_eve_item(attrs={eve_implant_attr_id: -10}, eff_ids=[eve_implant_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification - check that results were not rounded as well
    api_rah.update()
    assert api_rah.attrs[eve_res_em_attr_id].dogma == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_res_therm_attr_id].dogma == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_res_kin_attr_id].dogma == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_res_expl_attr_id].dogma == approx(0.85000000001, accuracy=11)
    api_ship.update()
    assert api_ship.attrs[eve_res_em_attr_id].dogma == approx(0.425)
    assert api_ship.attrs[eve_res_therm_attr_id].dogma == approx(0.5525)
    assert api_ship.attrs[eve_res_kin_attr_id].dogma == approx(0.6375)
    assert api_ship.attrs[eve_res_expl_attr_id].dogma == approx(0.765)
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification - modified values should be exposed, despite having non-modified stored
    api_rah.update()
    assert api_rah.attrs[eve_res_em_attr_id].dogma == approx(0.765)
    assert api_rah.attrs[eve_res_therm_attr_id].dogma == approx(0.765)
    assert api_rah.attrs[eve_res_kin_attr_id].dogma == approx(0.765)
    assert api_rah.attrs[eve_res_expl_attr_id].dogma == approx(0.765)
    api_ship.update()
    assert api_ship.attrs[eve_res_em_attr_id].dogma == approx(0.3825)
    assert api_ship.attrs[eve_res_therm_attr_id].dogma == approx(0.49725)
    assert api_ship.attrs[eve_res_kin_attr_id].dogma == approx(0.57375)
    assert api_ship.attrs[eve_res_expl_attr_id].dogma == approx(0.6885)


def test_no_cycle_time(client, consts):
    # Case when RAH effect does not refer effect duration attribute
    # Repeat setup here, because generic setup is hard to customize for this specific case
    eve_res_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_max_dmg_resonance, def_val=1)
    eve_res_em_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_em_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_therm_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_therm_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_kin_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_kin_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_expl_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_expl_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_shift_attr_id = client.mk_eve_attr(id_=consts.EveAttr.resist_shift_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.duration)
    eve_rah_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.adaptive_armor_hardener,
        cat_id=consts.EveEffCat.active)
    eve_rah_id = client.mk_eve_item(
        attrs={
            eve_res_em_attr_id: 0.85000000001,
            eve_res_therm_attr_id: 0.85000000001,
            eve_res_kin_attr_id: 0.85000000001,
            eve_res_expl_attr_id: 0.85000000001,
            eve_res_shift_attr_id: 6,
            eve_cycle_time_attr_id: 10000},
        eff_ids=[eve_rah_effect_id],
        defeff_id=eve_rah_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={
        eve_res_em_attr_id: 0.5,
        eve_res_therm_attr_id: 0.65,
        eve_res_kin_attr_id: 0.75,
        eve_res_expl_attr_id: 0.9})
    eve_implant_attr_id = client.mk_eve_attr()
    eve_implant_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            loc=consts.EveModLoc.ship,
            op=consts.EveModOp.post_percent,
            affector_attr_id=eve_implant_attr_id,
            affectee_attr_id=attr_id)
        for attr_id in (
            eve_res_em_attr_id,
            eve_res_therm_attr_id,
            eve_res_kin_attr_id,
            eve_res_expl_attr_id)]
    eve_implant_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_implant_mods)
    eve_implant_id = client.mk_eve_item(attrs={eve_implant_attr_id: -10}, eff_ids=[eve_implant_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification - check that results were not rounded as well
    api_rah.update()
    assert api_rah.attrs[eve_res_em_attr_id].dogma == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_res_therm_attr_id].dogma == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_res_kin_attr_id].dogma == approx(0.85000000001, accuracy=11)
    assert api_rah.attrs[eve_res_expl_attr_id].dogma == approx(0.85000000001, accuracy=11)
    api_ship.update()
    assert api_ship.attrs[eve_res_em_attr_id].dogma == approx(0.425)
    assert api_ship.attrs[eve_res_therm_attr_id].dogma == approx(0.5525)
    assert api_ship.attrs[eve_res_kin_attr_id].dogma == approx(0.6375)
    assert api_ship.attrs[eve_res_expl_attr_id].dogma == approx(0.765)
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification - modified values should be exposed, despite having non-modified stored
    api_rah.update()
    assert api_rah.attrs[eve_res_em_attr_id].dogma == approx(0.765)
    assert api_rah.attrs[eve_res_therm_attr_id].dogma == approx(0.765)
    assert api_rah.attrs[eve_res_kin_attr_id].dogma == approx(0.765)
    assert api_rah.attrs[eve_res_expl_attr_id].dogma == approx(0.765)
    api_ship.update()
    assert api_ship.attrs[eve_res_em_attr_id].dogma == approx(0.3825)
    assert api_ship.attrs[eve_res_therm_attr_id].dogma == approx(0.49725)
    assert api_ship.attrs[eve_res_kin_attr_id].dogma == approx(0.57375)
    assert api_ship.attrs[eve_res_expl_attr_id].dogma == approx(0.6885)


def test_res_zero_base(client, consts):
    # Test case when all resistances of one of RAHs are zero
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_rah1_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp1_id,
        resos=(1, 1, 1, 1),
        shift_amount=6)
    eve_rah2_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp2_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant_attr_id = client.mk_eve_attr()
    eve_implant_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp1_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_implant_attr_id,
        affectee_attr_id=eve_basic_info.res_em_attr_id)
    eve_implant_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_implant_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_implant_attr_id: -60}, eff_ids=[eve_implant_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah1_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah2_id, state=consts.ApiModuleState.active)
    # Verification - second RAH can adapt, despite first being unable to
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(1)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(1)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(1)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.925)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.82)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.655)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.5)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.60125)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.615)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.5895)
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification - since after modification total RAH resistances stop being 0 (even if it's just
    # one resistance), RAH sim can properly run again for all RAHs
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].base == approx(1)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.97)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].base == approx(1)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].base == approx(1)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.7975)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].base == approx(1)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.7825)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.97)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.91)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.805)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.715)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.4723543)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.509283)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.4967557)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.5218569)


def test_res_zero_modified(client, consts):
    # Test case when all resistances of one of RAHs are zero
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_rah1_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp1_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_rah2_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp2_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant1_attr_id = client.mk_eve_attr()
    eve_implant1_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc_grp,
            loc=consts.EveModLoc.ship,
            grp=eve_grp1_id,
            op=consts.EveModOp.pre_assign,
            affector_attr_id=eve_implant1_attr_id,
            affectee_attr_id=attr_id)
        for attr_id in (
            eve_basic_info.res_em_attr_id,
            eve_basic_info.res_therm_attr_id,
            eve_basic_info.res_kin_attr_id,
            eve_basic_info.res_expl_attr_id)]
    eve_implant1_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_implant1_mods)
    eve_implant1_id = client.mk_eve_item(attrs={eve_implant1_attr_id: 1}, eff_ids=[eve_implant1_effect_id])
    eve_implant2_attr_id = client.mk_eve_attr()
    eve_implant2_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp1_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_implant2_attr_id,
        affectee_attr_id=eve_basic_info.res_em_attr_id)
    eve_implant2_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_implant2_mod])
    eve_implant2_id = client.mk_eve_item(attrs={eve_implant2_attr_id: -60}, eff_ids=[eve_implant2_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah1_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah2_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.97)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.88)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.805)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.745)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.97)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.88)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.805)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.745)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.4723543)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.5123436)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.5014274)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.5219)
    # Action
    api_fit.add_implant(type_id=eve_implant1_id)
    # Verification - second RAH can adapt, despite first being unable to
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(1)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(1)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(1)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.925)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.82)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.655)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.5)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.60125)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.615)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.5895)
    # Action
    api_fit.add_implant(type_id=eve_implant2_id)
    # Verification - since after modification total RAH resistances stop being 0 (even if it's just
    # one resistance), RAH sim can properly run again for all RAHs
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.97)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.7975)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.7825)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.97)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.91)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.805)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.715)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.4723543)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.509283)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.4967557)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.5218569)


def test_shift_non_positive_base(client, consts):
    # Test case when shift amount of some RAHs is zero or less
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_rah1_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp1_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=-6)
    eve_rah2_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp1_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=0)
    eve_rah3_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp2_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant1_attr_id = client.mk_eve_attr()
    eve_implant1_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            loc=consts.EveModLoc.ship,
            op=consts.EveModOp.post_percent,
            affector_attr_id=eve_implant1_attr_id,
            affectee_attr_id=attr_id)
        for attr_id in (
            eve_basic_info.res_em_attr_id,
            eve_basic_info.res_therm_attr_id,
            eve_basic_info.res_kin_attr_id,
            eve_basic_info.res_expl_attr_id)]
    eve_implant1_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_implant1_mods)
    eve_implant1_id = client.mk_eve_item(attrs={eve_implant1_attr_id: -10}, eff_ids=[eve_implant1_effect_id])
    eve_implant2_attr_id = client.mk_eve_attr()
    eve_implant2_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp1_id,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_implant2_attr_id,
        affectee_attr_id=eve_basic_info.res_shift_attr_id)
    eve_implant2_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_implant2_mod])
    eve_implant2_id = client.mk_eve_item(attrs={eve_implant2_attr_id: 6}, eff_ids=[eve_implant2_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_fit.add_implant(type_id=eve_implant1_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah1_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah2_id, state=consts.ApiModuleState.active)
    api_rah3 = api_fit.add_module(type_id=eve_rah3_id, state=consts.ApiModuleState.active)
    # Verification - third RAH can adapt, despite first two being unable to
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_shift_attr_id].base == approx(-6)
    assert api_rah1.attrs[eve_basic_info.res_shift_attr_id].dogma == approx(-6)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.765)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.765)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.765)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.765)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_shift_attr_id].base == approx(0)
    assert api_rah2.attrs[eve_basic_info.res_shift_attr_id].dogma == approx(0)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.765)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.765)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.765)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.765)
    api_rah3.update()
    assert api_rah3.attrs[eve_basic_info.res_shift_attr_id].base == approx(6)
    assert api_rah3.attrs[eve_basic_info.res_shift_attr_id].dogma == approx(6)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.6775)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.5875)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.304377)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.3494064)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.3501266)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.3643384)
    # Action
    api_fit.add_implant(type_id=eve_implant2_id)
    # Verification - since after modification RAH shift becomes greater than zero, RAH sim can
    # properly run again for all RAHs
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_shift_attr_id].base == approx(-6)
    assert api_rah1.attrs[eve_basic_info.res_shift_attr_id].dogma == approx(6)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_shift_attr_id].base == approx(0)
    assert api_rah2.attrs[eve_basic_info.res_shift_attr_id].dogma == approx(6)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_rah3.update()
    assert api_rah3.attrs[eve_basic_info.res_shift_attr_id].base == approx(6)
    assert api_rah3.attrs[eve_basic_info.res_shift_attr_id].dogma == approx(6)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.3427137)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.3750061)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.3601336)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.3550695)


def test_shift_non_positive_modified(client, consts):
    # Test case when shift amount of some RAHs is zero or less
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_grp3_id = client.mk_eve_item_group()
    eve_rah1_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp1_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_rah2_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp2_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_rah3_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp3_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant1_attr_id = client.mk_eve_attr()
    eve_implant1_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            loc=consts.EveModLoc.ship,
            op=consts.EveModOp.post_percent,
            affector_attr_id=eve_implant1_attr_id,
            affectee_attr_id=attr_id)
        for attr_id in (
            eve_basic_info.res_em_attr_id,
            eve_basic_info.res_therm_attr_id,
            eve_basic_info.res_kin_attr_id,
            eve_basic_info.res_expl_attr_id)]
    eve_implant1_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_implant1_mods)
    eve_implant1_id = client.mk_eve_item(attrs={eve_implant1_attr_id: -10}, eff_ids=[eve_implant1_effect_id])
    eve_implant2_attr1_id = client.mk_eve_attr()
    eve_implant2_attr2_id = client.mk_eve_attr()
    eve_implant2_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp1_id,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_implant2_attr1_id,
        affectee_attr_id=eve_basic_info.res_shift_attr_id)
    eve_implant2_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp2_id,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_implant2_attr2_id,
        affectee_attr_id=eve_basic_info.res_shift_attr_id)
    eve_implant2_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.passive,
        mod_info=[eve_implant2_mod1, eve_implant2_mod2])
    eve_implant2_id = client.mk_eve_item(
        attrs={eve_implant2_attr1_id: -6, eve_implant2_attr2_id: 0},
        eff_ids=[eve_implant2_effect_id])
    eve_implant3_attr_id = client.mk_eve_attr()
    eve_implant3_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc_grp,
            loc=consts.EveModLoc.ship,
            grp=grp_id,
            op=consts.EveModOp.post_assign,
            affector_attr_id=eve_implant3_attr_id,
            affectee_attr_id=eve_basic_info.res_shift_attr_id)
        for grp_id in (eve_grp1_id, eve_grp2_id)]
    eve_implant3_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_implant3_mods)
    eve_implant3_id = client.mk_eve_item(attrs={eve_implant3_attr_id: 6}, eff_ids=[eve_implant3_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_fit.add_implant(type_id=eve_implant1_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah1_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah2_id, state=consts.ApiModuleState.active)
    api_rah3 = api_fit.add_module(type_id=eve_rah3_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_shift_attr_id].base == approx(6)
    assert api_rah1.attrs[eve_basic_info.res_shift_attr_id].dogma == approx(6)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_shift_attr_id].base == approx(6)
    assert api_rah2.attrs[eve_basic_info.res_shift_attr_id].dogma == approx(6)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_rah3.update()
    assert api_rah3.attrs[eve_basic_info.res_shift_attr_id].base == approx(6)
    assert api_rah3.attrs[eve_basic_info.res_shift_attr_id].dogma == approx(6)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.3427137)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.3750061)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.3601336)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.3550695)
    # Action
    api_fit.add_implant(type_id=eve_implant2_id)
    # Verification - third RAH can adapt, despite first two being unable to
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_shift_attr_id].base == approx(6)
    assert api_rah1.attrs[eve_basic_info.res_shift_attr_id].dogma == approx(-6)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.765)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.765)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.765)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.765)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_shift_attr_id].base == approx(6)
    assert api_rah2.attrs[eve_basic_info.res_shift_attr_id].dogma == approx(0)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.765)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.765)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.765)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.765)
    api_rah3.update()
    assert api_rah3.attrs[eve_basic_info.res_shift_attr_id].base == approx(6)
    assert api_rah3.attrs[eve_basic_info.res_shift_attr_id].dogma == approx(6)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.6775)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.5875)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.304377)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.3494064)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.3501266)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.3643384)
    # Action
    api_fit.add_implant(type_id=eve_implant3_id)
    # Verification - since after modification RAH shift becomes greater than zero, RAH sim can
    # properly run again for all RAHs
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.res_shift_attr_id].base == approx(6)
    assert api_rah1.attrs[eve_basic_info.res_shift_attr_id].dogma == approx(6)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.res_shift_attr_id].base == approx(6)
    assert api_rah2.attrs[eve_basic_info.res_shift_attr_id].dogma == approx(6)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_rah3.update()
    assert api_rah3.attrs[eve_basic_info.res_shift_attr_id].base == approx(6)
    assert api_rah3.attrs[eve_basic_info.res_shift_attr_id].dogma == approx(6)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.3427137)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.3750061)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.3601336)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.3550695)


def test_cycle_non_positive_base(client, consts):
    # Test case when cycle time of some RAHs is zero or less
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_rah1_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp1_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        cycle_time=-10000)
    eve_rah2_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp1_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        cycle_time=0)
    eve_rah3_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp2_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        cycle_time=10000)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant1_attr_id = client.mk_eve_attr()
    eve_implant1_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            loc=consts.EveModLoc.ship,
            op=consts.EveModOp.post_percent,
            affector_attr_id=eve_implant1_attr_id,
            affectee_attr_id=attr_id)
        for attr_id in (
            eve_basic_info.res_em_attr_id,
            eve_basic_info.res_therm_attr_id,
            eve_basic_info.res_kin_attr_id,
            eve_basic_info.res_expl_attr_id)]
    eve_implant1_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_implant1_mods)
    eve_implant1_id = client.mk_eve_item(attrs={eve_implant1_attr_id: -10}, eff_ids=[eve_implant1_effect_id])
    eve_implant2_attr_id = client.mk_eve_attr()
    eve_implant2_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp1_id,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_implant2_attr_id,
        affectee_attr_id=eve_basic_info.cycle_time_attr_id)
    eve_implant2_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_implant2_mod])
    eve_implant2_id = client.mk_eve_item(attrs={eve_implant2_attr_id: 10000}, eff_ids=[eve_implant2_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_fit.add_implant(type_id=eve_implant1_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah1_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah2_id, state=consts.ApiModuleState.active)
    api_rah3 = api_fit.add_module(type_id=eve_rah3_id, state=consts.ApiModuleState.active)
    # Verification - third RAH can adapt, despite first two being unable to
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.cycle_time_attr_id].base == approx(-10000)
    assert api_rah1.attrs[eve_basic_info.cycle_time_attr_id].dogma == approx(-10000)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.765)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.765)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.765)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.765)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.cycle_time_attr_id].base == approx(0)
    assert api_rah2.attrs[eve_basic_info.cycle_time_attr_id].dogma == approx(0)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.765)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.765)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.765)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.765)
    api_rah3.update()
    assert api_rah3.attrs[eve_basic_info.cycle_time_attr_id].base == approx(10000)
    assert api_rah3.attrs[eve_basic_info.cycle_time_attr_id].dogma == approx(10000)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.6775)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.5875)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.304377)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.3494064)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.3501266)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.3643384)
    # Action
    api_fit.add_implant(type_id=eve_implant2_id)
    # Verification - since after modification RAH cycle time becomes greater than zero, RAH sim can
    # properly run again for all RAHs
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.cycle_time_attr_id].base == approx(-10000)
    assert api_rah1.attrs[eve_basic_info.cycle_time_attr_id].dogma == approx(10000)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.cycle_time_attr_id].base == approx(0)
    assert api_rah2.attrs[eve_basic_info.cycle_time_attr_id].dogma == approx(10000)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_rah3.update()
    assert api_rah3.attrs[eve_basic_info.cycle_time_attr_id].base == approx(10000)
    assert api_rah3.attrs[eve_basic_info.cycle_time_attr_id].dogma == approx(10000)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.3427137)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.3750061)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.3601336)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.3550695)


def test_cycle_non_positive_modified(client, consts):
    # Test case when cycle time of some RAHs is zero or less
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_grp3_id = client.mk_eve_item_group()
    eve_rah1_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp1_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        cycle_time=10000)
    eve_rah2_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp2_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        cycle_time=10000)
    eve_rah3_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        grp_id=eve_grp3_id,
        resos=(0.85, 0.85, 0.85, 0.85),
        cycle_time=10000)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant1_attr_id = client.mk_eve_attr()
    eve_implant1_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            loc=consts.EveModLoc.ship,
            op=consts.EveModOp.post_percent,
            affector_attr_id=eve_implant1_attr_id,
            affectee_attr_id=attr_id)
        for attr_id in (
            eve_basic_info.res_em_attr_id,
            eve_basic_info.res_therm_attr_id,
            eve_basic_info.res_kin_attr_id,
            eve_basic_info.res_expl_attr_id)]
    eve_implant1_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_implant1_mods)
    eve_implant1_id = client.mk_eve_item(attrs={eve_implant1_attr_id: -10}, eff_ids=[eve_implant1_effect_id])
    eve_implant2_attr1_id = client.mk_eve_attr()
    eve_implant2_attr2_id = client.mk_eve_attr()
    eve_implant2_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp1_id,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_implant2_attr1_id,
        affectee_attr_id=eve_basic_info.cycle_time_attr_id)
    eve_implant2_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp2_id,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_implant2_attr2_id,
        affectee_attr_id=eve_basic_info.cycle_time_attr_id)
    eve_implant2_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.passive,
        mod_info=[eve_implant2_mod1, eve_implant2_mod2])
    eve_implant2_id = client.mk_eve_item(
        attrs={eve_implant2_attr1_id: -10000, eve_implant2_attr2_id: 0},
        eff_ids=[eve_implant2_effect_id])
    eve_implant3_attr_id = client.mk_eve_attr()
    eve_implant3_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc_grp,
            loc=consts.EveModLoc.ship,
            grp=grp_id,
            op=consts.EveModOp.post_assign,
            affector_attr_id=eve_implant3_attr_id,
            affectee_attr_id=eve_basic_info.cycle_time_attr_id)
        for grp_id in (eve_grp1_id, eve_grp2_id)]
    eve_implant3_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=eve_implant3_mods)
    eve_implant3_id = client.mk_eve_item(attrs={eve_implant3_attr_id: 10000}, eff_ids=[eve_implant3_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_fit.add_implant(type_id=eve_implant1_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah1 = api_fit.add_module(type_id=eve_rah1_id, state=consts.ApiModuleState.active)
    api_rah2 = api_fit.add_module(type_id=eve_rah2_id, state=consts.ApiModuleState.active)
    api_rah3 = api_fit.add_module(type_id=eve_rah3_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.cycle_time_attr_id].base == approx(10000)
    assert api_rah1.attrs[eve_basic_info.cycle_time_attr_id].dogma == approx(10000)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.cycle_time_attr_id].base == approx(10000)
    assert api_rah2.attrs[eve_basic_info.cycle_time_attr_id].dogma == approx(10000)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_rah3.update()
    assert api_rah3.attrs[eve_basic_info.cycle_time_attr_id].base == approx(10000)
    assert api_rah3.attrs[eve_basic_info.cycle_time_attr_id].dogma == approx(10000)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.3427137)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.3750061)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.3601336)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.3550695)
    # Action
    api_fit.add_implant(type_id=eve_implant2_id)
    # Verification - third RAH can adapt, despite first two being unable to
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.cycle_time_attr_id].base == approx(10000)
    assert api_rah1.attrs[eve_basic_info.cycle_time_attr_id].dogma == approx(-10000)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.765)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.765)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.765)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.765)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.cycle_time_attr_id].base == approx(10000)
    assert api_rah2.attrs[eve_basic_info.cycle_time_attr_id].dogma == approx(0)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.765)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.765)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.765)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.765)
    api_rah3.update()
    assert api_rah3.attrs[eve_basic_info.cycle_time_attr_id].base == approx(10000)
    assert api_rah3.attrs[eve_basic_info.cycle_time_attr_id].dogma == approx(10000)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.6775)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.5875)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.304377)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.3494064)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.3501266)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.3643384)
    # Action
    api_fit.add_implant(type_id=eve_implant3_id)
    # Verification - since after modification RAH cycle time becomes greater than zero, RAH sim can
    # properly run again for all RAHs
    api_rah1.update()
    assert api_rah1.attrs[eve_basic_info.cycle_time_attr_id].base == approx(10000)
    assert api_rah1.attrs[eve_basic_info.cycle_time_attr_id].dogma == approx(10000)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah1.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_rah2.update()
    assert api_rah2.attrs[eve_basic_info.cycle_time_attr_id].base == approx(10000)
    assert api_rah2.attrs[eve_basic_info.cycle_time_attr_id].dogma == approx(10000)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah2.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_rah3.update()
    assert api_rah3.attrs[eve_basic_info.cycle_time_attr_id].base == approx(10000)
    assert api_rah3.attrs[eve_basic_info.cycle_time_attr_id].dogma == approx(10000)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.795)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].base == approx(0.85)
    assert api_rah3.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.675)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.3427137)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.3750061)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.3601336)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.3550695)
