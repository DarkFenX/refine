from pytest import approx


def test_add_fw_item_remove_fw_item(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_fw_effect = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect.id)
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    api_fw_effect.remove()
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_ship.remove()
    api_ship.update(status_code=404)
    api_fit.remove()


def test_add_item_fw_remove_state_item_fw(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_fw_effect = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_struct = client.mk_eve_struct(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_ship(type_id=eve_struct.id)
    assert api_struct.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect.id)
    assert api_struct.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    api_fw_effect.change_fw_effect(state=False)
    assert api_struct.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_struct.remove()
    api_struct.update(status_code=404)
    api_fw_effect.remove()
    api_fit.remove()


def test_add_fw_item_state_remove_fit(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_fw_effect = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_char = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect.id, state=False)
    api_char = api_fit.set_char(type_id=eve_char.id)
    assert api_char.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_fw_effect.change_fw_effect(state=True)
    assert api_char.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    api_fit.remove()
    api_char.update(status_code=404)
