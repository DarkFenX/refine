from tests import approx, check_no_field
from tests.features.reactive_armor_hardener import make_eve_rah, make_eve_ship, setup_rah_basics


def test_res_changed_em(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    eve_res_boost_attr_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_res_boost_attr_id,
        affectee_attr_id=eve_basic_info.res_em_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_res_boost_attr_id: -30}, eff_ids=[eve_rig_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.94)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.76)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.76)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.94)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.47)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.494)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.4484)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.4794)
    # Action
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.715)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.79)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.895)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.35)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.46475)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.4661)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.45645)


def test_res_changed_therm(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    eve_res_boost_attr_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_res_boost_attr_id,
        affectee_attr_id=eve_basic_info.res_therm_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_res_boost_attr_id: -30}, eff_ids=[eve_rig_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.94)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.76)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.76)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.94)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.47)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.494)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.4484)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.4794)
    # Action
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.88)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.94)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.76)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.82)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.44)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.4277)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.4484)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.4182)


def test_res_changed_kin(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    eve_res_boost_attr_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_res_boost_attr_id,
        affectee_attr_id=eve_basic_info.res_kin_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_res_boost_attr_id: -30}, eff_ids=[eve_rig_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.94)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.76)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.76)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.94)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.47)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.494)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.4484)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.4794)
    # Action
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.88)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.685)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(1)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.835)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.44)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.44525)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.413)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.42585)


def test_res_changed_expl(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    eve_res_boost_attr_id = client.mk_eve_attr()
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_res_boost_attr_id,
        affectee_attr_id=eve_basic_info.res_expl_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_res_boost_attr_id: -30}, eff_ids=[eve_rig_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.94)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.76)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.76)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.94)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.47)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.494)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.4484)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.4794)
    # Action
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.925)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.715)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.76)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(1)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.4625)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.46475)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.4484)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.357)


def test_switch(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship1_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    eve_ship2_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.94)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.76)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.76)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.94)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.47)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.494)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.4484)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.4794)
    # Action
    api_ship = api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
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


def test_unloaded(client, consts):
    eve_basic_info = setup_rah_basics(client=client, consts=consts)
    eve_rah_id = make_eve_rah(client=client, basic_info=eve_basic_info, resos=(0.85, 0.85, 0.85, 0.85), shift_amount=6)
    eve_ship1_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.59, 0.51))
    eve_ship2_id = client.alloc_item_id()
    eve_ship3_id = make_eve_ship(client=client, basic_info=eve_basic_info, resos=(0.5, 0.65, 0.75, 0.9))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit(rah_incoming_dmg=(1, 1, 1, 1))
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    api_rah = api_fit.add_mod(type_id=eve_rah_id, state=consts.ApiState.active)
    # Verification
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.94)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.76)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.76)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.94)
    api_ship.update()
    assert api_ship.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.47)
    assert api_ship.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.494)
    assert api_ship.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.4484)
    assert api_ship.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.4794)
    # Action
    api_ship = api_fit.set_ship(type_id=eve_ship2_id)
    # Verification - unloaded ship should reset attributes and prevent sim from running
    api_rah.update()
    assert api_rah.attrs[eve_basic_info.res_em_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_therm_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_kin_attr_id].dogma == approx(0.85)
    assert api_rah.attrs[eve_basic_info.res_expl_attr_id].dogma == approx(0.85)
    api_ship.update()
    with check_no_field():
        api_ship.attrs  # pylint: disable=W0104
    # Action
    api_ship = api_fit.set_ship(type_id=eve_ship3_id)
    # Verification
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
