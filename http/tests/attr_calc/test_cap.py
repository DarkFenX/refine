from pytest import approx


def test_default(client, consts):
    # Check that cap is applied properly when item doesn't have base value
    # of capping attribute
    eve_capping_attr = client.mk_eve_attr(def_val=5)
    eve_capped_attr = client.mk_eve_attr(max_attr_id=eve_capping_attr.id)
    eve_src_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_capped_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item = client.mk_eve_item(
        attrs={eve_capped_attr.id: 3, eve_src_attr.id: 6},
        eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item.id)
    value = api_item.update().attr_vals[eve_capped_attr.id].dogma
    # Should be 3 * 6 = 18 without cap, but 5 with cap
    assert value == approx(5)


def test_unmodified(client, consts):
    # Check that cap is applied properly when item defines its value
    eve_capping_attr = client.mk_eve_attr(def_val=5)
    eve_capped_attr = client.mk_eve_attr(max_attr_id=eve_capping_attr.id)
    eve_src_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_capped_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item = client.mk_eve_item(
        attrs={eve_capping_attr.id: 2, eve_capped_attr.id: 3, eve_src_attr.id: 6},
        eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item.id)
    value = api_item.update().attr_vals[eve_capped_attr.id].dogma
    # Should be 3 * 6 = 18 without cap, but 2 with cap
    assert value == approx(2)


def test_modified(client, consts):
    # Check that cap is applied properly when item defines its value,
    # and it's modified further
    eve_capping_attr = client.mk_eve_attr()
    eve_capped_attr = client.mk_eve_attr(max_attr_id=eve_capping_attr.id)
    eve_src_attr = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_capped_attr.id)
    eve_mod2 = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_capping_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod1, eve_mod2])
    eve_item = client.mk_eve_item(
        attrs={eve_capping_attr.id: 0.1, eve_capped_attr.id: 3, eve_src_attr.id: 6},
        eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item.id)
    value = api_item.update().attr_vals[eve_capped_attr.id].dogma
    # Should be 3 * 6 = 18 without cap, but 0.1 * 6 = 0.6 with cap
    assert value == approx(0.6)


def test_update(client, consts):
    # Make sure that when value of capping attribute changes, values which
    # depend on it are updated
    eve_capping_attr = client.mk_eve_attr()
    eve_capped_attr = client.mk_eve_attr(max_attr_id=eve_capping_attr.id)
    eve_src_attr = client.mk_eve_attr()
    eve_capped_mod = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_capped_attr.id)
    eve_capped_effect = client.mk_eve_effect(mod_info=[eve_capped_mod])
    eve_capped_item = client.mk_eve_item(
        attrs={eve_capping_attr.id: 2, eve_capped_attr.id: 3, eve_src_attr.id: 6},
        eff_ids=[eve_capped_effect.id])
    eve_capping_mod = client.mk_eve_mod(
        func=consts.ModFunc.loc,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_capping_attr.id)
    eve_capping_effect = client.mk_eve_effect(mod_info=[eve_capping_mod])
    eve_capping_item = client.mk_eve_item(attrs={eve_src_attr.id: 3.5}, eff_ids=[eve_capping_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_capped_item = api_fit.add_rig(type_id=eve_capped_item.id)
    # Request capped attribute value before adding capping item, to make sure
    # capping attribute value is calculated
    value = api_capped_item.update().attr_vals[eve_capped_attr.id].dogma
    assert value == approx(2)
    api_fit.add_implant(type_id=eve_capping_item.id)
    # Here, capping attribute should be multiplied by 3.5 (2 * 3.5 = 7), which
    # is still below uncapped value of capped attribute (18)
    value = api_capped_item.update().attr_vals[eve_capped_attr.id].dogma
    assert value == approx(7)
