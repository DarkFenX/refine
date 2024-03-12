from pytest import approx


def setup_hig_test(client, consts, high_is_good):
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr(high_is_good=high_is_good)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.ship,
        op=consts.ModOp.post_assign,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_src1 = client.mk_eve_item(attrs={eve_src_attr.id: 10}, eff_ids=[eve_effect.id])
    eve_item_src2 = client.mk_eve_item(attrs={eve_src_attr.id: -20}, eff_ids=[eve_effect.id])
    eve_item_src3 = client.mk_eve_item(attrs={eve_src_attr.id: 53.02}, eff_ids=[eve_effect.id])
    eve_item_tgt = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item_src1 = api_fit.add_rig(type_id=eve_item_src1.id)
    api_item_src2 = api_fit.add_rig(type_id=eve_item_src2.id)
    api_item_src3 = api_fit.add_rig(type_id=eve_item_src3.id)
    api_item_tgt = api_fit.set_ship(type_id=eve_item_tgt.id)
    api_item_tgt.update()
    return (
        api_item_tgt.attrs[eve_tgt_attr.id].dogma,
        api_item_tgt.mods[eve_tgt_attr.id],
        api_item_src1,
        api_item_src2,
        api_item_src3)


def test_high_is_good(client, consts):
    attr_val, attr_mods, _, _, api_item_src3 = setup_hig_test(client, consts, high_is_good=True)
    assert attr_val == approx(53.02)
    assert attr_mods.one().op == consts.InfoOp.post_assign
    assert attr_mods.one().src.one().item_id == api_item_src3.id


def test_high_is_bad(client, consts):
    attr_val, attr_mods, _, api_item_src2, _ = setup_hig_test(client, consts, high_is_good=False)
    assert attr_val == approx(-20)
    assert attr_mods.one().op == consts.InfoOp.post_assign
    assert attr_mods.one().src.one().item_id == api_item_src2.id
