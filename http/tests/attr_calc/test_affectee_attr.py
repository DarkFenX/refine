from pytest import approx


def test_multiple(client, consts):
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr1 = client.mk_eve_attr()
    eve_tgt_attr2 = client.mk_eve_attr()
    eve_tgt_attr3 = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr1.id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr2.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod1, eve_mod2])
    eve_item = client.mk_eve_item(
        attrs={eve_src_attr.id: 20, eve_tgt_attr1.id: 50, eve_tgt_attr2.id: 80, eve_tgt_attr3.id: 100},
        eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_rig(type_id=eve_item.id)
    api_item.update()
    # First attribute should be modified by modifier 1
    assert api_item.attrs[eve_tgt_attr1.id].dogma == approx(60)
    # Second should be modified by modifier 2
    assert api_item.attrs[eve_tgt_attr2.id].dogma == approx(96)
    # Third should stay unmodified
    assert api_item.attrs[eve_tgt_attr3.id].dogma == approx(100)
