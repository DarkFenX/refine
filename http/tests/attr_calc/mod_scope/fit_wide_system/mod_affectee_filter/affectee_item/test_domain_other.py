from pytest import approx


def test_unaffected_self(client, consts):
    # Check that other reference from fit-wide effect doesn't affect anything
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20, eve_affectee_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_fw_effect(type_id=eve_item.id)
    assert api_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_root(client, consts):
    # Check that other reference from fit-wide effect doesn't affect anything
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fw_effect(type_id=eve_affector_item.id)
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_child(client, consts):
    # Check that other reference from fit-wide effect doesn't affect anything
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_module = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    eve_affectee_charge = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    eve_ship_item = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fw_effect(type_id=eve_affector_item.id)
    api_fit.set_ship(type_id=eve_ship_item.id)
    api_affectee_item = api_fit.add_mod(type_id=eve_affectee_module.id, charge_type_id=eve_affectee_charge.id)
    api_affectee_item.update()
    assert api_affectee_item.attrs[eve_affectee_attr.id].dogma == approx(100)
    assert api_affectee_item.charge.attrs[eve_affectee_attr.id].dogma == approx(100)
