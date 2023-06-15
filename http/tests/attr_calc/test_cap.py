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
    eve_item = client.mk_eve_item(attrs={eve_capped_attr.id: 3, eve_src_attr.id: 6}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item.id)
    value = api_item.update().attr_vals[eve_capped_attr.id].dogma
    # Should be 18 with cap, 5 with cap
    assert value == approx(5)
