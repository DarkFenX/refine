from pytest import approx


def get_value_after_switch(client, consts, state_from, state_to):
    eve_tgt_attr = client.mk_eve_attr()
    eve_src_attr_offline = client.mk_eve_attr()
    eve_src_attr_online = client.mk_eve_attr()
    eve_src_attr_active = client.mk_eve_attr()
    eve_src_attr_overload = client.mk_eve_attr()
    eve_mod_offline = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_src_attr_offline.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect_offline = client.mk_eve_effect(cat_id=consts.EffCat.passive, mod_info=[eve_mod_offline])
    eve_mod_online = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_src_attr_online.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect_online = client.mk_eve_effect(cat_id=consts.EffCat.online, mod_info=[eve_mod_online])
    eve_mod_active = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_src_attr_active.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect_active = client.mk_eve_effect(cat_id=consts.EffCat.active, mod_info=[eve_mod_active])
    eve_mod_overload = client.mk_eve_mod(
        func=consts.ModFunc.item,
        dom=consts.ModDom.item,
        op=consts.ModOp.post_mul,
        src_attr_id=eve_src_attr_overload.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect_overload = client.mk_eve_effect(cat_id=consts.EffCat.overload, mod_info=[eve_mod_overload])
    eve_item = client.mk_eve_item(
        attrs={
            eve_tgt_attr.id: 100, eve_src_attr_offline.id: 1.1, eve_src_attr_online.id: 1.3,
            eve_src_attr_active.id: 1.5, eve_src_attr_overload.id: 1.7},
        eff_ids=[eve_effect_offline.id, eve_effect_online.id, eve_effect_active.id, eve_effect_overload.id])
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item.id, state=state_from)
    api_item.change_mod(state=state_to)
    return api_item.update().attr_vals[eve_tgt_attr.id].dogma


def test_switch_up_single(client, consts):
    value = get_value_after_switch(client, consts, state_from=consts.State.offline, state_to=consts.State.online)
    assert value == approx(143)


def test_switch_up_multiple(client, consts):
    value = get_value_after_switch(client, consts, state_from=consts.State.online, state_to=consts.State.overload)
    assert value == approx(364.65)


def test_switch_down_single(client, consts):
    value = get_value_after_switch(client, consts, state_from=consts.State.overload, state_to=consts.State.active)
    assert value == approx(214.5)


def test_switch_down_multiple(client, consts):
    value = get_value_after_switch(client, consts, state_from=consts.State.active, state_to=consts.State.offline)
    assert value == approx(110)
