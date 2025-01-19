from tests import approx


def test_affected(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_item_id)
    api_fit.add_rig(type_id=eve_affector_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def test_unaffected_other_location(client, consts):
    # Make sure "top" entities described by other locations are not affected
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_affector_item_id)
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_unaffected_child(client, consts):
    # Check that items (in this case rig) are not affected if they belong to location even if its
    # "owner" (in this case structure) is affected
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_affector_item_id)
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_unaffected_other_fit(client, consts):
    # Check that local modifications are not carried over to another fit
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.add_rig(type_id=eve_affector_item_id)
    api_affectee_item = api_fit2.set_ship(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_affector_attr_id = client.mk_eve_attr()
    eve_middle_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_affector_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.struct,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_middle_attr_id)
    eve_affector_effect_id = client.mk_eve_effect(mod_info=[eve_affector_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 2}, eff_ids=[eve_affector_effect_id])
    eve_middle_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_middle_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_middle_effect_id = client.mk_eve_effect(mod_info=[eve_middle_mod])
    eve_middle_item_id = client.mk_eve_item(attrs={eve_middle_attr_id: 20}, eff_ids=[eve_middle_effect_id])
    eve_affectee_item_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_item_id)
    api_fit.add_rig(type_id=eve_middle_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_affector_item = api_fit.add_rig(type_id=eve_affector_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(140)
    api_affector_item.remove()
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def test_replace_root(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item1_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 100})
    eve_affectee_item2_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affectee_item1 = api_fit.set_ship(type_id=eve_affectee_item1_id)
    api_fit.add_rig(type_id=eve_affector_item_id)
    assert api_affectee_item1.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_affectee_item2 = api_fit.set_ship(type_id=eve_affectee_item2_id)
    assert api_affectee_item2.update().attrs[eve_affectee_attr_id].dogma == approx(60)


def test_src_switch_to_ship(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_affector_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_affectee_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(datas=[eve_d1, eve_d2], mod_info=[eve_mod])
    eve_affector_id = client.mk_eve_item(
        datas=[eve_d1, eve_d2],
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id])
    eve_root_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_struct(datas=[eve_d1], id_=eve_root_id, attrs={eve_affectee_attr_id: 100})
    client.mk_eve_ship(datas=[eve_d2], id_=eve_root_id, attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_affectee = api_fit.set_ship(type_id=eve_root_id)
    api_fit.add_rig(type_id=eve_affector_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(120)
