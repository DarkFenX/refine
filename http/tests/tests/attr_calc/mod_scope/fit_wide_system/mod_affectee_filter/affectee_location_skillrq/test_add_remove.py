from fw import approx


def test_add_fw_item_remove_fw_item(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_fw_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_fw_effect.remove()
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_module.remove()
    api_module.update(status_code=404)
    api_fit.remove()


def test_add_item_fw_remove_state_item_fw(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_fw_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100}, srqs={eve_skill_id: 1})
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_fw_effect.change_fw_effect(state=False)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_module.remove()
    api_module.update(status_code=404)
    api_fw_effect.remove()
    api_fit.remove()


def test_add_fw_item_state_remove_fit(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_fw_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_implant_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100}, srqs={eve_skill_id: 1})
    eve_char_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id, state=False)
    api_fit.set_character(type_id=eve_char_id)
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    assert api_implant.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_fw_effect.change_fw_effect(state=True)
    assert api_implant.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_fit.remove()
    api_implant.update(status_code=404)
