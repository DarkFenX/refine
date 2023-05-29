from pytest import approx


def test_penalized(client, consts):
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr(stackable=0)
    eve_modifier = client.mk_eve_mod(
        func='ItemModifier', dom='itemID', op=consts.ModOp.mod_sub,
        src_attr_id=eve_src_attr.id, tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_modifier])
    eve_item = client.mk_eve_item(attrs={eve_src_attr.id: 2.2, eve_tgt_attr.id: 10.3}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_high_mod(module_id=eve_item.id)
    assert approx(api_item.attr_vals[eve_tgt_attr.id].dogma) == 8.1


def test_non_penalized(client, consts):
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr(stackable=1)
    eve_modifier = client.mk_eve_mod(
        func='ItemModifier', dom='itemID', op=consts.ModOp.mod_sub,
        src_attr_id=eve_src_attr.id, tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_modifier])
    eve_item = client.mk_eve_item(attrs={eve_src_attr.id: 2.2, eve_tgt_attr.id: 10.3}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_high_mod(module_id=eve_item.id)
    assert approx(api_item.attr_vals[eve_tgt_attr.id].dogma) == 8.1
