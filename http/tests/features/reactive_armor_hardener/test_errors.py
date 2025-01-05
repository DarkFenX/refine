from tests import approx
from tests.features.reactive_armor_hardener import make_eve_rah, make_eve_ship, setup_rah_basics


def test_no_attr_res_em(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts, attr_res_em=None)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant_attr_id = client.mk_eve_attr()
    eve_implant_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            dom=consts.EveModDom.ship,
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
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.85)
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
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant_attr_id = client.mk_eve_attr()
    eve_implant_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            dom=consts.EveModDom.ship,
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
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.85)
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
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant_attr_id = client.mk_eve_attr()
    eve_implant_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            dom=consts.EveModDom.ship,
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
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.85)
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
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant_attr_id = client.mk_eve_attr()
    eve_implant_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            dom=consts.EveModDom.ship,
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
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.85)
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
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85))
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    eve_implant_attr_id = client.mk_eve_attr()
    eve_implant_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.loc,
            dom=consts.EveModDom.ship,
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
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.85)
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
            eve_res_em_attr_id: 0.85,
            eve_res_therm_attr_id: 0.85,
            eve_res_kin_attr_id: 0.85,
            eve_res_expl_attr_id: 0.85,
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
            dom=consts.EveModDom.ship,
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
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_res_em_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_res_therm_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_res_kin_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_res_expl_attr_id].dogma == approx(0.85)
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
            eve_res_em_attr_id: 0.85,
            eve_res_therm_attr_id: 0.85,
            eve_res_kin_attr_id: 0.85,
            eve_res_expl_attr_id: 0.85,
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
            dom=consts.EveModDom.ship,
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
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_res_em_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_res_therm_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_res_kin_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_res_expl_attr_id].dogma == approx(0.85)
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
    pass


def test_res_zero_modified(client, consts):
    # Test case when all resistances of one of RAHs are zero
    pass


def test_shift_non_positive_base(client, consts):
    # Test case when shift amount of some RAHs is zero or less
    pass


def test_shift_non_positive_modified(client, consts):
    # Test case when shift amount of some RAHs is zero or less
    pass


def test_cycle_non_positive_base(client, consts):
    # Test case when cycle time of some RAHs is zero or less
    pass


def test_cycle_non_positive_modified(client, consts):
    # Test case when cycle time of some RAHs is zero or less
    pass
