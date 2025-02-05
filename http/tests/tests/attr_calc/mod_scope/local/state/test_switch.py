from tests import approx


def get_value_after_switch(*, client, consts, state_from, state_to):
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_affector_attr_offline_id = client.mk_eve_attr()
    eve_affector_attr_online_id = client.mk_eve_attr()
    eve_affector_attr_active_id = client.mk_eve_attr()
    eve_affector_attr_overload_id = client.mk_eve_attr()
    eve_effect_online_id = client.mk_eve_online_effect()
    eve_mod_cat_offline = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_offline_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_cat_offline_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_mod_cat_offline])
    eve_mod_cat_online = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_online_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_cat_online_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod_cat_online])
    eve_mod_cat_active = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_active_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_cat_active_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod_cat_active])
    eve_mod_cat_overload = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_overload_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_cat_overload_id = client.mk_eve_effect(cat_id=consts.EveEffCat.overload, mod_info=[eve_mod_cat_overload])
    eve_item_id = client.mk_eve_item(
        attrs={
            eve_affectee_attr_id: 100, eve_affector_attr_offline_id: 1.1, eve_affector_attr_online_id: 1.3,
            eve_affector_attr_active_id: 1.5, eve_affector_attr_overload_id: 1.7},
        eff_ids=[
            eve_effect_online_id, eve_effect_cat_offline_id, eve_effect_cat_online_id,
            eve_effect_cat_active_id, eve_effect_cat_overload_id],
        defeff_id=eve_effect_cat_active_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item_id, state=state_from)
    api_item.change_mod(state=state_to)
    return api_item.update().attrs[eve_affectee_attr_id].dogma


def test_switch_up_single(client, consts):
    value = get_value_after_switch(
        client=client,
        consts=consts,
        state_from=consts.ApiModuleState.offline,
        state_to=consts.ApiModuleState.online)
    assert value == approx(143)


def test_switch_up_multiple(client, consts):
    value = get_value_after_switch(
        client=client,
        consts=consts,
        state_from=consts.ApiModuleState.online,
        state_to=consts.ApiModuleState.overload)
    assert value == approx(364.65)


def test_switch_down_single(client, consts):
    value = get_value_after_switch(
        client=client,
        consts=consts,
        state_from=consts.ApiModuleState.overload,
        state_to=consts.ApiModuleState.active)
    assert value == approx(214.5)


def test_switch_down_multiple(client, consts):
    value = get_value_after_switch(
        client=client,
        consts=consts,
        state_from=consts.ApiModuleState.active,
        state_to=consts.ApiModuleState.offline)
    assert value == approx(110)
