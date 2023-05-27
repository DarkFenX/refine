

def test_penalized(client, consts):
    src_attr = client.mk_attr()
    tgt_attr = client.mk_attr(stackable=0)
    modifier = client.mk_mod(
        func='ItemModifier', dom='itemID', op=consts.ModOp.mod_add,
        src_attr_id=src_attr.id, tgt_attr_id=tgt_attr.id)
    effect = client.mk_effect(mod_info=[modifier])
    item = client.mk_item(attrs={src_attr.id: 5.2, tgt_attr.id: 3.1}, eff_ids=[effect.id])
    client.create_sources()
    ss_id = client.create_ss()
    fit_id = client.create_fit(ss_id)
    module_id = client.add_high_mod(ss_id, fit_id, item.id, 'offline')
    module = client.get_item(ss_id, module_id)
    assert module['attr_vals'][str(tgt_attr.id)][2] == 8.3


def test_non_penalized(client, consts):
    src_attr = client.mk_attr()
    tgt_attr = client.mk_attr(stackable=1)
    mod = client.mk_mod(
        func='ItemModifier', dom='itemID', op=consts.ModOp.mod_add,
        src_attr_id=src_attr.id, tgt_attr_id=tgt_attr.id)
    effect = client.mk_effect(mod_info=[mod])
    item = client.mk_item(cat_id=consts.ItemCat.ship, attrs={src_attr.id: 5.2, tgt_attr.id: 3.1}, eff_ids=[effect.id])
    client.create_sources()
    ss_id = client.create_ss()
    fit_id = client.create_fit(ss_id)
    module_id = client.add_high_mod(ss_id, fit_id, item.id, 'offline')
    module = client.get_item(ss_id, module_id)
    assert module['attr_vals'][str(tgt_attr.id)][2] == 8.3
