from fw import approx


def test_add_fw_item_remove_fw_item(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_fw_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_fw_effect.remove()
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_ship.remove()
    api_ship.update(status_code=404)
    api_fit.remove()


def test_add_item_fw_remove_state_item_fw(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_fw_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_struct_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_ship(type_id=eve_struct_id)
    assert api_struct.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    assert api_struct.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_fw_effect.change_fw_effect(state=False)
    assert api_struct.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_struct.remove()
    api_struct.update(status_code=404)
    api_fw_effect.remove()
    api_fit.remove()


def test_add_fw_item_state_remove_fit(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_fw_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_char_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id, state=False)
    api_char = api_fit.set_character(type_id=eve_char_id)
    assert api_char.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_fw_effect.change_fw_effect(state=True)
    assert api_char.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_fit.remove()
    api_char.update(status_code=404)
