from pytest import approx


# Currently there are no effects used by EVE which affect multiple items with group filter
# in "other" domain, so we don't support it either
def test_unaffected(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.loc_grp,
        dom=consts.ModDom.item,
        grp=eve_grp.id,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_fit.add_rig(type_id=eve_src_item.id)
    api_tgt_item1 = api_fit.add_rig(type_id=eve_tgt_item.id)
    api_tgt_item2 = api_fit.add_implant(type_id=eve_tgt_item.id)
    value = api_tgt_item1.update().attr_vals[eve_tgt_attr.id].dogma
    assert value == approx(100)
    value = api_tgt_item2.update().attr_vals[eve_tgt_attr.id].dogma
    assert value == approx(100)
