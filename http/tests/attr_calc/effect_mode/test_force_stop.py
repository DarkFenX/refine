from tests import approx


def test_force_stop(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_mod])
    eve_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20, eve_affectee_attr.id: 100}, eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item.id, state=consts.ApiState.offline)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr.id].dogma == approx(120)
    assert api_item.effects[eve_effect.id].running is True
    assert api_item.effects[eve_effect.id].mode == consts.ApiEffMode.full_compliance
    # Action
    api_item.change_mod(effect_modes={eve_effect.id: consts.ApiEffMode.force_stop})
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr.id].dogma == approx(100)
    assert api_item.effects[eve_effect.id].running is False
    assert api_item.effects[eve_effect.id].mode == consts.ApiEffMode.force_stop
    # Action
    api_item.change_mod(effect_modes={eve_effect.id: consts.ApiEffMode.full_compliance})
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr.id].dogma == approx(120)
    assert api_item.effects[eve_effect.id].running is True
    assert api_item.effects[eve_effect.id].mode == consts.ApiEffMode.full_compliance
