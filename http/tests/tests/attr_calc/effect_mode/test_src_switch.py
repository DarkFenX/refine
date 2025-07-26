from tests import Effect, approx, check_no_field


def test_src_switch(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_affector_attr_id = client.mk_eve_attr(datas=[eve_d2])
    eve_affectee_attr_id = client.mk_eve_attr(datas=[eve_d2])
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(datas=[eve_d2], cat_id=consts.EveEffCat.active, mod_info=[eve_mod])
    eve_item_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d1], id_=eve_item_id)
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_item_id,
        attrs={eve_affector_attr_id: 20, eve_affectee_attr_id: 100},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_normal = api_fit.add_rig(type_id=eve_item_id)
    api_mutable = api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.offline)
    # Verification
    api_normal.update()
    with check_no_field():
        api_normal.attrs  # noqa: B018
    with check_no_field():
        api_normal.effects  # noqa: B018
    api_mutable.update()
    with check_no_field():
        api_mutable.attrs  # noqa: B018
    with check_no_field():
        api_mutable.effects  # noqa: B018
    # Action
    api_normal.change_rig(effect_modes={api_effect_id: consts.ApiEffMode.force_run})
    api_mutable.change_module(effect_modes={api_effect_id: consts.ApiEffMode.force_run})
    # Verification
    api_normal.update()
    with check_no_field():
        api_normal.attrs  # noqa: B018
    with check_no_field():
        api_normal.effects  # noqa: B018
    api_mutable.update()
    with check_no_field():
        api_mutable.attrs  # noqa: B018
    with check_no_field():
        api_mutable.effects  # noqa: B018
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_normal.update()
    assert api_normal.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_normal.effects[api_effect_id].running is True
    assert api_normal.effects[api_effect_id].mode == consts.ApiEffMode.force_run
    api_mutable.update()
    assert api_mutable.attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_mutable.effects[api_effect_id].running is True
    assert api_mutable.effects[api_effect_id].mode == consts.ApiEffMode.force_run
