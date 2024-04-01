from pytest import approx


def get_value_for_state(client, consts, state):
    eve_tgt_attr = client.mk_eve_attr()
    eve_src_attr_offline = client.mk_eve_attr()
    eve_src_attr_online = client.mk_eve_attr()
    eve_src_attr_active = client.mk_eve_attr()
    eve_src_attr_overload = client.mk_eve_attr()
    eve_effect_online = client.mk_eve_online_effect()
    eve_mod_cat_offline = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_src_attr_offline.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect_cat_offline = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_mod_cat_offline])
    eve_mod_cat_online = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_src_attr_online.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect_cat_online = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod_cat_online])
    eve_mod_cat_active = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_src_attr_active.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect_cat_active = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod_cat_active])
    eve_mod_cat_overload = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        src_attr_id=eve_src_attr_overload.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_effect_cat_overload = client.mk_eve_effect(cat_id=consts.EveEffCat.overload, mod_info=[eve_mod_cat_overload])
    eve_item = client.mk_eve_item(
        attrs={
            eve_tgt_attr.id: 100, eve_src_attr_offline.id: 1.1, eve_src_attr_online.id: 1.3,
            eve_src_attr_active.id: 1.5, eve_src_attr_overload.id: 1.7},
        eff_ids=[
            eve_effect_online.id, eve_effect_cat_offline.id, eve_effect_cat_online.id,
            eve_effect_cat_active.id, eve_effect_cat_overload.id],
        defeff_id=eve_effect_cat_active.id)
    client.create_sources()
    api_ss = client.create_ss()
    api_fit = api_ss.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item.id, state=state)
    return api_item.update().attrs[eve_tgt_attr.id].dogma


def test_fit_offline(client, consts):
    assert get_value_for_state(client, consts, state=consts.ApiState.offline) == approx(110)


def test_fit_online(client, consts):
    assert get_value_for_state(client, consts, state=consts.ApiState.online) == approx(143)


def test_fit_active(client, consts):
    assert get_value_for_state(client, consts, state=consts.ApiState.active) == approx(214.5)


def test_fit_overload(client, consts):
    assert get_value_for_state(client, consts, state=consts.ApiState.overload) == approx(364.65)
