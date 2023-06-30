from pytest import approx


def test_force_stop(client, consts):
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EffCat.passive, mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item.id, state=consts.State.offline)
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(120)
    assert api_item.effects[eve_effect.id] == (True, consts.EffMode.full_compliance)
    api_item.change_mod(effect_modes={eve_effect.id: consts.EffMode.force_stop})
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(100)
    assert api_item.effects[eve_effect.id] == (False, consts.EffMode.force_stop)
    api_item.change_mod(effect_modes={eve_effect.id: consts.EffMode.full_compliance})
    api_item.update()
    assert api_item.attrs[eve_tgt_attr.id].dogma == approx(120)
    assert api_item.effects[eve_effect.id] == (True, consts.EffMode.full_compliance)
