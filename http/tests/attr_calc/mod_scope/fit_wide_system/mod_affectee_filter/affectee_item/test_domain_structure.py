from pytest import approx


def test_affected_multiple(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_struct(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_item.id)
    api_affector_item = api_fit.add_fw_effect(type_id=eve_affector_item.id, state=False)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_affector_item.change_fw_effect(state=True)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    api_affector_item.change_fw_effect(state=False)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_other_domain(client, consts):
    # Make sure "top" entities described by other domains are not affected
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_ship(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fw_effect(type_id=eve_affector_item.id)
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_child(client, consts):
    # Check that items (in this case rig) are not affected if they belong to location even if
    # its "owner" (in this case structure) is affected
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    eve_struct_item = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fw_effect(type_id=eve_affector_item.id)
    api_fit.set_ship(type_id=eve_struct_item.id)
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_other_fit(client, consts):
    # Check that fit-wide modifications are not carried over to another fit
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_struct(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.add_fw_effect(type_id=eve_affector_item.id)
    api_affectee_item = api_fit2.set_ship(type_id=eve_affectee_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_replace_root(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item1 = client.mk_eve_struct(attrs={eve_affectee_attr.id: 100})
    eve_affectee_item2 = client.mk_eve_struct(attrs={eve_affectee_attr.id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affectee_item1 = api_fit.set_ship(type_id=eve_affectee_item1.id)
    api_fit.add_fw_effect(type_id=eve_affector_item.id)
    assert api_affectee_item1.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    api_affectee_item2 = api_fit.set_ship(type_id=eve_affectee_item2.id)
    assert api_affectee_item2.update().attrs[eve_affectee_attr.id].dogma == approx(60)
