from pytest import approx


# Currently there are no effects used by EVE which affect multiple items
# filtered "other" domain and skill requirement, so we don't support it either
def test_unaffected(client, consts):
    eve_skill = client.mk_eve_item()
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.loc_srq,
        dom=consts.ModDom.other,
        srq=eve_skill.id,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 100}, srqs={eve_skill.id: 1})
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
