from pytest import approx


def test_root_add_afor_afee_proj_remove_state_proj_fit(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: 3},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_struct = client.mk_eve_struct(attrs={eve_affectee_attr.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_struct = api_affectee_fit.set_ship(type_id=eve_affectee_struct.id)
    assert api_affectee_struct.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affector_module.change_mod(add_tgts=[api_affectee_struct.id])
    assert api_affectee_struct.update().attrs[eve_affectee_attr.id].dogma == approx(1)
    api_affector_module.change_mod(state=consts.ApiState.online)
    assert api_affectee_struct.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affector_module.change_mod(rm_tgts=[api_affectee_struct.id])
    assert api_affectee_struct.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affectee_fit.remove()
    api_affectee_struct.update(status_code=404)
    api_affector_fit.remove()


def test_root_add_afee_afor_proj_state_remove_afor_afee(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: 3},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.online)
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affector_module.change_mod(add_tgts=[api_affectee_ship.id])
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affector_module.change_mod(state=consts.ApiState.active)
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1)
    api_affector_module.remove()
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affectee_ship.remove()
    api_affectee_ship.update(status_code=404)
    api_affectee_fit.remove()
    api_affector_fit.remove()


def test_root_add_afee_afor_proj_remove_afee(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: 3},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affector_module.change_mod(add_tgts=[api_affectee_ship.id])
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1)
    api_affectee_ship.remove()
    api_affectee_ship.update(status_code=404)
    api_affectee_fit.remove()
    api_affector_fit.remove()


def test_child_add_afor_afee_proj_remove_state_proj_fit(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: 3},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_drone = client.mk_eve_ship(attrs={eve_affectee_attr.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_drone = api_affectee_fit.add_drone(type_id=eve_affectee_drone.id)
    assert api_affectee_drone.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affector_module.change_mod(add_tgts=[api_affectee_drone.id])
    assert api_affectee_drone.update().attrs[eve_affectee_attr.id].dogma == approx(1)
    api_affector_module.change_mod(state=consts.ApiState.online)
    assert api_affectee_drone.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affector_module.change_mod(rm_tgts=[api_affectee_drone.id])
    assert api_affectee_drone.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affectee_fit.remove()
    api_affectee_drone.update(status_code=404)
    api_affector_fit.remove()


def test_child_add_afee_afor_proj_state_remove_afor_afee(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: 3},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_fighter = client.mk_eve_ship(attrs={eve_affectee_attr.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_fighter = api_affectee_fit.add_fighter(type_id=eve_affectee_fighter.id)
    assert api_affectee_fighter.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.online)
    assert api_affectee_fighter.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affector_module.change_mod(add_tgts=[api_affectee_fighter.id])
    assert api_affectee_fighter.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affector_module.change_mod(state=consts.ApiState.active)
    assert api_affectee_fighter.update().attrs[eve_affectee_attr.id].dogma == approx(1)
    api_affector_module.remove()
    assert api_affectee_fighter.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affectee_fighter.remove()
    api_affectee_fighter.update(status_code=404)
    api_affectee_fit.remove()
    api_affector_fit.remove()


def test_child_add_afee_afor_proj_remove_afee(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: 3},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_drone = client.mk_eve_ship(attrs={eve_affectee_attr.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_drone = api_affectee_fit.add_drone(type_id=eve_affectee_drone.id)
    assert api_affectee_drone.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    assert api_affectee_drone.update().attrs[eve_affectee_attr.id].dogma == approx(-2)
    api_affector_module.change_mod(add_tgts=[api_affectee_drone.id])
    assert api_affectee_drone.update().attrs[eve_affectee_attr.id].dogma == approx(1)
    api_affectee_drone.remove()
    api_affectee_drone.update(status_code=404)
    api_affectee_fit.remove()
    api_affector_fit.remove()
