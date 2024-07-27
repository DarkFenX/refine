from tests import approx


def test_affected_multiple(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 100})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.set_ship(type_id=eve_ship.id)
    api_fit2.set_ship(type_id=eve_ship.id)
    api_affectee_item1 = api_fit1.add_rig(type_id=eve_affectee_item.id)
    api_affectee_item2 = api_fit2.add_rig(type_id=eve_affectee_item.id)
    api_sol.add_sw_effect(type_id=eve_affector_item.id)
    assert api_affectee_item1.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    assert api_affectee_item2.update().attrs[eve_affectee_attr.id].dogma == approx(120)


def test_unaffected_other_domain(client, consts):
    # Check that entities from other domains are not affected
    eve_grp = client.mk_eve_item_group()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 100})
    eve_struct = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct.id)
    api_sol.add_sw_effect(type_id=eve_affector_item.id)
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_other_group(client, consts):
    # Check that entities belonging to other item groups are not affected
    eve_grp1 = client.mk_eve_item_group()
    eve_grp2 = client.mk_eve_item_group()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp1.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(grp_id=eve_grp2.id, attrs={eve_affectee_attr.id: 100})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_sol.add_sw_effect(type_id=eve_affector_item.id)
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_replace_root(client, consts):
    # Modifiers which target items on ship location shouldn't apply when ship isn't set
    eve_grp = client.mk_eve_item_group()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_affectee_item = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 100})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_item.id)
    api_sol.add_sw_effect(type_id=eve_affector_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    api_ship.remove()
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_fit.set_ship(type_id=eve_ship.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(120)


def test_src_switch_to_struct(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_grp_id = eve_d1.mk_item_group().id
    eve_d2.mk_item_group(id_=eve_grp_id)
    eve_affector_attr_id = eve_d1.mk_attr().id
    eve_d2.mk_attr(id_=eve_affector_attr_id)
    eve_affectee_attr_id = eve_d1.mk_attr().id
    eve_d2.mk_attr(id_=eve_affectee_attr_id)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = eve_d1.mk_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod]).id
    eve_d2.mk_effect(id_=eve_effect_id, cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_sw_effect_id = eve_d1.mk_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id]).id
    eve_d2.mk_item(id_=eve_sw_effect_id, attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_rig_id = eve_d1.mk_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 100}).id
    eve_d2.mk_item(id_=eve_rig_id, grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 100})
    eve_root_id = eve_d1.mk_ship().id
    eve_d2.mk_struct(id_=eve_root_id)
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_root_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(120)
