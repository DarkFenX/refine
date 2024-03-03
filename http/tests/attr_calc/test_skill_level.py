from pytest import approx


def test_switch(client, consts):
    eve_tgt_attr = client.mk_eve_attr()
    eve_src_attr = client.mk_eve_attr(id_=consts.Attr.skill_level)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_skill(type_id=eve_item.id, level=5)
    assert api_item.update().attrs[eve_tgt_attr.id].dogma == approx(500)
    api_item.change_skill(level=3)
    assert api_item.update().attrs[eve_tgt_attr.id].dogma == approx(300)
