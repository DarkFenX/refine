from pytest import approx


def test_force_run(client, consts):
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EffCat.active, mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item.id, state=consts.State.offline)
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(100)
    api_item.change_mod(effect_modes={eve_effect.id: consts.EffMode.force_run})
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(120)
    api_item.change_mod(effect_modes={eve_effect.id: consts.EffMode.full_compliance})
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(100)
