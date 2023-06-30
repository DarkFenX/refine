from pytest import approx


def test_state_offline(client, consts):
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
    api_item.change_mod(effect_modes={eve_effect.id: consts.EffMode.state_compliance})
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(120)
    api_item.change_mod(state=consts.State.ghost)
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(100)
    api_item.change_mod(state=consts.State.offline)
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(120)


def test_state_online(client, consts):
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EffCat.online, mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item.id, state=consts.State.online)
    api_item.change_mod(effect_modes={eve_effect.id: consts.EffMode.state_compliance})
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(120)
    api_item.change_mod(state=consts.State.offline)
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(100)
    api_item.change_mod(state=consts.State.online)
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(120)


def test_state_active(client, consts):
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
    api_item = api_fit.add_mod(type_id=eve_item.id, state=consts.State.active)
    api_item.change_mod(effect_modes={eve_effect.id: consts.EffMode.state_compliance})
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(120)
    api_item.change_mod(state=consts.State.online)
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(100)
    api_item.change_mod(state=consts.State.active)
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(120)


def test_state_overload(client, consts):
    eve_src_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EffCat.overload, mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_src_attr.id: 20, eve_tgt_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item.id, state=consts.State.overload)
    api_item.change_mod(effect_modes={eve_effect.id: consts.EffMode.state_compliance})
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(120)
    api_item.change_mod(state=consts.State.active)
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(100)
    api_item.change_mod(state=consts.State.overload)
    value = api_item.update().attrs[eve_tgt_attr.id].dogma
    assert value == approx(120)
