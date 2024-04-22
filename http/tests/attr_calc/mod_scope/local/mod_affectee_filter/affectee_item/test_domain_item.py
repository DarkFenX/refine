from pytest import approx


def test_affected_parent(client, consts):
    # Check that item which is "owner" of a location can affect itself
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.set_char(type_id=eve_item.id)
    assert api_item.update().attrs[eve_tgt_attr.id].dogma == approx(120)


def test_affected_child(client, consts):
    # Check that item which belongs to some location can affect itself
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_rig(type_id=eve_item.id)
    assert api_item.update().attrs[eve_tgt_attr.id].dogma == approx(120)


def test_affected_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_src_attr = client.mk_eve_attr()
    eve_mid_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_mid_attr.id)
    eve_src_effect = client.mk_eve_effect(mod_info=[eve_src_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 2}, eff_ids=[eve_src_effect.id])
    eve_mid_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_mid_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_mid_effect = client.mk_eve_effect(mod_info=[eve_mid_mod])
    eve_tgt_item = client.mk_eve_item(attrs={eve_mid_attr.id: 20, eve_tgt_attr.id: 100}, eff_ids=[eve_mid_effect.id])
    eve_ship_item = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_item.id)
    api_tgt_item = api_fit.add_rig(type_id=eve_tgt_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(120)
    api_src_item = api_fit.add_rig(type_id=eve_src_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(140)
    api_src_item.remove()
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(120)


def test_unaffected_parent(client, consts):
    # Check that item which is parent of domain does not affect parent items of other domains and
    # children of its domain
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affecting_item = client.mk_eve_item(attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    eve_unaffected_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_char(type_id=eve_affecting_item.id)
    api_unaffected_parent = api_fit.set_ship(type_id=eve_unaffected_item.id)
    assert api_unaffected_parent.update().attrs[eve_tgt_attr.id].dogma == approx(100)
    api_unaffected_child = api_fit.add_implant(type_id=eve_unaffected_item.id)
    assert api_unaffected_child.update().attrs[eve_tgt_attr.id].dogma == approx(100)


def test_unaffected_child(client, consts):
    # Check that item which belongs to a domain does not affect top item of its domain and other
    # items within its domain
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affecting_item = client.mk_eve_item(attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    eve_unaffected_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_affecting_item.id)
    api_unaffected_parent = api_fit.set_ship(type_id=eve_unaffected_item.id)
    assert api_unaffected_parent.update().attrs[eve_tgt_attr.id].dogma == approx(100)
    api_unaffected_child = api_fit.add_rig(type_id=eve_unaffected_item.id)
    assert api_unaffected_child.update().attrs[eve_tgt_attr.id].dogma == approx(100)
