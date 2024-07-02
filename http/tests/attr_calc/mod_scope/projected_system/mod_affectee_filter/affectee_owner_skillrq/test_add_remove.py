from pytest import approx


def test_add_pe_item_proj_remove_state_proj_fit(client, consts):
    eve_skill = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.char,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_drone = client.mk_eve_item(attrs={eve_affectee_attr.id: 100}, srqs={eve_skill.id: 1})
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_fit = api_sol.create_fit()
    api_fit.set_char(type_id=eve_char.id)
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_drone = api_fit.add_drone(type_id=eve_drone.id)
    assert api_drone.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_drone.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    api_proj_effect.change_proj_effect(state=False)
    assert api_drone.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_proj_effect.change_proj_effect(rm_projs=[api_ship.id])
    assert api_drone.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_fit.remove()
    api_drone.update(status_code=404)
    api_proj_effect.remove()


def test_add_item_pe_proj_state_remove_pe_item(client, consts):
    eve_skill = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.char,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_drone = client.mk_eve_item(attrs={eve_affectee_attr.id: 100}, srqs={eve_skill.id: 1})
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_char(type_id=eve_char.id)
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_drone = api_fit.add_drone(type_id=eve_drone.id)
    assert api_drone.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id, state=False)
    assert api_drone.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_drone.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_proj_effect.change_proj_effect(state=True)
    assert api_drone.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    api_proj_effect.remove()
    assert api_drone.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_drone.remove()
    api_drone.update(status_code=404)
    api_fit.remove()


def test_add_item_pe_proj_remove_item(client, consts):
    eve_skill = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.char,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_drone = client.mk_eve_item(attrs={eve_affectee_attr.id: 100}, srqs={eve_skill.id: 1})
    eve_char = client.mk_eve_item()
    eve_struct = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_char(type_id=eve_char.id)
    api_struct = api_fit.set_ship(type_id=eve_struct.id)
    api_drone = api_fit.add_drone(type_id=eve_drone.id)
    assert api_drone.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    assert api_drone.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_proj_effect.change_proj_effect(add_projs=[api_struct.id])
    assert api_drone.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    api_drone.remove()
    api_drone.update(status_code=404)
    api_proj_effect.remove()
    api_fit.remove()
