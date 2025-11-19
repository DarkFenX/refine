from tests import approx


def test_add_sw_fit_item_remove_sw_item_fit(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_sw_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_drone_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 100}, srqs={eve_skill_id: 1})
    eve_char_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_sw_effect.remove()
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_drone.remove()
    api_drone.update(status_code=404)
    api_fit.remove()


def test_add_fit_sw_item_remove_item_sw_fit(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_sw_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_drone_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 100}, srqs={eve_skill_id: 1})
    eve_char_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_drone.remove()
    api_drone.update(status_code=404)
    api_sw_effect.remove()
    api_fit.remove()


def test_add_fit_item_sw_remove_fit_sw(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_sw_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_drone_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 100}, srqs={eve_skill_id: 1})
    eve_char_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_fit.remove()
    api_drone.update(status_code=404)
    api_sw_effect.remove()


def test_add_sw_fit_item_state_remove_state_item_fit_sw(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_sw_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_drone_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 100}, srqs={eve_skill_id: 1})
    eve_char_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_sw_effect_id, state=False)
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_sw_effect.change_sw_effect(state=True)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_sw_effect.change_sw_effect(state=False)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_drone.remove()
    api_drone.update(status_code=404)
    api_fit.remove()
    api_sw_effect.remove()
