from pytest import approx


def test_affected_top(client, consts):
    # Check that item which is top of a domain can affect itself
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.set_char(type_id=eve_item.id)
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(120)


def test_affected_child(client, consts):
    # Check that item which belongs to some domain can affect itself
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_rig(type_id=eve_item.id)
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(120)


def test_unaffected_top(client, consts):
    # Check that item which is top of domain does not affect top items of other
    # domains and children of its domain
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affecting_item = client.mk_eve_item(attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    eve_unaffected_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.set_char(type_id=eve_affecting_item.id)
    api_unaffected_top_item = api_fit.set_ship(type_id=eve_unaffected_item.id)
    value = api_unaffected_top_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(100)
    api_unaffected_child_item = api_fit.add_implant(type_id=eve_unaffected_item.id)
    value = api_unaffected_child_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(100)


def test_unaffected_child(client, consts):
    # Check that item which belongs to a domain does not affect top item of its
    # domain and other items within its domain
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affecting_item = client.mk_eve_item(attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    eve_unaffected_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.add_rig(type_id=eve_affecting_item.id)
    api_unaffected_top_item = api_fit.set_ship(type_id=eve_unaffected_item.id)
    value = api_unaffected_top_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(100)
    api_unaffected_child_item = api_fit.add_rig(type_id=eve_unaffected_item.id)
    value = api_unaffected_child_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(100)
