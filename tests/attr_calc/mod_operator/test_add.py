
def test_penalized(client, consts):
    src_attr = client.mk_attr()
    tgt_attr = client.mk_attr(stackable=0)
    modifier = client.mk_mod(
        func='ItemModifier', dom='itemID', op=consts.ModOp.mod_add,
        src_attr_id=src_attr.id, tgt_attr_id=tgt_attr.id)
    effect = client.mk_effect(mod_info=[modifier])
    item = client.mk_item(attrs={src_attr.id: 5.2, tgt_attr.id: 3.1}, eff_ids=[effect.id])
    client.create_sources()
    ss = client.create_ss()
    fit = ss.create_fit()
    module = fit.add_high_mod(module_id=item.id, state='offline')
    assert module.attr_vals[tgt_attr.id].dogma == 8.3


def test_non_penalized(client, consts):
    src_attr = client.mk_attr()
    tgt_attr = client.mk_attr(stackable=1)
    mod = client.mk_mod(
        func='ItemModifier', dom='itemID', op=consts.ModOp.mod_add,
        src_attr_id=src_attr.id, tgt_attr_id=tgt_attr.id)
    effect = client.mk_effect(mod_info=[mod])
    item = client.mk_item(attrs={src_attr.id: 5.2, tgt_attr.id: 3.1}, eff_ids=[effect.id])
    client.create_sources()
    ss = client.create_ss()
    fit = ss.create_fit()
    module = fit.add_high_mod(module_id=item.id, state='offline')
    assert module.attr_vals[tgt_attr.id].dogma == 8.3
