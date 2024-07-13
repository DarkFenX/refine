from tests import approx


def test_affected_root(client, consts):
    # Check that item which is "owner" of a location can affect itself
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20, eve_affectee_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.set_char(type_id=eve_item.id)
    assert api_item.update().attrs[eve_affectee_attr.id].dogma == approx(120)


def test_affected_child(client, consts):
    # Check that item which belongs to some location can affect itself
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20, eve_affectee_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_rig(type_id=eve_item.id)
    assert api_item.update().attrs[eve_affectee_attr.id].dogma == approx(120)


def test_unaffected_root(client, consts):
    # Check that item which is root of domain does not affect root items of other domains and
    # children of its domain
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affecting_item = client.mk_eve_item(
        attrs={eve_affector_attr.id: 20, eve_affectee_attr.id: 100},
        eff_ids=[eve_effect.id])
    eve_unaffected_item = client.mk_eve_ship(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_char(type_id=eve_affecting_item.id)
    api_unaffected_root = api_fit.set_ship(type_id=eve_unaffected_item.id)
    assert api_unaffected_root.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_unaffected_child = api_fit.add_implant(type_id=eve_unaffected_item.id)
    assert api_unaffected_child.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_child(client, consts):
    # Check that item which belongs to a domain does not affect top item of its domain and other
    # items within its domain
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affecting_item = client.mk_eve_item(
        attrs={eve_affector_attr.id: 20, eve_affectee_attr.id: 100},
        eff_ids=[eve_effect.id])
    eve_unaffected_item = client.mk_eve_ship(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_affecting_item.id)
    api_unaffected_root = api_fit.set_ship(type_id=eve_unaffected_item.id)
    assert api_unaffected_root.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_unaffected_child = api_fit.add_rig(type_id=eve_unaffected_item.id)
    assert api_unaffected_child.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_affector_attr = client.mk_eve_attr()
    eve_middle_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_affector_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_middle_attr.id)
    eve_affector_effect = client.mk_eve_effect(mod_info=[eve_affector_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 2}, eff_ids=[eve_affector_effect.id])
    eve_middle_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_middle_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_middle_effect = client.mk_eve_effect(mod_info=[eve_middle_mod])
    eve_affectee_item = client.mk_eve_item(
        attrs={eve_middle_attr.id: 20, eve_affectee_attr.id: 100},
        eff_ids=[eve_middle_effect.id])
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    api_affector_item = api_fit.add_rig(type_id=eve_affector_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(140)
    api_affector_item.remove()
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(120)
