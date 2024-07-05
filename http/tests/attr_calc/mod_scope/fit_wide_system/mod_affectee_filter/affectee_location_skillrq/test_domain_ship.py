from pytest import approx


def test_affected(client, consts):
    eve_skill = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_srq,
        dom=consts.EveModDom.ship,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100}, srqs={eve_skill.id: 1})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_item.id)
    api_fit.add_fw_effect(type_id=eve_affector_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(120)


def test_unaffected_other_domain(client, consts):
    # Check that entities from other domains are not affected
    eve_skill = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_srq,
        dom=consts.EveModDom.ship,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100}, srqs={eve_skill.id: 1})
    eve_char_item = client.mk_eve_item()
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_char(type_id=eve_char_item.id)
    api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_fw_effect(type_id=eve_affector_item.id)
    api_affectee_item = api_fit.add_implant(type_id=eve_affectee_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_other_skillreq(client, consts):
    # Check that entities which don't have needed skill requirement are not affected
    eve_skill1 = client.mk_eve_item()
    eve_skill2 = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_srq,
        dom=consts.EveModDom.ship,
        srq=eve_skill1.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100}, srqs={eve_skill2.id: 1})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_fw_effect(type_id=eve_affector_item.id)
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_via_other_root(client, consts):
    # Check that effects which target ship items are not applied via structure
    eve_skill = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_srq,
        dom=consts.EveModDom.ship,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100}, srqs={eve_skill.id: 1})
    eve_struct = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct.id)
    api_fit.add_fw_effect(type_id=eve_affector_item.id)
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_other_fit(client, consts):
    # Check that fit-wide modifications are not carried over to another fit
    eve_skill = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_srq,
        dom=consts.EveModDom.ship,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100}, srqs={eve_skill.id: 1})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.add_fw_effect(type_id=eve_affector_item.id)
    api_fit2.set_ship(type_id=eve_ship.id)
    api_affectee_item = api_fit2.add_rig(type_id=eve_affectee_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_replace_root(client, consts):
    # Modifiers which target items on ship location shouldn't apply when ship isn't set
    eve_skill = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_srq,
        dom=consts.EveModDom.ship,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100}, srqs={eve_skill.id: 1})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_item.id)
    api_fit.add_fw_effect(type_id=eve_affector_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    api_ship.remove()
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_fit.set_ship(type_id=eve_ship.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(120)
