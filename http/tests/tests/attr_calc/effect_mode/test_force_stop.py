from fw import Effect, approx


def test_force_stop(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20, eve_affectee_attr_id: 100},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.offline)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].modified == approx(120)
    assert api_item.effects[api_effect_id].running is True
    assert api_item.effects[api_effect_id].mode == consts.ApiEffMode.full_compliance
    # Action
    api_item.change_module(effect_modes={api_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].modified == approx(100)
    assert api_item.effects[api_effect_id].running is False
    assert api_item.effects[api_effect_id].mode == consts.ApiEffMode.force_stop
    # Action
    api_item.change_module(effect_modes={api_effect_id: consts.ApiEffMode.full_compliance})
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr_id].modified == approx(120)
    assert api_item.effects[api_effect_id].running is True
    assert api_item.effects[api_effect_id].mode == consts.ApiEffMode.full_compliance
