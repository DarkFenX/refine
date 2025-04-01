from tests import approx
from tests.tests.features.reactive_armor_hardener import make_eve_rah, make_eve_ship, setup_rah_basics


def test_normal(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
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
    eve_implant_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.implant,
        attrs={eve_implant_attr_id: -10},
        eff_ids=[eve_implant_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(1, 1, 1, 1))
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.675)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.795)
    api_rah_mod_em = api_rah.mods.find_by_affector_item(
        affectee_attr_id=eve_basic_info.res_em_attr_id,
        affector_item_id=api_implant.id).one()
    assert api_rah_mod_em.op == consts.ApiModOp.post_percent
    assert api_rah_mod_em.initial_val == approx(-10)
    assert api_rah_mod_em.stacking_mult is None
    assert api_rah_mod_em.applied_val == approx(-10)
    assert api_rah_mod_em.affectors.one().item_id == api_implant.id
    assert api_rah_mod_em.affectors.one().attr_id == eve_implant_attr_id
    api_rah_mod_therm = api_rah.mods.find_by_affector_item(
        affectee_attr_id=eve_basic_info.res_therm_attr_id,
        affector_item_id=api_implant.id).one()
    assert api_rah_mod_therm.op == consts.ApiModOp.post_percent
    assert api_rah_mod_therm.initial_val == approx(-10)
    assert api_rah_mod_therm.stacking_mult is None
    assert api_rah_mod_therm.applied_val == approx(-10)
    assert api_rah_mod_therm.affectors.one().item_id == api_implant.id
    assert api_rah_mod_therm.affectors.one().attr_id == eve_implant_attr_id
    api_rah_mod_kin = api_rah.mods.find_by_affector_item(
        affectee_attr_id=eve_basic_info.res_kin_attr_id,
        affector_item_id=api_implant.id).one()
    assert api_rah_mod_kin.op == consts.ApiModOp.post_percent
    assert api_rah_mod_kin.initial_val == approx(-10)
    assert api_rah_mod_kin.stacking_mult is None
    assert api_rah_mod_kin.applied_val == approx(-10)
    assert api_rah_mod_kin.affectors.one().item_id == api_implant.id
    assert api_rah_mod_kin.affectors.one().attr_id == eve_implant_attr_id
    api_rah_mod_expl = api_rah.mods.find_by_affector_item(
        affectee_attr_id=eve_basic_info.res_expl_attr_id,
        affector_item_id=api_implant.id).one()
    assert api_rah_mod_expl.op == consts.ApiModOp.post_percent
    assert api_rah_mod_expl.initial_val == approx(-10)
    assert api_rah_mod_expl.stacking_mult is None
    assert api_rah_mod_expl.applied_val == approx(-10)
    assert api_rah_mod_expl.affectors.one().item_id == api_implant.id
    assert api_rah_mod_expl.affectors.one().attr_id == eve_implant_attr_id
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.4275)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.43875)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.43365)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.40545)
    api_ship_mod_em = api_ship.mods.find_by_affector_item(
        affectee_attr_id=eve_basic_info.res_em_attr_id,
        affector_item_id=api_rah.id).one()
    assert api_ship_mod_em.op == consts.ApiModOp.pre_mul
    assert api_ship_mod_em.initial_val == approx(0.855)
    assert api_ship_mod_em.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_ship_mod_em.applied_val == approx(0.855)
    assert api_ship_mod_em.affectors.one().item_id == api_rah.id
    assert api_ship_mod_em.affectors.one().attr_id == eve_basic_info.res_em_attr_id
    api_ship_mod_therm = api_ship.mods.find_by_affector_item(
        affectee_attr_id=eve_basic_info.res_therm_attr_id,
        affector_item_id=api_rah.id).one()
    assert api_ship_mod_therm.op == consts.ApiModOp.pre_mul
    assert api_ship_mod_therm.initial_val == approx(0.675)
    assert api_ship_mod_therm.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_ship_mod_therm.applied_val == approx(0.675)
    assert api_ship_mod_therm.affectors.one().item_id == api_rah.id
    assert api_ship_mod_therm.affectors.one().attr_id == eve_basic_info.res_therm_attr_id
    api_ship_mod_kin = api_ship.mods.find_by_affector_item(
        affectee_attr_id=eve_basic_info.res_kin_attr_id,
        affector_item_id=api_rah.id).one()
    assert api_ship_mod_kin.op == consts.ApiModOp.pre_mul
    assert api_ship_mod_kin.initial_val == approx(0.735)
    assert api_ship_mod_kin.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_ship_mod_kin.applied_val == approx(0.735)
    assert api_ship_mod_kin.affectors.one().item_id == api_rah.id
    assert api_ship_mod_kin.affectors.one().attr_id == eve_basic_info.res_kin_attr_id
    api_ship_mod_expl = api_ship.mods.find_by_affector_item(
        affectee_attr_id=eve_basic_info.res_expl_attr_id,
        affector_item_id=api_rah.id).one()
    assert api_ship_mod_expl.op == consts.ApiModOp.pre_mul
    assert api_ship_mod_expl.initial_val == approx(0.795)
    assert api_ship_mod_expl.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_ship_mod_expl.applied_val == approx(0.795)
    assert api_ship_mod_expl.affectors.one().item_id == api_rah.id
    assert api_ship_mod_expl.affectors.one().attr_id == eve_basic_info.res_expl_attr_id


def test_incoming(client, consts):
    # Check that incoming modifications are not hidden by RAH sim
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(0.85, 0.85, 0.85, 0.85),
        shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
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
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.855)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.675)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.735)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.795)
    api_em_mod = api_rah.mods[eve_basic_info.res_em_attr_id].one()
    assert api_em_mod.op == consts.ApiModOp.post_percent
    assert api_em_mod.initial_val == approx(-10)
    assert api_em_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_em_mod.applied_val == approx(-10)
    assert api_em_mod.affectors.one().item_id == api_implant.id
    assert api_em_mod.affectors.one().attr_id == eve_implant_attr_id
    api_therm_mod = api_rah.mods[eve_basic_info.res_therm_attr_id].one()
    assert api_therm_mod.op == consts.ApiModOp.post_percent
    assert api_therm_mod.initial_val == approx(-10)
    assert api_therm_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_therm_mod.applied_val == approx(-10)
    assert api_therm_mod.affectors.one().item_id == api_implant.id
    assert api_therm_mod.affectors.one().attr_id == eve_implant_attr_id
    api_kin_mod = api_rah.mods[eve_basic_info.res_kin_attr_id].one()
    assert api_kin_mod.op == consts.ApiModOp.post_percent
    assert api_kin_mod.initial_val == approx(-10)
    assert api_kin_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_kin_mod.applied_val == approx(-10)
    assert api_kin_mod.affectors.one().item_id == api_implant.id
    assert api_kin_mod.affectors.one().attr_id == eve_implant_attr_id
    api_expl_mod = api_rah.mods[eve_basic_info.res_expl_attr_id].one()
    assert api_expl_mod.op == consts.ApiModOp.post_percent
    assert api_expl_mod.initial_val == approx(-10)
    assert api_expl_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_expl_mod.applied_val == approx(-10)
    assert api_expl_mod.affectors.one().item_id == api_implant.id
    assert api_expl_mod.affectors.one().attr_id == eve_implant_attr_id


def test_outgoing_insignificance(client, consts):
    # Check that outgoing modifications use adapted values for insignificance testing
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(
        client=client,
        basic_info=eve_basic_info,
        resos=(1, 1, 1, 0),
        shift_amount=10)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    eve_implant_attr1_id = client.mk_eve_attr()
    eve_implant_attr2_id = client.mk_eve_attr()
    eve_implant_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_implant_attr1_id,
        affectee_attr_id=eve_basic_info.res_kin_attr_id)
    eve_implant_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_implant_attr2_id,
        affectee_attr_id=eve_basic_info.res_expl_attr_id)
    eve_implant_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.passive,
        mod_info=[eve_implant_mod1, eve_implant_mod2])
    eve_implant_id = client.mk_eve_item(
        attrs={eve_implant_attr1_id: 0, eve_implant_attr2_id: 1},
        eff_ids=[eve_implant_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dps=(0, 1, 0, 0))
    api_fit.add_implant(type_id=eve_implant_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_module(type_id=eve_rah_id, state=consts.ApiModuleState.active)
    # Verification - only thermal resistance modification is significant, since it multiplies by 0,
    # while other 3 multiply by 1 (even those with base value of 0, and modified value of 0).
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(1)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.5)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.59)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.51)
    assert len(api_ship.mods) == 1
    api_mod = api_ship.mods[eve_basic_info.res_therm_attr_id].one()
    assert api_mod.op == consts.ApiModOp.pre_mul
    assert api_mod.initial_val == approx(0)
    assert api_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_mod.applied_val == approx(0)
    assert api_mod.affectors.one().item_id == api_rah.id
    assert api_mod.affectors.one().attr_id == eve_basic_info.res_therm_attr_id
